
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::ops::*;

use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

use crate::builtin::glam_helpers::{GlamConv, GlamType};
use crate::builtin::{inner, math::*, vector3::*};

use super::{real, RQuat};
use super::{Basis, EulerOrder};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Quaternion {
    pub x: real,
    pub y: real,
    pub z: real,
    pub w: real,
}

impl Quaternion {
    pub fn new(x: real, y: real, z: real, w: real) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_angle_axis(axis: Vector3, angle: real) -> Self {
        let d = axis.length();
        if d == 0.0 {
            Self::new(0.0, 0.0, 0.0, 0.0)
        } else {
            let sin_angle = (angle * 0.5).sin();
            let cos_angle = (angle * 0.5).cos();
            let s = sin_angle / d;
            let x = axis.x * s;
            let y = axis.y * s;
            let z = axis.z * s;
            let w = cos_angle;
            Self::new(x, y, z, w)
        }
    }

    pub fn angle_to(self, to: Self) -> real {
        self.glam2(&to, RQuat::angle_between)
    }

    pub fn dot(self, with: Self) -> real {
        self.glam2(&with, RQuat::dot)
    }

    pub fn to_exp(self) -> Self {
        let mut v = Vector3::new(self.x, self.y, self.z);
        let theta = v.length();
        v = v.normalized();

        if theta < CMP_EPSILON || !v.is_normalized() {
            Self::default()
        } else {
            Self::from_angle_axis(v, theta)
        }
    }

    pub fn from_euler(self, euler: Vector3) -> Self {
        let half_a1 = euler.y * 0.5;
        let half_a2 = euler.x * 0.5;
        let half_a3 = euler.z * 0.5;
        let cos_a1 = half_a1.cos();
        let sin_a1 = half_a1.sin();
        let cos_a2 = half_a2.cos();
        let sin_a2 = half_a2.sin();
        let cos_a3 = half_a3.cos();
        let sin_a3 = half_a3.sin();

        Self::new(
            sin_a1 * cos_a2 * sin_a3 + cos_a1 * sin_a2 * cos_a3,
            sin_a1 * cos_a2 * cos_a3 - cos_a1 * sin_a2 * sin_a3,
            -sin_a1 * sin_a2 * cos_a3 + cos_a1 * cos_a2 * sin_a3,
            sin_a1 * sin_a2 * sin_a3 + cos_a1 * cos_a2 * cos_a3,
        )
    }

    pub fn get_angle(self) -> real {
        2.0 * self.w.acos()
    }

    pub fn get_axis(self) -> Vector3 {
        let Self { x, y, z, w } = self;
        let axis = Vector3::new(x, y, z);

        if self.w.abs() > 1.0 - CMP_EPSILON {
            axis
        } else {
            let r = 1.0 / (1.0 - w * w).sqrt();
            r * axis
        }
    }

    pub fn to_euler(self, order: EulerOrder) -> Vector3 {
        Basis::from_quat(self).to_euler(order)
    }

    pub fn inverse(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    pub fn is_equal_approx(self, to: Self) -> bool {
        is_equal_approx(self.x, to.x)
            && is_equal_approx(self.y, to.y)
            && is_equal_approx(self.z, to.z)
            && is_equal_approx(self.w, to.w)
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    pub fn is_normalized(self) -> bool {
        is_equal_approx(self.length_squared(), 1.0)
    }

    pub fn length(self) -> real {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> real {
        self.dot(self)
    }

    pub fn log(self) -> Self {
        let v = self.get_axis() * self.get_angle();
        Quaternion::new(v.x, v.y, v.z, 0.0)
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }

    pub fn slerp(self, to: Self, weight: real) -> Self {
        let mut cosom = self.dot(to);
        let to1: Self;
        let omega: real;
        let sinom: real;
        let scale0: real;
        let scale1: real;
        if cosom < 0.0 {
            cosom = -cosom;
            to1 = -to;
        } else {
            to1 = to;
        }

        if 1.0 - cosom > CMP_EPSILON {
            omega = cosom.acos();
            sinom = omega.sin();
            scale0 = ((1.0 - weight) * omega).sin() / sinom;
            scale1 = (weight * omega).sin() / sinom;
        } else {
            scale0 = 1.0 - weight;
            scale1 = weight;
        }

        scale0 * self + scale1 * to1
    }

    pub fn slerpni(self, to: Self, weight: real) -> Self {
        let dot = self.dot(to);
        if dot.abs() > 0.9999 {
            return self;
        }
        let theta = dot.acos();
        let sin_t = 1.0 / theta.sin();
        let new_factor = (weight * theta).sin() * sin_t;
        let inv_factor = ((1.0 - weight) * theta).sin() * sin_t;

        inv_factor * self + new_factor * to
    }

    // pub fn spherical_cubic_interpolate(self, b: Self, pre_a: Self, post_b: Self, weight: real) -> Self {}
    // TODO: Implement godot's function in rust
    /*
        pub fn spherical_cubic_interpolate_in_time(
            self,
            b: Self,
            pre_a: Self,
            post_b: Self,
            weight: real,
            b_t: real,
            pre_a_t: real,
            post_b_t: real,
        ) -> Self {
        }
    */

    #[doc(hidden)]
    pub fn as_inner(&self) -> inner::InnerQuaternion {
        inner::InnerQuaternion::from_outer(self)
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, other: Quaternion) -> Self {
        // TODO use super::glam?