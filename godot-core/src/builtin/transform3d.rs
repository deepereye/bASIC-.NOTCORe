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
    pub const IDENTITY: Self = Self::new(Basis::IDENTITY, Vector3::ZERO);

    /// `Transform3D` with mirroring applied perpendicular to the YZ plane.
    ///
    /// _Godot equivalent: `Transform3D.FLIP_X`_
    pub const FLIP_X: Self = Self::new(Basis::FLIP_X, Vector3::ZERO);

    /// `Transform3D` with mirroring applied perpendicular to the XZ plane.
    ///
    /// _Godot equivalent: `Transform3D.FLIP_Y`_
    pub const FLIP_Y: Self = Self::new(Basis::FLIP_Y, Vector3::ZERO);

    /// `Transform3D` with mirroring applied perpendicular to the XY plane.
    ///
    /// _Godot equivalent: `Transform3D.FLIP_Z`_
    pub const FLIP_Z: Self = Self::new(Basis::FLIP_Z, Vector3::ZERO);

    /// Create a new transform from a [`Basis`] and a [`Vector3`].
    ///
    /// _Godot equivalent: Transform3D(Basis basis, Vector3 origin)_
    pub const fn new(basis: Basis, origin: Vector3) -> Self {
        Self { basis, origin }
    }

    /// Create a new transform from 4 matrix-columns.
    ///
    /// _Godot equivalent: Transform3D(Vector3 x_axis, Vector3 y_axis, Vector3 z_axis, Vector3 origin)_
    pub const fn from_cols(a: Vector3, b: Vector3, c: Vector3, origin: Vector3) -> Self {
        Self {
            basis: Basis::from_cols(a, b, c),
            origin,
        }
    }

    /// Constructs a Transform3d from a Projection by trimming the last row of
    /// the projection matrix.
    ///
    /// _Godot equivalent: Transform3D(Projection from)_
    pub fn from_projection(proj: Projection) -> Self {
        let a = Vector3::new(proj.cols[0].x, proj.cols[0].y, proj.cols[0].z);
        let b = Vector3::new(proj.cols[1].x, proj.cols[1].y, proj.cols[1].z);
        let c = Vector3::new(proj.cols[2].x, proj.cols[2].y, proj.cols[2].z);
        let o = Vector3::new(proj.cols[3].x, proj.cols[3].y, proj.cols[3].z);

        Self {
            basis: Basis::from_cols(a, b, c),
            origin: o,
        }
    }

    /// Returns the inverse of the transform, under the assumption that the
    /// transformation is composed of rotation, scaling and translation.
    ///
    /// _Godot equivalent: Transform3D.affine_inverse()_
    #[must_use]
    pub fn affine_inverse(self) -> Self {
        self.glam(|aff| aff.inverse())
    }

    /// Returns a transform interpolated between this transform and another by
    /// a given weight (on the range of 0.0 to 1.0).
    ///
    /// _Godot equivalent: Transform3D.interpolate_with()_
    #[must_use]
    pub fn interpolate_with(self, other: Self, weight: real) -> Self {
        let src_scale = self.basis.scale();
        let src_rot = self.basis.to_quat().normalized();
        let src_loc = self.origin;

        let dst_scale = other.basis.scale();
        let dst_rot = other.basis.to_quat().normalized();
        let dst_loc = other.origin;

        let mut basis = Basis::from_scale(src_scale.lerp(dst_scale, weight));
        basis = Basis::from_quat(src_rot.slerp(dst_rot, weight)) * basis;

        Self {
            basis,
            origin: src_loc.lerp(dst_loc, weight),
        }
    }

    /// Returns `true if this transform and transform are approximately equal, by
    /// calling is_equal_approx each basis and origin.
    ///
    /// _Godot equivalent: Transform3D.is_equal_approx()_
    pub fn is_equal_approx(&self, other: &Self) -> bool {
        self.basis.is_equal_approx(&other.basis) && self.origin.is_equal_approx(other.origin)
    }

    /// Returns `true if this transform is finite by calling `is_finite` on the
    /// basis and origin.
    ///
    /// _Godot equivalent: Transform3D.is_