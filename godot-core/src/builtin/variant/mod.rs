/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::{GodotString, StringName};
use godot_ffi as sys;
use godot_ffi::GodotFfi;
use std::{fmt, ptr};
use sys::types::OpaqueVariant;
use sys::{ffi_methods, interface_fn};

mod impls;
mod variant_traits;

pub use impls::*;
pub use sys::{VariantOperator, VariantType};
pub use variant_traits::*;

#[repr(C, align(8))]
pub struct Variant {
    opaque: OpaqueVariant,
}

impl Variant {
    /// Create an empty variant (`null` value in GDScript).
    pub fn nil() -> Self {
        Self::default()
    }

    /// Create a variant holding a non-nil value.
    ///
    /// Equivalent to `value.to_variant()`.
    pub fn from<T: ToVariant>(value: T) -> Self {
        value.to_variant()
    }

    /// ⚠️ Convert to type `T`, panicking on failure.
    ///
    /// Equivalent to `T::from_variant(&self)`.
    ///
    /// # Panics
    /// When this variant holds a different type.
    pub fn to<T: FromVariant>(&self) -> T {
        T::from_variant(self)
    }

    /// Convert to type `T`, returning `Err` on failure.
    ///
    /// Equivalent to `T::try_from_variant(&self)`.
    pub fn try_to<T: FromVariant>(&self) -> Result<T, VariantConversionError> {
        T::try_from_variant(self)
    }

    /// Checks whether the variant is empty (`null` value in GDScript).
    ///
    /// See also [`Self::get_type`].
    pub fn is_nil(&self) -> bool {
        // Use get_type() rather than sys_type(), to also cover nullptr OBJECT as NIL
        self.get_type() == VariantType::Nil
    }

    /// Returns the type that is currently held by this variant.
    ///
    /// If this variant holds a type `Object` but no instance (represented as a null object pointer), then `Nil` will be returned for
    /// consistency. This may deviate from Godot behavior -- for example, calling `Node::get_node_or_null()` with an invalid
    /// path returns a variant tha