
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi as sys;

use crate::builtin::meta::VariantMetadata;
use crate::builtin::*;
use crate::obj::Share;
use std::fmt;
use std::marker::PhantomData;
use sys::{ffi_methods, interface_fn, GodotFfi};

/// Godot's `Array` type.
///
/// Unlike GDScript, all indices and sizes are unsigned, so negative indices are not supported.
///
/// # Typed arrays
///
/// Godot's `Array` can be either typed or untyped.
///
/// An untyped array can contain any kind of [`Variant`], even different types in the same array.
/// We represent this in Rust as `Array`, which is just a type alias for `TypedArray<Variant>`.
///
/// Godot also supports typed arrays, which are also just `Variant` arrays under the hood, but with
/// runtime checks that no values of the wrong type are put into the array. We represent this as
/// `TypedArray<T>`, where the type `T` implements `VariantMetadata`, `FromVariant` and `ToVariant`.
///
/// # Reference semantics
///
/// Like in GDScript, `Array` acts as a reference type: multiple `Array` instances may
/// refer to the same underlying array, and changes to one are visible in the other.
///
/// To create a copy that shares data with the original array, use [`Share::share()`]. If you want
/// to create a copy of the data, use [`duplicate_shallow()`] or [`duplicate_deep()`].
///
/// # Thread safety
///
/// Usage is safe if the `Array` is used on a single thread only. Concurrent reads on
/// different threads are also safe, but any writes must be externally synchronized. The Rust
/// compiler will enforce this as long as you use only Rust threads, but it cannot protect against
/// concurrent modification on other threads (e.g. created through GDScript).
// `T` must be restricted to `VariantMetadata` in the type, because `Drop` can only be implemented
// for `T: VariantMetadata` because `drop()` requires `sys_mut()`, which is on the `GodotFfi`
// trait, whose `from_sys_init()` requires `Default`, which is only implemented for `T:
// VariantMetadata`. Whew. This could be fixed by splitting up `GodotFfi` if desired.
#[repr(C)]
pub struct Array<T: VariantMetadata> {
    opaque: sys::types::OpaqueArray,
    _phantom: PhantomData<T>,
}

/// A Godot `Array` without an assigned type.
pub type VariantArray = Array<Variant>;

// TODO check if these return a typed array
impl_builtin_froms!(VariantArray;
    PackedByteArray => array_from_packed_byte_array,
    PackedColorArray => array_from_packed_color_array,
    PackedFloat32Array => array_from_packed_float32_array,
    PackedFloat64Array => array_from_packed_float64_array,
    PackedInt32Array => array_from_packed_int32_array,
    PackedInt64Array => array_from_packed_int64_array,
    PackedStringArray => array_from_packed_string_array,
    PackedVector2Array => array_from_packed_vector2_array,
    PackedVector3Array => array_from_packed_vector3_array,
);

impl<T: VariantMetadata> Array<T> {
    fn from_opaque(opaque: sys::types::OpaqueArray) -> Self {
        // Note: type is not yet checked at this point, because array has not yet been initialized!
        Self {
            opaque,
            _phantom: PhantomData,
        }
    }

    /// Returns the number of elements in the array. Equivalent of `size()` in Godot.
    pub fn len(&self) -> usize {
        to_usize(self.as_inner().size())
    }

    /// Returns `true` if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.as_inner().is_empty()
    }

    /// Returns a 32-bit integer hash value representing the array and its contents.
    ///
    /// Note: Arrays with equal content will always produce identical hash values. However, the
    /// reverse is not true. Returning identical hash values does not imply the arrays are equal,
    /// because different arrays can have identical hash values due to hash collisions.
    pub fn hash(&self) -> u32 {
        // The GDExtension interface only deals in `i64`, but the engine's own `hash()` function
        // actually returns `uint32_t`.
        self.as_inner().hash().try_into().unwrap()
    }

    /// Clears the array, removing all elements.
    pub fn clear(&mut self) {
        self.as_inner().clear();
    }

    /// Resizes the array to contain a different number of elements. If the new size is smaller,
    /// elements are removed from the end. If the new size is larger, new elements are set to
    /// [`Variant::nil()`].
    pub fn resize(&mut self, size: usize) {
        self.as_inner().resize(to_i64(size));
    }

    /// Reverses the order of the elements in the array.
    pub fn reverse(&mut self) {
        self.as_inner().reverse();
    }

    /// Sorts the array.
    ///
    /// Note: The sorting algorithm used is not
    /// [stable](https://en.wikipedia.org/wiki/Sorting_algorithm#Stability). This means that values
    /// considered equal may have their order changed when using `sort_unstable`.
    pub fn sort_unstable(&mut self) {
        self.as_inner().sort();
    }

    /// Shuffles the array such that the items will have a random order. This method uses the
    /// global random number generator common to methods such as `randi`. Call `randomize` to
    /// ensure that a new seed will be used each time if you want non-reproducible shuffling.
    pub fn shuffle(&mut self) {
        self.as_inner().shuffle();
    }

    /// Asserts that the given index refers to an existing element.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    fn check_bounds(&self, index: usize) {
        let len = self.len();
        assert!(
            index < len,
            "Array index {index} is out of bounds: length is {len}",
        );
    }

    /// Returns a pointer to the element at the given index.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    fn ptr(&self, index: usize) -> *const Variant {
        self.check_bounds(index);

        // SAFETY: We just checked that the index is not out of bounds.
        unsafe { self.ptr_unchecked(index) }
    }

    /// Returns a mutable pointer to the element at the given index.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    fn ptr_mut(&self, index: usize) -> *mut Variant {
        self.check_bounds(index);

        // SAFETY: We just checked that the index is not out of bounds.
        unsafe { self.ptr_mut_unchecked(index) }
    }

    /// Returns a pointer to the element at the given index.
    ///
    /// # Safety
    ///
    /// Calling this with an out-of-bounds index is undefined behavior.
    unsafe fn ptr_unchecked(&self, index: usize) -> *const Variant {
        let variant_ptr = interface_fn!(array_operator_index_const)(self.sys(), to_i64(index));
        Variant::ptr_from_sys(variant_ptr)
    }

    /// Returns a mutable pointer to the element at the given index.
    ///
    /// # Safety
    ///
    /// Calling this with an out-of-bounds index is undefined behavior.
    unsafe fn ptr_mut_unchecked(&self, index: usize) -> *mut Variant {
        let variant_ptr = interface_fn!(array_operator_index)(self.sys(), to_i64(index));
        Variant::ptr_from_sys_mut(variant_ptr)
    }

    #[doc(hidden)]
    pub fn as_inner(&self) -> inner::InnerArray {
        // SAFETY: The memory layout of `TypedArray<T>` does not depend on `T`.
        inner::InnerArray::from_outer_typed(self)
    }

    /// Changes the generic type on this array, without changing its contents. Needed for API
    /// functions that return a variant array even though we know its type, and for API functions
    /// that take a variant array even though we want to pass a typed one.
    ///
    /// This is marked `unsafe` since it can be used to break the invariant that a `TypedArray<T>`
    /// always holds a Godot array whose runtime type is `T`.