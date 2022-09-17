/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::obj::GodotClass;
use crate::out;
use godot_ffi as sys;

use std::any::type_name;
use std::cell;

/// Manages storage and lifecycle of user's extension class instances.
pub struct InstanceStorage<T: GodotClass> {
    user_instance: cell::RefCell<T>,

    // Declared after `user_instance`, is dropped last
    pub lifecycle: Lifecycle,
    godot_ref_count: i32,
}

#[derive(Copy, Clone, Debug)]
pub enum Lifecycle {
    Alive,
    Destroying,
    Dead, // reading this would typically already be too late, only best-effort in case of UB
}

/// For all Godot extension classes
impl<T: GodotClass> InstanceStorage<T> {
    pub fn construct(user_instance: T) -> Self {
        out!("    Storage::construct             <{}>", type_name::<T>());

        Self {
            user_inst