/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use godot::bind::{godot_api, GodotClass, GodotExt};
use godot::builtin::{
    FromVariant, GodotString, StringName, ToVariant, Variant, VariantConversionError, Vector3,
};
use godot::engine::node::InternalMode;
use godot::engine::{file_access, Area2D, Camera3D, FileAccess, Node, Node3D, Object, RefCounted};
use godot::obj::{Base, Gd, InstanceId};
use godot::obj::{Inherits, Share};
use godot::sys::GodotFfi;

use crate::{expect_panic, 