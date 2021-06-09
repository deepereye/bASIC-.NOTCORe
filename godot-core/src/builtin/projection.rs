
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::ops::*;

use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

use super::glam_helpers::{GlamConv, GlamType};
use super::{inner::InnerProjection, Plane, Transform3D, Vector2, Vector4};
use super::{real, RMat4, RealConv};

/// A 4x4 matrix used for 3D projective transformations. It can represent
/// transformations such as translation, rotation, scaling, shearing, and
/// perspective division. It consists of four Vector4 columns.
///
/// For purely linear transformations (translation, rotation, and scale), it is
/// recommended to use Transform3D, as it is more performant and has a lower
/// memory footprint.
///
/// Used internally as Camera3D's projection matrix.
///
/// Note: The current implementation largely makes calls to godot for its
/// methods and as such are not as performant as other types.
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Projection {
    /// The columns of the projection matrix.
    pub cols: [Vector4; 4],
}

impl Projection {
    /// A Projection with no transformation defined. When applied to other data
    /// structures, no transformation is performed.
    ///
    /// _Godot equivalent: Projection.IDENTITY_
    pub const IDENTITY: Self = Self::from_diagonal(1.0, 1.0, 1.0, 1.0);

    /// A Projection with all values initialized to 0. When applied to other
    /// data structures, they will be zeroed.
    ///
    /// _Godot equivalent: Projection.ZERO_
    pub const ZERO: Self = Self::from_diagonal(0.0, 0.0, 0.0, 0.0);

    /// Create a new projection from a list of column vectors.
    pub const fn new(cols: [Vector4; 4]) -> Self {
        Self { cols }
    }

    /// Create a diagonal matrix from the given values.
    pub const fn from_diagonal(x: real, y: real, z: real, w: real) -> Self {
        Self::from_cols(
            Vector4::new(x, 0.0, 0.0, 0.0),
            Vector4::new(0.0, y, 0.0, 0.0),
            Vector4::new(0.0, 0.0, z, 0.0),
            Vector4::new(0.0, 0.0, 0.0, w),
        )
    }

    /// Create a matrix from four column vectors.
    ///
    /// _Godot equivalent: Projection(Vector4 x_axis, Vector4 y_axis, Vector4 z_axis, Vector4 w_axis)_
    pub const fn from_cols(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Self { cols: [x, y, z, w] }
    }

    /// Creates a new Projection that projects positions from a depth range of
    /// -1 to 1 to one that ranges from 0 to 1, and flips the projected
    /// positions vertically, according to flip_y.
    ///
    /// _Godot equivalent: Projection.create_depth_correction()_
    pub fn create_depth_correction(flip_y: bool) -> Self {
        InnerProjection::create_depth_correction(flip_y)
    }

    /// Creates a new Projection for projecting positions onto a head-mounted
    /// display with the given X:Y aspect ratio, distance between eyes, display
    /// width, distance to lens, oversampling factor, and depth clipping planes.
    ///
    /// _Godot equivalent: Projection.create_for_hmd()_
    #[allow(clippy::too_many_arguments)]
    pub fn create_for_hmd(
        eye: ProjectionEye,
        aspect: real,
        intraocular_dist: real,
        display_width: real,
        display_to_lens: real,
        oversample: real,
        near: real,
        far: real,
    ) -> Self {
        InnerProjection::create_for_hmd(
            eye as i64,
            aspect.as_f64(),
            intraocular_dist.as_f64(),
            display_width.as_f64(),
            display_to_lens.as_f64(),
            oversample.as_f64(),
            near.as_f64(),
            far.as_f64(),
        )
    }

    /// Creates a new Projection that projects positions in a frustum with the
    /// given clipping planes.
    ///
    /// _Godot equivalent: Projection.create_frustum()_
    pub fn create_frustum(
        left: real,
        right: real,
        bottom: real,
        top: real,
        near: real,
        far: real,
    ) -> Self {
        InnerProjection::create_frustum(
            left.as_f64(),
            right.as_f64(),
            bottom.as_f64(),
            top.as_f64(),
            near.as_f64(),
            far.as_f64(),
        )
    }

    /// Creates a new Projection that projects positions in a frustum with the
    /// given size, X:Y aspect ratio, offset, and clipping planes.
    ///
    /// `flip_fov` determines whether the projection's field of view is flipped
    /// over its diagonal.
    ///
    /// _Godot equivalent: Projection.create_frustum_aspect()_
    pub fn create_frustum_aspect(
        size: real,
        aspect: real,
        offset: Vector2,
        near: real,
        far: real,
        flip_fov: bool,
    ) -> Self {
        InnerProjection::create_frustum_aspect(
            size.as_f64(),
            aspect.as_f64(),
            offset,
            near.as_f64(),
            far.as_f64(),
            flip_fov,
        )
    }

    /// Creates a new Projection that projects positions using an orthogonal
    /// projection with the given clipping planes.
    ///
    /// _Godot equivalent: Projection.create_orthogonal()_
    pub fn create_orthogonal(
        left: real,
        right: real,
        bottom: real,
        top: real,
        near: real,
        far: real,
    ) -> Self {
        InnerProjection::create_orthogonal(
            left.as_f64(),
            right.as_f64(),
            bottom.as_f64(),
            top.as_f64(),
            near.as_f64(),
            far.as_f64(),
        )
    }

    /// Creates a new Projection that projects positions using an orthogonal
    /// projection with the given size, X:Y aspect ratio, and clipping planes.
    ///
    /// `flip_fov` determines whether the projection's field of view is flipped
    /// over its diagonal.
    ///
    /// _Godot equivalent: Projection.create_orthogonal_aspect()_
    pub fn create_orthogonal_aspect(
        size: real,
        aspect: real,
        near: real,
        far: real,
        flip_fov: bool,
    ) -> Self {
        InnerProjection::create_orthogonal_aspect(
            size.as_f64(),
            aspect.as_f64(),
            near.as_f64(),
            far.as_f64(),
            flip_fov,
        )
    }

    /// Creates a new Projection that projects positions using a perspective
    /// projection with the given Y-axis field of view (in degrees), X:Y aspect
    /// ratio, and clipping planes
    ///
    /// `flip_fov` determines whether the projection's field of view is flipped
    /// over its diagonal.
    ///
    /// _Godot equivalent: Projection.create_perspective()_
    pub fn create_perspective(
        fov_y: real,
        aspect: real,
        near: real,
        far: real,
        flip_fov: bool,
    ) -> Self {
        InnerProjection::create_perspective(
            fov_y.as_f64(),
            aspect.as_f64(),
            near.as_f64(),
            far.as_f64(),
            flip_fov,
        )
    }

    /// Creates a new Projection that projects positions using a perspective
    /// projection with the given Y-axis field of view (in degrees), X:Y aspect
    /// ratio, and clipping distances. The projection is adjusted for a
    /// head-mounted display with the given distance between eyes and distance
    /// to a point that can be focused on.
    ///
    /// `flip_fov` determines whether the projection's field of view is flipped
    /// over its diagonal.
    ///
    /// _Godot equivalent: Projection.create_perspective_hmd()_
    #[allow(clippy::too_many_arguments)]
    pub fn create_perspective_hmd(
        fov_y: real,
        aspect: real,
        near: real,
        far: real,
        flip_fov: bool,
        eye: ProjectionEye,
        intraocular_dist: real,
        convergence_dist: real,
    ) -> Self {
        InnerProjection::create_perspective_hmd(
            fov_y.as_f64(),
            aspect.as_f64(),
            near.as_f64(),
            far.as_f64(),
            flip_fov,
            eye as i64,
            intraocular_dist.as_f64(),
            convergence_dist.as_f64(),
        )
    }

    /// Return the determinant of the matrix.
    ///
    /// _Godot equivalent: Projection.determinant()_