/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Stub for various other built-in classes, which are currently incomplete, but whose types
// are required for codegen
use crate::builtin::{inner, StringName, Vector2};
use crate::obj::{Gd, GodotClass};
use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

// TODO: Swap more inner math types with glam types
// Note: ordered by enu