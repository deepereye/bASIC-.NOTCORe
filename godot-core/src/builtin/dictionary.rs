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
    /// same dictionary data, use [`Share::share`].
    ///
    /// _Godot equivalent: `dict.duplicate(false)`_
    pub fn duplicate_shallow(&self) -> Self {
        self.as_inner().duplicate(false)
    }

    /// Removes a key from the map, and returns the value associated with
    /// the key if the key was in the dictionary.
    ///
    /// _Godot equivalent: `erase`_
    pub fn remove<K: ToVariant>(&mut self, key: K) -> Option<Variant> {
        let key = key.to_variant();
        let old_value = self.get(key.clone());
        self.as_inner().erase(key);
        old_value
    }

    /// Reverse-search a key by its value.
    ///
    /// Unlike Godot, this will return `None` if the key does not exist and `Some(Variant::nil())` the key is `NIL`.
    ///
    /// This operation is rarely needed and very inefficient. If you find yourself needing it a lot, consider
    /// using a `HashMap` or `Dictionary` with the inverse mapping (`V` -> `K`).
    ///
    /// _Godot equivalent: `find_key`_
    pub fn find_key_by_value<V: ToVariant>(&self, value: V) -> Option<Variant> {
        let key = self.as_inner().find_key(value.to_variant());

        if !key.is_nil() || self.contains_key(key.clone()) {
            Some(key)
        } else {
            None
        }
    }

    /// Returns the value for the given key, or `None`.
    ///
    /// Note that `NIL` values are returned as `Some(Variant::nil())`, while absent values are returned as `None`.
    /// If you want to treat both as `NIL`, use [`Self::get_or_nil`].
    pub fn get<K: ToVariant>(&self, key: K) -> Option<Variant> {
        let key = key.to_variant();
        if !self.contains_key(key.clone()) {
            return None;
        }

        Some(self.get_or_nil(key))
    }

    /// Returns the value at the key in the dictionary, or `NIL` otherwise.
    ///
    /// This method does not let you differentiate `NIL` values stored as values from absent keys.
    /// If you need that, use [`Self::get`].
    ///
    /// _Godot equivalent: `dict.get(key, null)`_
    pub fn get_or_nil<K: ToVariant>(&self, key: K) -> Variant {
        self.as_inner().get(key.to_variant(), Variant::nil())
    }

    /// Returns `true` if the dictionary contains the given key.
    ///
    /// _Godot equivalent: `has`_
    pub fn contains_key<K: ToVariant>(&self, key: K) -> bool {
        let key = key.to_variant();
        self.as_inner().has(key)
    }

    /// Returns `true` if the dictionary contains all the given keys.
    ///
    /// _Godot equivalent: `has_all`_
    pub fn contains_all_keys(&self, keys: VariantArray) -> bool {
        self.as_inner().has_all(keys)
    }

    /// Returns a 32-bit integer hash value representing the dictionary and its contents.
    pub fn hash(&self) -> u32 {
        self.as_inner().hash().try_into().unwrap()
    }

    /// Creates a new `Array` containing all the keys currently in the dictionary.
    ///
    /// _Godot equivalent: `keys`_
    pub fn keys_array(&self) -> VariantArray {
        self.as_inner().keys()
    }

    /// Creates a new `Array` containing all the values currently in the dictionary.
    ///
    /// _Godot equivalent: `values`_
    pub fn values_array(&self) -> VariantArray {
        self.as_inner().values()
    }

    /// Returns true if the dictionary is empty.
    pub fn is_empty(&self) -> bool {
        self.as_inner().is_empty()
    }

    /// Copies all keys and values from `other` into `self`.
    ///
    /// If `overwrite` is true, it will overwrite pre-existing keys.
    ///
    /// _Godot equivalent: `merge`_
    pub fn extend_dictionary(&mut self, other: Self, overwrite: bool) {
        self.as_inner().merge(other, overwrite)
    }

    /// Returns the number of entries in the dictionary.
    ///
    /// This is equivalent to `size` in Godot.
    pub fn len(&self) -> usize {
        self.as_inner().size().try_into().unwrap()
    }

    /// Insert a value at the given key, returning the previous value for that key (if available).
    ///
    /// If you don't need the previous value, use [`Self::set`] instead.
    pub fn insert<K: ToVariant, V: ToVariant>(&mut self, key: K, value: V) -> Option<Variant> {
        let key = key.to_variant();
        let old_value = self.get(key.clone());
        self.set(key, value);
        old_value
    }

    /// Set a key to a given value.
    ///
    /// If you are interested in the previous value, use [`Self::insert`] instead.
    ///
    /// _Godot equivalent: `dict[key] = value`_
    pub fn set<K: ToVariant, V: ToVariant>(&mut self, key: K, value: V) {
        let key = key.to_variant();
        unsafe {
            *self.get_ptr_mut(key) = value.to_variant();
        }
    }

    /// Returns an iterator over the key-value pairs of the `Dictionary`. The pairs are each of type `(Variant, Variant)`.
    /// Each pair references the original `Dictionary`, but instead of a `&`-reference to key-value pairs as
    /// you might expect, the iterator returns a (cheap, shallow) copy of each key-value pair.
    ///
    /// Note that it's possible to modify the `Dictionary` through another reference while iterating
    /// over it. This will not result in unsoundness or crashes, but will cause the iterator to
    /// behave in an unspecified way.
    pub fn iter_shared(&self) -> Iter<'_> {
        Iter::new(self)
    }

    /// Returns an iterator over the keys `Dictionary`. The keys are each of type `Variant`. Each key references
    /// the original `Dictionary`, but instead of a `&`-reference to keys pairs as you might expect, the
    /// iterator returns a (cheap, shallow) copy of each key pair.
    ///
    /// Note that it's possible to modify the `Dictionary` through another reference while iterating
    /// over it. This will not result in unsoundness or crashes, but will cause the iterator to
    /// behave in an unspecified way.
    pub fn keys_shared(&self) -> Keys<'_> {
        Keys::new(self)
    }

    #[doc(hidden)]
    pub fn as_inner(&self) -> inner::InnerDictionary {
        inner::InnerDictionary::from_outer(self)
    }

    /// Get the pointer corresponding to the given key in the dictionary.
    ///
    /// If there exists n