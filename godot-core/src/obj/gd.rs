
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;

use godot_ffi as sys;
use godot_ffi::VariantType;
use sys::types::OpaqueObject;
use sys::{ffi_methods, interface_fn, static_assert_eq_size, GodotFfi};

use crate::builtin::meta::{ClassName, VariantMetadata};
use crate::builtin::{FromVariant, ToVariant, Variant, VariantConversionError};
use crate::obj::dom::Domain as _;
use crate::obj::mem::Memory as _;
use crate::obj::{cap, dom, mem, GodotClass, Inherits, Share};
use crate::obj::{GdMut, GdRef, InstanceId};
use crate::storage::InstanceStorage;
use crate::{callbacks, engine, out};

/// Smart pointer to objects owned by the Godot engine.
///
/// This smart pointer can only hold _objects_ in the Godot sense: instances of Godot classes (`Node`, `RefCounted`, etc.)
/// or user-declared structs (`#[derive(GodotClass)]`). It does **not** hold built-in types (`Vector3`, `Color`, `i32`).
///
/// `Gd<T>` never holds null objects. If you need nullability, use `Option<Gd<T>>`.
///
/// This smart pointer behaves differently depending on `T`'s associated types, see [`GodotClass`] for their documentation.
/// In particular, the memory management strategy is fully dependent on `T`:
///
/// * Objects of type [`RefCounted`] or inherited from it are **reference-counted**. This means that every time a smart pointer is
///   shared using [`Share::share()`], the reference counter is incremented, and every time one is dropped, it is decremented.
///   This ensures that the last reference (either in Rust or Godot) will deallocate the object and call `T`'s destructor.
///
/// * Objects inheriting from [`Object`] which are not `RefCounted` (or inherited) are **manually-managed**.
///   Their destructor is not automatically called (unless they are part of the scene tree). Creating a `Gd<T>` means that
///   you are responsible of explicitly deallocating such objects using [`Gd::free()`].
///
/// * For `T=Object`, the memory strategy is determined **dynamically**. Due to polymorphism, a `Gd<T>` can point to either
///   reference-counted or manually-managed types at runtime. The behavior corresponds to one of the two previous points.
///   Note that if the dynamic type is also `Object`, the memory is manually-managed.
///
/// [`Object`]: crate::engine::Object
/// [`RefCounted`]: crate::engine::RefCounted
pub struct Gd<T: GodotClass> {
    // Note: `opaque` has the same layout as GDExtensionObjectPtr == Object* in C++, i.e. the bytes represent a pointer
    // To receive a GDExtensionTypePtr == GDExtensionObjectPtr* == Object**, we need to get the address of this
    // Hence separate sys() for GDExtensionTypePtr, and obj_sys() for GDExtensionObjectPtr.
    // The former is the standard FFI type, while the latter is used in object-specific GDExtension engines.
    // pub(crate) because accessed in obj::dom
    pub(crate) opaque: OpaqueObject,

    // Last known instance ID -- this may no longer be valid!
    cached_instance_id: std::cell::Cell<Option<InstanceId>>,
    _marker: PhantomData<*const T>,
}

// Size equality check (should additionally be covered by mem::transmute())
static_assert_eq_size!(
    sys::GDExtensionObjectPtr,
    sys::types::OpaqueObject,
    "Godot FFI: pointer type `Object*` should have size advertised in JSON extension file"
);

/// _The methods in this impl block are only available for user-declared `T`, that is,
/// structs with `#[derive(GodotClass)]` but not Godot classes like `Node` or `RefCounted`._ <br><br>
impl<T> Gd<T>
where
    T: GodotClass<Declarer = dom::UserDomain>,
{
    /// Moves a user-created object into this smart pointer, submitting ownership to the Godot engine.
    ///
    /// This is only useful for types `T` which do not store their base objects (if they have a base,
    /// you cannot construct them standalone).
    pub fn new(user_object: T) -> Self {
        Self::with_base(move |_base| user_object)
    }

    /// Creates a default-constructed instance of `T` inside a smart pointer.
    ///
    /// This is equivalent to the GDScript expression `T.new()`.
    pub fn new_default() -> Self
    where
        T: cap::GodotInit,
    {
        unsafe {
            let object_ptr = callbacks::create::<T>(ptr::null_mut());
            Gd::from_obj_sys(object_ptr)
        }
    }

    /// Creates a `Gd<T>` using a function that constructs a `T` from a provided base.
    ///
    /// Imagine you have a type `T`, which has a `#[base]` field that you cannot default-initialize.
    /// The `init` function provides you with a `Base<T::Base>` object that you can use inside your `T`, which
    /// is then wrapped in a `Gd<T>`.
    ///
    /// Example:
    /// ```no_run
    /// # use godot::prelude::*;
    /// #[derive(GodotClass)]
    /// #[class(init, base=Node2D)]
    /// struct MyClass {
    ///     #[base]
    ///     my_base: Base<Node2D>,
    ///     other_field: i32,
    /// }
    ///
    /// let obj = Gd::<MyClass>::with_base(|my_base| {
    ///     // accepts the base and returns a constructed object containing it
    ///     MyClass { my_base, other_field: 732 }
    /// });
    /// ```
    pub fn with_base<F>(init: F) -> Self
    where
        F: FnOnce(crate::obj::Base<T::Base>) -> T,
    {
        let object_ptr = callbacks::create_custom(init);
        unsafe { Gd::from_obj_sys(object_ptr) }
    }

    /// Hands out a guard for a shared borrow, through which the user instance can be read.
    ///
    /// The pattern is very similar to interior mutability with standard [`RefCell`][std::cell::RefCell].
    /// You can either have multiple `GdRef` shared guards, or a single `GdMut` exclusive guard to a Rust
    /// `GodotClass` instance, independently of how many `Gd` smart pointers point to it. There are runtime
    /// checks to ensure that Rust safety rules (e.g. no `&` and `&mut` coexistence) are upheld.
    ///
    /// # Panics
    /// * If another `Gd` smart pointer pointing to the same Rust instance has a live `GdMut` guard bound.
    /// * If there is an ongoing function call from GDScript to Rust, which currently holds a `&mut T`
    ///   reference to the user instance. This can happen through re-entrancy (Rust -> GDScript -> Rust call).
    // Note: possible names: write/read, hold/hold_mut, r/w, r/rw, ...
    pub fn bind(&self) -> GdRef<T> {
        GdRef::from_cell(self.storage().get())
    }

    /// Hands out a guard for an exclusive borrow, through which the user instance can be read and written.
    ///
    /// The pattern is very similar to interior mutability with standard [`RefCell`][std::cell::RefCell].
    /// You can either have multiple `GdRef` shared guards, or a single `GdMut` exclusive guard to a Rust
    /// `GodotClass` instance, independently of how many `Gd` smart pointers point to it. There are runtime
    /// checks to ensure that Rust safety rules (e.g. no `&mut` aliasing) are upheld.
    ///
    /// # Panics
    /// * If another `Gd` smart pointer pointing to the same Rust instance has a live `GdRef` or `GdMut` guard bound.
    /// * If there is an ongoing function call from GDScript to Rust, which currently holds a `&T` or `&mut T`
    ///   reference to the user instance. This can happen through re-entrancy (Rust -> GDScript -> Rust call).
    pub fn bind_mut(&mut self) -> GdMut<T> {
        GdMut::from_cell(self.storage().get_mut())
    }

    /// Storage object associated with the extension instance
    // FIXME proper + safe interior mutability, also that Clippy is happy
    #[allow(clippy::mut_from_ref)]
    pub(crate) fn storage(&self) -> &mut InstanceStorage<T> {
        let callbacks = crate::storage::nop_instance_callbacks();

        unsafe {
            let token = sys::get_library();
            let binding =
                interface_fn!(object_get_instance_binding)(self.obj_sys(), token, &callbacks);

            debug_assert!(
                !binding.is_null(),
                "Class {} -- null instance; does the class have a Godot creator function?",
                std::any::type_name::<T>()
            );
            crate::private::as_storage::<T>(binding)
        }
    }
}

/// _The methods in this impl block are available for any `T`._ <br><br>
impl<T: GodotClass> Gd<T> {