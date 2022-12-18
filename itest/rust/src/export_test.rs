/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::{engine::Texture, prelude::*};

// No tests currently, tests using HasProperty are in Godot scripts.

#[derive(GodotClass)]
#[class(base=Node)]
struct HasProperty {
    #[base]
    base: Base<Node>,
    #[export(getter = "get_int_val", setter = "set_int_val")]
    int_val: i32,
    #[export(getter = "get_string_val", setter = "set_string_val")]
    string_val: GodotString,
    #[export(getter = "get_object_val", setter = "set_object_val")]
    object_val: Option<Gd<Object>>,
    #[export(getter = "get_texture_val", setter = "set_texture_val", hint = PROPERTY_HINT_RESOURCE_TYPE, hint_desc = "Texture")]
    texture_val: Option<Gd<Texture>>,
}

#[godot_api]
impl HasProperty {
    #[func]
    pub fn get_int_val(&self) -> i32 {
        self.int_val
    }

    #[func]
    pub fn set_int_val(&mut self, val: i32) {
        self.int_val = val;
    }

    #[func]
    pub fn get_string_val(&self) -> God