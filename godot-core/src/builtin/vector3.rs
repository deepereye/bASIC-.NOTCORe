
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::ops::*;

use std::fmt;

use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

use crate::builtin::math::*;
use crate::builtin::Vector3i;

use super::glam_helpers::GlamConv;
use super::glam_helpers::GlamType;
use super::{real, RVec3};

/// Vector used for 3D math using floating point coordinates.
///
/// 3-element structure that can be used to represent positions in 3D space or any other triple of
/// numeric values.
///
/// It uses floating-point coordinates of 32-bit precision, unlike the engine's `float` type which
/// is always 64-bit. The engine can be compiled with the option `precision=double` to use 64-bit
/// vectors; use the gdext library with the `double-precision` feature in that case.
///
/// See [`Vector3i`] for its integer counterpart.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    /// The vector's X component.
    pub x: real,
    /// The vector's Y component.
    pub y: real,
    /// The vector's Z component.
    pub z: real,
}

impl Vector3 {
    /// Vector with all components set to `0.0`.
    pub const ZERO: Self = Self::splat(0.0);

    /// Vector with all components set to `1.0`.
    pub const ONE: Self = Self::splat(1.0);

    /// Unit vector in -X direction. Can be interpreted as left in an untransformed 3D world.
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0);

    /// Unit vector in +X direction. Can be interpreted as right in an untransformed 3D world.
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0);

    /// Unit vector in +Y direction. Typically interpreted as up in a 3D world.
    pub const UP: Self = Self::new(0.0, 1.0, 0.0);

    /// Unit vector in -Y direction. Typically interpreted as down in a 3D world.
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0);

    /// Unit vector in -Z direction. Can be interpreted as "into the screen" in an untransformed 3D world.
    pub const FORWARD: Self = Self::new(0.0, 0.0, -1.0);

    /// Unit vector in +Z direction. Can be interpreted as "out of the screen" in an untransformed 3D world.
    pub const BACK: Self = Self::new(0.0, 0.0, 1.0);

    /// Returns a `Vector3` with the given components.
    pub const fn new(x: real, y: real, z: real) -> Self {
        Self { x, y, z }
    }

    /// Returns a new `Vector3` with all components set to `v`.
    pub const fn splat(v: real) -> Self {
        Self::new(v, v, v)
    }

    /// Constructs a new `Vector3` from a [`Vector3i`].
    pub const fn from_vector3i(v: Vector3i) -> Self {
        Self {
            x: v.x as real,
            y: v.y as real,
            z: v.z as real,
        }
    }

    /// Converts the corresponding `glam` type to `Self`.
    fn from_glam(v: RVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    /// Converts `self` to the corresponding `glam` type.
    fn to_glam(self) -> RVec3 {
        RVec3::new(self.x, self.y, self.z)
    }

    pub fn angle_to(self, to: Self) -> real {
        self.to_glam().angle_between(to.to_glam())
    }

    pub fn bezier_derivative(self, control_1: Self, control_2: Self, end: Self, t: real) -> Self {
        let x = bezier_derivative(self.x, control_1.x, control_2.x, end.x, t);
        let y = bezier_derivative(self.y, control_1.y, control_2.y, end.y, t);
        let z = bezier_derivative(self.z, control_1.z, control_2.z, end.z, t);

        Self::new(x, y, z)
    }

    pub fn bezier_interpolate(self, control_1: Self, control_2: Self, end: Self, t: real) -> Self {
        let x = bezier_interpolate(self.x, control_1.x, control_2.x, end.x, t);
        let y = bezier_interpolate(self.y, control_1.y, control_2.y, end.y, t);
        let z = bezier_interpolate(self.z, control_1.z, control_2.z, end.z, t);

        Self::new(x, y, z)
    }

    pub fn bounce(self, normal: Self) -> Self {
        -self.reflect(normal)
    }

    pub fn ceil(self) -> Self {
        Self::from_glam(self.to_glam().ceil())
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::from_glam(self.to_glam().clamp(min.to_glam(), max.to_glam()))
    }

    pub fn cross(self, with: Self) -> Self {
        Self::from_glam(self.to_glam().cross(with.to_glam()))
    }

    pub fn cubic_interpolate(self, b: Self, pre_a: Self, post_b: Self, weight: real) -> Self {
        let x = cubic_interpolate(self.x, b.x, pre_a.x, post_b.x, weight);
        let y = cubic_interpolate(self.y, b.y, pre_a.y, post_b.y, weight);
        let z = cubic_interpolate(self.z, b.z, pre_a.z, post_b.z, weight);

        Self::new(x, y, z)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn cubic_interpolate_in_time(
        self,
        b: Self,
        pre_a: Self,
        post_b: Self,
        weight: real,
        b_t: real,
        pre_a_t: real,
        post_b_t: real,
    ) -> Self {
        let x = cubic_interpolate_in_time(
            self.x, b.x, pre_a.x, post_b.x, weight, b_t, pre_a_t, post_b_t,
        );
        let y = cubic_interpolate_in_time(
            self.y, b.y, pre_a.y, post_b.y, weight, b_t, pre_a_t, post_b_t,
        );
        let z = cubic_interpolate_in_time(
            self.z, b.z, pre_a.z, post_b.z, weight, b_t, pre_a_t, post_b_t,
        );

        Self::new(x, y, z)
    }

    pub fn direction_to(self, to: Self) -> Self {
        (to - self).normalized()
    }

    pub fn distance_squared_to(self, to: Self) -> real {
        (to - self).length_squared()
    }

    pub fn distance_to(self, to: Self) -> real {
        (to - self).length()
    }

    pub fn dot(self, with: Self) -> real {
        self.to_glam().dot(with.to_glam())
    }

    pub fn floor(self) -> Self {
        Self::from_glam(self.to_glam().floor())
    }

    pub fn inverse(self) -> Self {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }

    pub fn is_equal_approx(self, to: Self) -> bool {
        is_equal_approx(self.x, to.x)
            && is_equal_approx(self.y, to.y)
            && is_equal_approx(self.z, to.z)
    }

    pub fn is_finite(self) -> bool {
        self.to_glam().is_finite()
    }

    pub fn is_normalized(self) -> bool {
        self.to_glam().is_normalized()
    }

    pub fn is_zero_approx(self) -> bool {
        is_zero_approx(self.x) && is_zero_approx(self.y) && is_zero_approx(self.z)
    }

    pub fn length_squared(self) -> real {
        self.to_glam().length_squared()
    }

    pub fn lerp(self, to: Self, weight: real) -> Self {
        Self::from_glam(self.to_glam().lerp(to.to_glam(), weight))
    }

    pub fn limit_length(self, length: Option<real>) -> Self {
        Self::from_glam(self.to_glam().clamp_length_max(length.unwrap_or(1.0)))
    }

    pub fn max_axis_index(self) -> Vector3Axis {
        if self.x < self.y {
            if self.y < self.z {
                Vector3Axis::Z
            } else {
                Vector3Axis::Y
            }
        } else if self.x < self.z {
            Vector3Axis::Z
        } else {
            Vector3Axis::X
        }
    }

    pub fn min_axis_index(self) -> Vector3Axis {
        if self.x < self.y {
            if self.x < self.z {
                Vector3Axis::X
            } else {
                Vector3Axis::Z
            }
        } else if self.y < self.z {
            Vector3Axis::Y
        } else {
            Vector3Axis::Z
        }
    }

    pub fn move_toward(self, to: Self, delta: real) -> Self {
        let vd = to - self;
        let len = vd.length();
        if len <= delta || len < CMP_EPSILON {
            to
        } else {
            self + vd / len * delta
        }
    }

    pub fn posmod(self, pmod: real) -> Self {
        Self::new(
            fposmod(self.x, pmod),
            fposmod(self.y, pmod),
            fposmod(self.z, pmod),
        )
    }