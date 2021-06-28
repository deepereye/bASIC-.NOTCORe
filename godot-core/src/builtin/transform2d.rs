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
   