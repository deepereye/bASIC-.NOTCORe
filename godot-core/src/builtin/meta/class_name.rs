/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi as sys;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::builtin::*;
use crate::obj::GodotClass;

/// Utility to construct class names known at compile time.
/// Cannot be a function since the backing string must be retained.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct ClassName {
    backing: StringName,
}

impl C