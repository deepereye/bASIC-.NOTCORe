/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::{fmt::Display, ops::*};

use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

use super::glam_helpers::{GlamConv, GlamType};
use super::{real, RAffine3};
use super::{Basis, Projection, Vector3};

/// Affine 3D transform (3x4 matrix).
///
/// Used for 3D linear transformations. Uses a basis + origin representation.
///
/// Ex