
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate as sys;
use std::fmt::Debug;

/// Adds methods to convert from and to Godot FFI pointers.
/// See [crate::ffi_methods] for ergonomic implementation.
pub trait GodotFfi {
    /// Construct from Godot opaque pointer.
    ///
    /// # Safety
    /// `ptr` must be a valid _type ptr_: it must follow Godot's convention to encode `Self`,
    /// which is different depending on the type.
    unsafe fn from_sys(ptr: sys::GDExtensionTypePtr) -> Self;

    /// Construct uninitialized opaque data, then initialize it with `init_fn` function.
    ///
    /// # Safety
    /// `init_fn` must be a function that correctly handles a (possibly-uninitialized) _type ptr_.
    unsafe fn from_sys_init(init_fn: impl FnOnce(sys::GDExtensionTypePtr)) -> Self;

    /// Like [`Self::from_sys_init`], but pre-initializes the sys pointer to a `Default::default()` instance
    /// before calling `init_fn`.
    ///
    /// Some FFI functions in Godot expect a pre-existing instance at the destination pointer, e.g. CoW/ref-counted
    /// builtin types like `Array`, `Dictionary`, `String`, `StringName`.
    ///
    /// If not overridden, this just calls [`Self::from_sys_init`].
    ///
    /// # Safety
    /// `init_fn` must be a function that correctly handles a (possibly-uninitialized) _type ptr_.
    unsafe fn from_sys_init_default(init_fn: impl FnOnce(sys::GDExtensionTypePtr)) -> Self
    where
        Self: Sized, // + Default
    {
        Self::from_sys_init(init_fn)

        // TODO consider using this, if all the implementors support it
        // let mut result = Self::default();
        // init_fn(result.sys_mut());
        // result
    }

    /// Return Godot opaque pointer, for an immutable operation.
    ///
    /// Note that this is a `*mut` pointer despite taking `&self` by shared-ref.
    /// This is because most of Godot's rust API is not const-correct. This can still
    /// enhance user code (calling `sys_mut` ensures no aliasing at the time of the call).
    fn sys(&self) -> sys::GDExtensionTypePtr;
