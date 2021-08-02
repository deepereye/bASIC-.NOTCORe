/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::{fmt::Display, ops::*};

use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

use super::glam_helpers::{GlamConv, GlamType};
use super::{math::*, Vector2};

use super::real_consts::PI;
use super::{real, RAffine2, RMat2};

/// Affine 2D transform (2x3 matrix).
///
/// Represents transformations such as translation, rotation, or scaling.
///
/// Expressed as a 2x3 matrix, this transform consists of a two column vectors
/// `a` and `b` representing the basis of the transform, as well as the origin:
/// ```text
/// [ a.x  b.x  origin.x ]
/// [ a.y  b.y  origin.y ]
/// ```
///
/// For methods that don't take translation into account, see [`Basis2D`].
#[derive(Default, Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Transform2D {
    /// The first basis vector.
    ///
    /// This is equivalent to the `x` field from godot.
    pub a: Vector2,

    /// The second basis vector.
    ///
    /// This is equivalent to the `y` field from godot.
    pub b: Vector2,

    /// The origin of the transform. The coordinate space defined by this transform
    /// starts at this point.
    ///
    /// _Godot equivalent: `Transform2D.origin`_
    pub origin: Vector2,
}

impl Transform2D {
    /// The identity transform, with no translation, rotation or scaling
    /// applied. When applied to other data structures, `IDENTITY` performs no
    /// transformation.
    ///
    /// _Godot equivalent: `Transform2D.IDENTITY`_
    pub const IDENTITY: Self = Self::from_basis_origin(Basis2D::IDENTITY, Vector2::ZERO);

    /// The `Transform2D` that will flip something along its x axis.
    ///
    /// _Godot equivalent: `Transform2D.FLIP_X`_
    pub const FLIP_X: Self = Self::from_basis_origin(Basis2D::FLIP_X, Vector2::ZERO);

    /// The `Transform2D` that will flip something along its y axis.
    ///
    /// _Godot equivalent: `Transform2D.FLIP_Y`_
    pub const FLIP_Y: Self = Self::from_basis_origin(Basis2D::FLIP_Y, Vector2::ZERO);

    const fn from_basis_origin(basis: Basis2D, origin: Vector2) -> Self {
        let [a, b] = basis.cols;
        Self { a, b, origin }
    }

    /// Create a new `Transform2D` with the given column vectors.
    ///
    /// _Godot equivalent: `Transform2D(Vector2 x_axis, Vector2 y_axis, Vector2 origin)_
    pub const fn from_cols(a: Vector2, b: Vector2, origin: Vector2) -> Self {
        Self { a, b, origin }
    }

    /// Create a new `Transform2D` which will rotate by the given angle.
    pub fn from_angle(angle: real) -> Self {
        Self::from_angle_origin(angle, Vector2::ZERO)
    }

    /// Create a new `Transform2D` which will rotate by `angle` and translate
    /// by `origin`.
    ///
    /// _Godot equivalent: `Transform2D(float rotation, Vector2 position)`_
    pub fn from_angle_origin(angle: real, origin: Vector2) -> Self {
        Self::from_basis_origin(Basis2D::from_angle(angle), origin)
    }

    /// Create a new `Transform2D` which will rotate by `angle`, scale by
    /// `scale`, skew by `skew` and translate by `origin`.
    ///
    /// _Godot equivalent: `Transform2D(float rotation, Vector2 scale, float skew, Vector2 position)`_
    pub fn from_angle_scale_skew_origin(
        angle: real,
        scale: Vector2,
        skew: real,
        origin: Vector2,
    ) -> Self {
        // Translated from Godot's implementation

        Self::from_basis_origin(
            Basis2D::from_cols(
                Vector2::new(angle.cos(), angle.sin()),
                Vector2::new(-(angle + skew).sin(), (angle + skew).cos()),
            )
            .scaled(scale),
            origin,
        )
    }

    /// Create a reference to the first two columns of the transform
    /// interpreted as a [`Basis2D`].
    fn basis<'a>(&'a self) -> &'a Basis2D {
        // SAFETY: Both `Basis2D` and `Transform2D` are `repr(C)`, and the
        // layout of `Basis2D` is a prefix of `Transform2D`

        unsafe { std::mem::transmute::<&'a Transform2D, &'a Basis2D>(self) }
    }

    /// Create a [`Basis2D`] from the first two columns of the transform.
    fn to_basis(self) -> Basis2D {
        Basis2D::from_cols(self.a, self.b)
    }

    /// Returns the inverse of the transform, under the assumption that the
    /// transformation is composed of rotation, scaling and translation.
    ///
    /// _Godot equivalent: `Transform2D.affine_inverse()`_
    #[must_use]
    pub fn affine_inverse(self) -> Self {
        self.glam(|aff| aff.inverse())
    }

    /// Returns the transform's rotation (in radians).
    ///
    /// _Godot equivalent: `Transform2D.get_rotation()`_
    pub fn rotation(&self) -> real {
        self.basis().rotation()
    }

    /// Returns the transform's scale.
    ///
    /// _Godot equivalent: `Transform2D.get_scale()`_
    #[must_use]
    pub fn scale(&self) -> Vector2 {
        self.basis().scale()
    }

    /// Returns the transform's skew (in radians).
    ///
    /// _Godot equivalent: `Transform2D.get_skew()`_
    #[must_use]
    pub fn skew(&self) -> real {
        self.basis().skew()
    }

    /// Returns a transform interpolated between this transform and another by
    /// a given `weight` (on the range of 0.0 to 1.0).
    ///
    /// _Godot equivalent: `Transform2D.interpolate_with()`_
    #[must_use]
    pub fn inter