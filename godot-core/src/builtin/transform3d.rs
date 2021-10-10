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
/// Expressed as a 3x4 matrix, this transform consists of 3 basis (column)
/// vectors `a`, `b`, `c` as well as an origin `o`:
/// ```text
/// [ a.x  b.x  c.x  o.x ]
/// [ a.y  b.y  c.y  o.y ]
/// [ a.z  b.z  c.z  o.z ]
/// ```
#[derive(Default, Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Transform3D {
    /// The basis is a matrix containing 3 vectors as its columns. They can be
    /// interpreted as the basis vectors of the transformed coordinate system.
    pub basis: Basis,

    /// The new origin of the transformed coordinate system.
    pub origin: Vector3,
}

impl Transform3D {
    /// The identity transform, with no translation, rotation or scaling
    /// applied. When applied to other data structures, `IDENTITY` performs no
    /// transformation.
    ///
    /// _Godot equivalent: `Transform3D.IDENTITY`_
    pub const IDENTITY: Self = Self::new(Basis