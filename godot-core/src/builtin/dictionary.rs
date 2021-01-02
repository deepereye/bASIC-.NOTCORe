/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi as sys;

use crate::builtin::{inner, FromVariant, ToVariant, Variant};
use crate::obj::Share;
use std::fmt;
use std::marker::PhantomData;
use std::ptr::addr_of_mut;
use sys::types::OpaqueDictionary;
use sys::{ffi_methods, interface_fn, GodotFfi};

use super::VariantArray;

/// Godot's `Dictionary` type.
///
/// The keys and values of the dictionary are all `Variant`s, so they can be of different types.
/// Variants are designed to be generally cheap to clone.
///
/// # Thread safety
///
/// The same principles apply as for [`VariantArray`]. Consult its documentation for details.
#[repr(C)]
pub struct Dictionary {
    opaque: OpaqueDictionary,
}

impl Dictionary {
    fn from_opaque(opaque: OpaqueDictionary) -> Self {
        Self { opaque }
    }

    /// Constructs an empty `Dictionary`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Removes all key-value pairs from the dictionary.
    pub fn clear(&mut self) {
        self.as_inner().clear()
    }

    /// Returns a deep copy of the dictionary. All nested arrays and dictionaries are duplicated and
    /// will not be shared with the original dictionary. Note that any `Object`-derived elements will
    /// still be shallow copied.
    ///
    /// To create a shallow copy, use [`Self::duplicate_shallow`] instead. To create a new reference to
    /// the same array data, use [`Share::share`].
    ///
    /// _Godot equivalent: `dict.duplicate(true)`_
    pub fn duplicate_deep(&self) -> Self {
        self.as_inner().duplicate(true)
    }

    /// Returns a shallow copy of the dictionary. All dictionary keys and values are copied, but
    /// any reference types (such as `Array`, `Dictionary` and `Object`) will still refer to the
    /// same value.
    ///
    /// To create a deep copy, use [`Self::duplicate_deep`] instead. To create a new reference to the
    /// same dictionary data, use [`Share::sha