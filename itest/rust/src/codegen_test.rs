/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// This file tests the presence, naming and accessibility of generated symbols.
// Functionality is only tested on a superficial level (to make sure general FFI mechanisms work).

use crate::itest;
use godot::builtin::inner::{InnerColor, InnerString};
use godot::engine::{FileAccess, HttpRequest};
use godot::prelude::*;

#[itest]
fn codegen_class_renamed() {
    // Known as `HTTPRequest` in Godot
    let obj = HttpRequest::new_alloc();
    obj.free();
}

#[itest]
fn codegen_base_renamed() {
    // The registration is done at startup time, so it may already fail during GDExtension init.
    // Nevertheless, try to instantiate an object with base HttpRequest her