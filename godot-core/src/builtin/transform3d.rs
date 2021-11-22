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
    /// _Godot equivalent: Transform3D.is_finite()_
    pub fn is_finite(&self) -> bool {
        self.basis.is_finite() && self.origin.is_finite()
    }

    /// Returns a copy of the transform rotated such that the forward axis (-Z)
    /// points towards the `target` position.
    ///
    /// See [`Basis::new_looking_at()`] for more information.
    ///
    /// _Godot equivalent: Transform3D.looking_at()_
    #[must_use]
    pub fn looking_at(self, target: Vector3, up: Vector3) -> Self {
        Self {
            basis: Basis::new_looking_at(target - self.origin, up),
            origin: self.origin,
        }
    }

    /// Returns the transform with the basis orthogonal (90 degrees), and
    /// normalized axis vectors (scale of 1 or -1).
    ///
    /// _Godot equivalent: Transform3D.orthonormalized()_
    #[must_use]
    pub fn orthonormalized(self) -> Self {
        Self {
            basis: self.basis.orthonormalized(),
            origin: self.origin,
        }
    }

    /// Returns a copy of the transform rotated by the given `angle` (in radians).
    /// This method is an optimized version of multiplying the given transform `X`
    /// with a corresponding rotation transform `R` from the left, i.e., `R * X`.
    /// This can be seen as transforming with respect to the global/parent frame.
    ///
    /// _Godot equivalent: `Transform2D.rotated()`_
    #[must_use]
    pub fn rotated(self, axis: Vector3, angle: real) -> Self {
        let rotation = Basis::from_axis_angle(axis, angle);
        Self {
            basis: rotation * self.basis,
            origin: rotation * self.origin,
        }
    }
    /// Returns a copy of the transform rotated by the given `angle` (in radians).
    /// This method is an optimized version of multiplying the given transform `X`
    /// with a corresponding rotation transform `R` from the right, i.e., `X * R`.
    /// This can be seen as transforming with respect to the local frame.
    ///
    /// _Godot equivalent: `Transform2D.rotated_local()`_
    #[must_use]
    pub fn rotated_local(self, axis: Vector3, angle: real) -> Self {
        Self {
            basis: self.basis * Basis::from_axis_angle(axis, angle),
            origin: self.origin,
        }
    }

    /// Returns a copy of the transform scaled by the given scale factor.
    /// This method is an optimized version of multiplying the given transform `X`
    /// with a corresponding scaling transform `S` from the left, i.e., `S * X`.
    /// This can be seen as transforming with respect to the global/parent frame.
    ///
    /// _Godot equivalent: `Transform2D.scaled()`_
    #[must_use]
    pub fn scaled(self, scale: Vector3) -> Self {
        Self {
            basis: Basis::from_scale(scale) * self.basis,
            origin: self.origin * scale,
        }
    }

    /// Returns a copy of the transform scaled by the given scale factor.
    /// This method is an optimized version of multiplying the given transform `X`
    /// with a corresponding scaling transform `S` from the right, i.e., `X * S`.
    /// This can be seen as transforming with respect to the local frame.
    ///
    /// _Godot equivalent: `Transform2D.scaled_local()`_
    #[must_use]
    pub fn scaled_local(self, scale: Vector3) -> Self {
        Self {
            basis: self.basis * Basis::from_scale(scale),
            origin: self.origin,
        }
    }

    /// Returns a copy of the transform translated by the given offset.
    /// This method is an optimized version of multiplying the given transform `X`
    /// with a corresponding translation transform `T` from the left, i.e., `T * X`.
    /// This can be seen as transforming with respect to the global/parent frame.
    ///
    /// _Godot equivalent: `Transform2D.translated()`_
    #[must_use]
    pub fn translated(self, offset: Vector3) -> Self {
        Self {
            basis: self.basis,
            origin: self.origin + offset,
        }
    }

    /// Returns a copy of the transform translated by the given offset.
    /// This method is an optimized version of multiplying the given transform `X`
    /// with a corresponding translation transform `T` from the right, i.e., `X * T`.
    /// This can be seen as transforming with respect to the local frame.
    ///
    /// _Godot equivalent: `Transform2D.translated()`_
    #[must_use]
    pub fn translated_local(self, offset: Vector3) -> Self {
        Self {
            basis: self.basis,
            origin: self.origin + (self.basis * offset),
        }
    }
}

impl Display for Transform3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Godot output:
        // [X: (1, 2, 3), Y: (4, 5, 6), Z: (7, 8, 9), O: (10, 11, 12)]
        // Where X,Y,O are the columns
        let [a, b, c] = self.basis.to_cols();
        let o = self.origin;

        write!(f, "[a: {a}, b: {b}, c: {c}, o: {o}]")
    }
}

impl From<Basis> for Transform3D {
    /// Create a new transform with origin `(0,0,0)` from this basis.
    fn from(basis: Basis) -> Self {
        Self::new(basis, Vector3::ZERO)
    }
}

impl Mul for Transform3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.glam2(&rhs, |a, b| a * b)
    }
}

impl Mul<Vector3> for Transform3D {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        self.glam2(&rhs, |t, v| t.transform_point3(v))
    }
}

impl Mul<real> for Transform3D {
    type Output = Self;

    fn mul(self, rhs: real) -> Self::Output {
        Self {
            basis: self.basis * rhs,
            origin: self.origin * rhs,
        }
    }
}

impl GlamType for RAffine3 {
    type Mapped = Transform3D;

    fn to_front(&self) -> Self::Mapped {
        Transform3D::new(self.matrix3.to_front(), self.translation.to_front())
    }

    // When `double-precision` is enabled this will complain. But it is
    // needed for when it is not enabled.
    #[allow(clippy::useless_conversion)]
    fn from_front(mapped: &Self::Mapped) -> Self {
        Self {
            matrix3: mapped.basis.to_glam().into(),
            translation: mapped.origin.to_glam().into(),
        }
    }
}

impl GlamConv for Transform3D {
    type Glam = RAffine3;
}

impl GodotFfi for Transform3D {
    ffi_methods! { type sys::GDExtensionTypePtr = *mut Self; .. }
}

#[cfg(test)]
mod test {
    use super::*;

    // Tests translated from Godot.

    const DUMMY_TRANSFORM: Transform3D = Transform3D::new(
        Basis::from_cols(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::ne