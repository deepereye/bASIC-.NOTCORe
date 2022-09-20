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
            user_instance: cell::RefCell::new(user_instance),
            lifecycle: Lifecycle::Alive,
            godot_ref_count: 1,
        }
    }

    pub(crate) fn on_inc_ref(&mut self) {
        self.godot_ref_count += 1;
        out!(
            "    Storage::on_inc_ref (rc={})     <{}>", // -- {:?}",
            self.godot_ref_count,
            type_name::<T>(),
            //self.user_instance
        );
    }

    pub(crate) fn on_dec_ref(&mut self) {
        self.godot_ref_count -= 1;
        out!(
            "  | Storage::on_dec_ref (rc={})     <{}>", // -- {:?}",
            self.godot_ref_count,
            type_name::<T>(),
            //self.user_instance
        );
    }

    /* pub fn destroy(&mut self) {
        assert!(
            self.user_instance.is_some(),
            "Cannot destroy user instance which is not yet initialized"
        );
        assert!(
            !self.de