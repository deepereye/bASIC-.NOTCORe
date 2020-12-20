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

    /// Constructs a new `Color` with the given components as bytes. 0 is mapped to 0.0, 255 is
    /// mapped to 1.0.
    ///
    /// _Godot equivalent: the global `Color8` function_
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_rgba(from_u8(r), from_u8(g), from_u8(b), from_u8(a))
    }

    /// Constructs a new `Color` with the given components as `u16` words. 0 is mapped to 0.0,
    /// 65535 (`0xffff`) is mapped to 1.0.
    pub fn from_rgba16(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self::from_rgba(from_u16(r), from_u16(g), from_u16(b), from_u16(a))
    }

    /// Constructs a new `Color` from a 32-bits value with the given channel `order`.
    ///
    /// _Godot equivalent: `Color.hex`, if `ColorChannelOrder::Rgba` is used_
    pub fn from_u32_rgba(u: u32, order: ColorChannelOrder) -> Self {
        let [r, g, b, a] = order.unpack(u.to_be_bytes());
        Color::from_rgba8(r, g, b, a)
    }

    /// Constructs a new `Color` from a 64-bits value with the given channel `order`.
    ///
    /// _Godot equivalent: `Color.hex64`, if `ColorChannelOrder::Rgba` is used_
    pub fn from_u64_rgba(u: u64, order: ColorChannelOrder) -> Self {
        let [r, g, b, a] = order.unpack(to_be_words(u));
        Color::from_rgba16(r, g, b, a)
    }

    /// Constructs a `Color` from an HTML color code string. Valid values for the string are:
    ///
    /// - `#RRGGBBAA` and `RRGGBBAA` where each of `RR`, `GG`, `BB` and `AA` stands for two hex
    ///   digits (case insensitive).
    /// - `#RRGGBB` and `RRGGBB`. Equivalent to `#RRGGBBff`.
    /// - `#RGBA` and `RGBA` where each of `R`, `G`, `B` and `A` stands for a single hex digit.
    ///   Equivalent to `#RRGGBBAA`, i.e. each digit is repeated twice.
    /// - `#RGB` and `RGB`. Equivalent to `#RRGGBBff`.
    ///
    /// Returns `None` if the format is invalid.
    pub fn from_html<S: Into<GodotString>>(html: S) -> Option<Self> {
        let html = html.into();
        InnerColor::html_is_valid(html.clone()).then(|| InnerColor::html(html))
    }

    /// Constructs a `Color` from a string, which can be either:
    ///
    /// - An HTML color code as accepted by [`Color::from_html`].
    /// - The name of a built-in color constant, such as `BLUE` or `lawn-green`. Matching is case
    ///   insensitive and hyphens can be used interchangeably with underscores. See the [list of
    ///   color constants][color_constants] in the Godot API documentation, or the visual [cheat
    ///   sheet][cheat_sheet] for the full list.
    ///
    /// Returns `None` if the string is neither a valid HTML color code nor an existing color name.
    ///
    /// Most color constants have an alpha of 1; use [`Color::with_alpha`] to change it.
    ///
    /// [color_constants]: https://docs.godotengine.org/en/latest/classes/class_color.html#constants
    /// [cheat_sheet]: https://raw.githubusercontent.com/godotengine/godot-docs/master/img/color_constants.png
    pub fn from_string<S: Into<GodotString>>(string: S) -> Option<Self> {
        let color = InnerColor::from_string(
            string.into(),
            Self::from_rgba(f32::NAN, f32::NAN, f32::NAN, f32::NAN),
        );
        // Assumption: the implementation of `from_string` in the engine will never return any NaN
        // upon success.
        if color.r.is_nan() {
            None
        } else {
            Some(color)
        }
    }

    /// Constructs a `Color` from an [HSV profile](https://en.wikipedia.org/wiki/HSL_and_HSV). The
    /// hue (`h`), saturation (`s`), and value (`v`) are typically between 0.0 and 1.0. Alpha is
    /// set to 1; use [`Color::with_alpha`] to change it.
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        InnerColor::from_hsv(h, s, v, 1.0)
    }

    /// Constructs a `Color` from an [OK HSL
    /// profile](https://bottosson.github.io/posts/colorpicker/). The hue (`h`), saturation (`s`),
    /// and lightness (`l`) are typically between 0.0 and 1.0. Alpha is set to 1; use
    /// [`Color::with_alpha`] to change it.
    pub fn from_ok_hsl(h: f64, s: f64, l: f64) -> Self {
        InnerColor::from_ok_hsl(h, s, l, 1.0)
    }

    /// Constructs a `Color` from an RGBE9995 format integer. This is a special OpenGL texture
    /// format where the three color components have 9 bits of precision and all three share a
    /// single 5-bit exponent.
    pub fn from_rgbe9995(rgbe: u32) -> Self {
        InnerColor::from_rgbe9995(rgbe as i64)
    }

    /// Returns a copy of this color with the given alpha value. Useful for chaining with
    /// constructors like [`Color::from_string`] and [`Color::from_hsv`].
    #[must_use]
    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    /// Returns the red channel value as a byte. If `self.r` is outside the range from 0 to 1, the
    /// returned byte is clamped.
    pub fn r8(self) -> u8 {
        to_u8(self.r)
    }

    /// Returns the green channel value as a byte. If `self.g` is outside the range from 0 to 1,
    /// the returned byte is clamped.
    pub fn g8(self) -> u8 {
        to_u8(self.g)
    }

    /// Returns the blue channel value as a byte. If `self.b` is outside the range from 0 to 1, the
    /// returned byte is clamped.
    pub fn b8(self) -> u8 {
        to_u8(self.b)
    }

    /// Returns the alpha channel value as a byte. If `self.a` is outside the range from 0 to 1,
    /// the returned byte is clamped.
    pub fn a8(self) -> u8 {
        to_u8(self.a)
    }

    /// Sets the red channel value as a byte, mapped to the range from 0 to 1.
    pub fn set_r8(&mut self, r: u8) {
        self.r = from_u8(r);
    }

    /// Sets the green channel value as a byte, mapped to the range from 0 to 1.
    pub fn set_g8(&mut self, g: u8) {
        self.g = from_u8(g);
    }

    /// Sets the blue channel value as a byte, mapped to the range from 0 to 1.
    pub fn set_b8(&mut self, b: u8) {
        self.b = from_u8(b);
    }

    /// Sets the alpha channel value as a byte, mapped to the range from 0 to 1.
    pub fn set_a8(&mut self, a: u8) {
        self.a = from_u8(a);
    }

    // TODO add getters and 