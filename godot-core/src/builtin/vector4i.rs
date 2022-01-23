/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt;

use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

use crate::builtin::Vector4;

/// Vector used for 4D math using integer coordinates.
///
/// 4-element structure that can be used to represent 4D grid coordinates or sets of integers.
///
/// It uses integer coordinates and is therefore preferable to [`Vector4`] when exact precision is
/// required. Note that the values are limited to 32 bits, and unlike [`Vector4`] this cannot be
/// configured with an engine build option. Use `i64` or [`PackedInt64Array`] if 64-bit values are
/// needed.
#[derive(Debug, Default, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector4i {
    /// The vector's X component.
    pub x: i32,
    /// The vector's Y component.
    pub y: i32,
    /// The vector's Z component.
    pub z: i32,
    /// The vector's W component.
    pub w: i32,
}

impl_vector_operators!(Vector4i, i32, (x, y, z, w));
impl_vector_index!(Vector4i, i32, (x, y, z, w), Vector4iAxis, (X, Y, Z, W));
impl_common_vector_fns!(Vector4i, i32);

impl Vector4i {
    /// Returns a `Vector4i` with the given components.
    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y,