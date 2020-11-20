
use std::cmp::Ordering;
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::{fmt::Display, ops::*};

use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

use super::glam_helpers::{GlamConv, GlamType};
use super::real_consts::FRAC_PI_2;
use super::{math::*, Quaternion, Vector3};
use super::{real, RMat3, RQuat, RVec2, RVec3};

/// A 3x3 matrix, typically used as an orthogonal basis for [`Transform3D`](crate::builtin::Transform3D).
///
/// Indexing into a `Basis` is done in row-major order. So `mat[1]` would return the first *row* and not
/// the first *column*/basis vector. This means that indexing into the matrix happens in the same order
/// it usually does in math, except that we index starting at 0.
///
/// The basis vectors are the columns of the matrix, whereas the [`rows`](Self::rows) field represents
/// the row vectors.
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Basis {
    /// The rows of the matrix. These are *not* the basis vectors.  
    ///
    /// To access the basis vectors see [`col_a()`](Self::col_a), [`set_col_a()`](Self::set_col_a),
    /// [`col_b()`](Self::col_b), [`set_col_b()`](Self::set_col_b), [`col_c`](Self::col_c()),
    /// [`set_col_c()`](Self::set_col_c).
    pub rows: [Vector3; 3],
}

impl Basis {
    /// The identity basis, with no rotation or scaling applied.
    ///
    /// _Godot equivalent: `Basis.IDENTITY`_
    pub const IDENTITY: Self = Self::from_diagonal(1.0, 1.0, 1.0);

    /// The basis that will flip something along the X axis when used in a transformation.
    ///
    /// _Godot equivalent: `Basis.FLIP_X`_
    pub const FLIP_X: Self = Self::from_diagonal(-1.0, 1.0, 1.0);

    /// The basis that will flip something along the Y axis when used in a transformation.
    ///
    /// _Godot equivalent: `Basis.FLIP_Y`_
    pub const FLIP_Y: Self = Self::from_diagonal(1.0, -1.0, 1.0);

    /// The basis that will flip something along the Z axis when used in a transformation.
    ///
    /// _Godot equivalent: `Basis.FLIP_Z`_
    pub const FLIP_Z: Self = Self::from_diagonal(1.0, 1.0, -1.0);

    /// Create a new basis from 3 row vectors. These are *not* basis vectors.
    pub const fn from_rows(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Self { rows: [x, y, z] }
    }

    /// Create a new basis from 3 column vectors.
    pub const fn from_cols(a: Vector3, b: Vector3, c: Vector3) -> Self {
        Self::from_rows_array(&[a.x, b.x, c.x, a.y, b.y, c.y, a.z, b.z, c.z])
    }

    /// Create a `Basis` from an axis and angle.
    ///
    /// _Godot equivalent: `Basis(Vector3 axis, float angle)`_
    pub fn from_axis_angle(axis: Vector3, angle: real) -> Self {
        RMat3::from_axis_angle(axis.to_glam(), angle).to_front()
    }

    /// Create a diagonal matrix from the given values.
    pub const fn from_diagonal(x: real, y: real, z: real) -> Self {
        Self {
            rows: [
                Vector3::new(x, 0.0, 0.0),
                Vector3::new(0.0, y, 0.0),
                Vector3::new(0.0, 0.0, z),
            ],
        }
    }

    /// Create a diagonal matrix from the given values.
    ///
    /// _Godot equivalent: `Basis.from_scale(Vector3 scale)`
    pub const fn from_scale(scale: Vector3) -> Self {
        Self::from_diagonal(scale.x, scale.y, scale.z)
    }

    const fn from_rows_array(rows: &[real; 9]) -> Self {
        let [ax, bx, cx, ay, by, cy, az, bz, cz] = rows;
        Self::from_rows(
            Vector3::new(*ax, *bx, *cx),
            Vector3::new(*ay, *by, *cy),
            Vector3::new(*az, *bz, *cz),
        )
    }

    /// Create a `Basis` from a `Quaternion`.
    ///
    /// _Godot equivalent: `Basis(Quaternion from)`_
    pub fn from_quat(quat: Quaternion) -> Self {
        RMat3::from_quat(quat.to_glam()).to_front()
    }

    /// Create a `Basis` from three angles `a`, `b`, and `c` interpreted
    /// as Euler angles according to the given `EulerOrder`.
    ///
    /// _Godot equivalent: `Basis.from_euler(Vector3 euler, int order)`_
    pub fn from_euler(order: EulerOrder, angles: Vector3) -> Self {
        // Translated from "Basis::from_euler" in
        // https://github.com/godotengine/godot/blob/master/core/math/basis.cpp

        // We can't use glam to do these conversions since glam uses intrinsic rotations
        // whereas godot uses extrinsic rotations.
        // see https://github.com/bitshifter/glam-rs/issues/337
        let Vector3 { x: a, y: b, z: c } = angles;
        let xmat =
            Basis::from_rows_array(&[1.0, 0.0, 0.0, 0.0, a.cos(), -a.sin(), 0.0, a.sin(), a.cos()]);
        let ymat =
            Basis::from_rows_array(&[b.cos(), 0.0, b.sin(), 0.0, 1.0, 0.0, -b.sin(), 0.0, b.cos()]);
        let zmat =
            Basis::from_rows_array(&[c.cos(), -c.sin(), 0.0, c.sin(), c.cos(), 0.0, 0.0, 0.0, 1.0]);

        match order {
            EulerOrder::XYZ => xmat * ymat * zmat,
            EulerOrder::XZY => xmat * zmat * ymat,
            EulerOrder::YXZ => ymat * xmat * zmat,
            EulerOrder::YZX => ymat * zmat * xmat,
            EulerOrder::ZXY => zmat * xmat * ymat,
            EulerOrder::ZYX => zmat * ymat * xmat,
        }
    }

    /// Creates a `Basis` with a rotation such that the forward axis (-Z) points
    /// towards the `target` position.
    ///
    /// The up axis (+Y) points as close to the `up` vector as possible while
    /// staying perpendicular to the forward axis. The resulting Basis is
    /// orthonormalized. The `target` and `up` vectors cannot be zero, and
    /// cannot be parallel to each other.
    ///
    /// _Godot equivalent: `Basis.looking_at()`_
    pub fn new_looking_at(target: Vector3, up: Vector3) -> Self {
        super::inner::InnerBasis::looking_at(target, up)
    }

    /// Creates a `[Vector3; 3]` with the columns of the `Basis`.
    pub fn to_cols(self) -> [Vector3; 3] {
        self.transposed().rows
    }

    /// Creates a [`Quaternion`] representing the same rotation as this basis.
    ///
    /// _Godot equivalent: `Basis()`_
    pub fn to_quat(self) -> Quaternion {
        RQuat::from_mat3(&self.orthonormalized().to_glam()).to_front()
    }

    const fn to_rows_array(self) -> [real; 9] {
        let [Vector3 {
            x: ax,
            y: bx,
            z: cx,
        }, Vector3 {
            x: ay,
            y: by,
            z: cy,
        }, Vector3 {
            x: az,
            y: bz,
            z: cz,
        }] = self.rows;
        [ax, bx, cx, ay, by, cy, az, bz, cz]
    }

    /// Returns the scale of the matrix.
    ///
    /// _Godot equivalent: `Basis.get_scale()`_
    #[must_use]
    pub fn scale(&self) -> Vector3 {
        let det = self.determinant();
        let det_sign = if det < 0.0 { -1.0 } else { 1.0 };

        Vector3::new(
            self.col_a().length(),
            self.col_b().length(),
            self.col_c().length(),
        ) * det_sign
    }

    /// Returns the rotation of the matrix in euler angles.
    ///
    /// The order of the angles are given by `order`.
    ///
    /// _Godot equivalent: `Basis.get_euler()`_
    pub fn to_euler(self, order: EulerOrder) -> Vector3 {
        use glam::swizzles::Vec3Swizzles as _;

        let col_a = self.col_a().to_glam();
        let col_b = self.col_b().to_glam();
        let col_c = self.col_c().to_glam();

        let row_a = self.rows[0].to_glam();
        let row_b = self.rows[1].to_glam();
        let row_c = self.rows[2].to_glam();

        let major = match order {
            EulerOrder::XYZ => self.rows[0].z,
            EulerOrder::XZY => self.rows[0].y,
            EulerOrder::YXZ => self.rows[1].z,
            EulerOrder::YZX => self.rows[1].x,
            EulerOrder::ZXY => self.rows[2].y,
            EulerOrder::ZYX => self.rows[2].x,
        };

        // Return the simplest forms for pure rotations
        if let Some(pure_rotation) = match order {
            EulerOrder::XYZ => self
                .to_euler_pure_rotation(major, 1, row_a.zx())
                .map(RVec3::yxz),
            EulerOrder::YXZ => {
                self.to_euler_pure_rotation(major, 0, RVec2::new(-major, self.rows[1].y))
            }
            _ => None,
        } {
            return pure_rotation.to_front();
        }

        match order {
            EulerOrder::XYZ => {
                -Self::to_euler_inner(major, col_c.yz(), row_a.yx(), col_b.zy()).yxz()
            }
            EulerOrder::XZY => {
                Self::to_euler_inner(major, col_b.zy(), row_a.zx(), col_c.yz()).yzx()
            }
            EulerOrder::YXZ => {
                let mut vec = Self::to_euler_inner(major, col_c.xz(), row_b.xy(), row_a.yx());
                if Self::is_between_neg1_1(major).is_lt() {
                    vec.y = -vec.y;
                }
                vec
            }
            EulerOrder::YZX => {