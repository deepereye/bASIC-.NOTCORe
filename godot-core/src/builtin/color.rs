/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::inner::InnerColor;
use crate::builtin::GodotString;
use godot_ffi as sys;
use std::ops;
use sys::{ffi_methods, GodotFfi};

/// Color built-in type, in floating-point RGBA format.
///
/// Channel values are _typically_ in the range of 0 to 1, but this is not a requirement, and
/// values outside this range are explicitly allowed for e.g. High Dynamic Range (HDR).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    /// The color's red component.
    pub r: f32,
    /// The color's green component.
    pub g: f32,
    /// The color's blue component.
    pub b: f32,
    /// The color's alpha component. A value of 0 means that the color is fully transparent. A
    /// value of 1 means that the color is fully opaque.
    pub a: f32,
}

impl Color {
    // TODO implement all the other color constants using code generation

    /// Transparent black.
    pub const TRANSPARENT_BLACK: Color = Self::from_rgba(0.0, 0.0, 0.0, 0.0);

    /// Transparent white.
    ///
    /// _Godot equivalent: `Color.TRANSPARENT`_
    pub const TRANSPARENT_WHITE: Color = Self::from_rgba(1.0, 1.0, 1.0, 0.0);

    /// Opaque black.
    pub const BLACK: Color = Self::from_rgba(0.0, 0.0, 0.0, 1.0);

    /// Opaque white.
    pub const WHITE: Color = Self::from_rgba(1.0, 1.0, 1.0, 1.0);

    /// Constructs a new `Color` with the given components.
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Constructs a new `Color` with the given color components, and the alpha channel set to 1.
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    /// Constructs a new `Color` with the