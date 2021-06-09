
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::GodotString;
use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

use std::fmt;
use std::hash::{Hash, Hasher};

#[repr(C)]
pub struct StringName {