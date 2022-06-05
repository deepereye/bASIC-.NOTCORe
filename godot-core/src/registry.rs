/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![allow(dead_code)] // FIXME

use crate::obj::*;
use crate::private::as_storage;
use crate::storage::InstanceStorage;
use godot_ffi as sys;

use sys::interface_fn;

use crate::bind::GodotExt;
use crate::builtin::meta::ClassName;
use crate::builtin::StringName;
use crate::out;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ptr;

/// Piece of information that is gathered by the self-registration ("plugin") system.
#[derive(Debug)]
pub struct ClassPlugin {
    pub class_name: &'static str,
    pub component: PluginComponent,
}

/// Type-erased function object, holding a `register_class` function.
#[derive(Copy, Clone)]
pub struct ErasedRegisterFn {
    // Wrapper needed because Debug can't be derived on function pointers with reference parameters, so this won't work:
    // pub type ErasedRegisterFn = fn(&mut dyn std::any::Any);
    // (see https://stackoverflow.com/q/53380040)
    pub raw: fn(&mut dyn Any),
}

impl Debug for ErasedRegisterFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:0>16x}", self.raw as usize)
    }
}

/// Represents the data part of a [`ClassPlugin`] instance.
#[derive(Debug, Clone)]
pub enum PluginComponent {
    /// Class definition itself, must always be available
    ClassDef {
        base_class_name: &'static str,

        /// Godot low-level`create` function, wired up to library-generated `init`
        generated_create_fn: Option<
            unsafe extern "C" fn(
                _class_userdata: *mut std::ffi::c_void, //
            ) -> sys::GDExtensionObjectPtr,
        >,

        free_fn: unsafe extern "C" fn(
            _class_user_data: *mut std::ffi::c_void,
            instance: sys::GDExtensionClassInstancePtr,
        ),
    },

    /// Collected from `#[godot_api] impl MyClass`
    UserMethodBinds {
        /// Callback to library-generated function which registers functions in the `impl`
        ///
        /// Always present since that's the entire point of this `impl` block.
        generated_register_fn: ErasedRegisterFn,
    },

    /// Collected from `#[godot_api] impl GodotExt for MyClass`
    UserVirtuals {
        /// Callback to user-defined `register_class` function
        user_register_fn: Option<ErasedRegisterFn>,

        /// Godot low-level`create` function, wired up to the user's `init`
        user_create_fn: Option<
            unsafe extern "C" fn(
                _class_userdata: *mut std::ffi::c_void, //
            ) -> sys::GDExtensionObjectPtr,
        >,

        /// User-defined `to_string` function
        user_to_string_fn: Option<
            unsafe extern "C" fn(
                p_instance: sys::GDExtensionClassInstancePtr,
                r_is_valid: *mut sys::GDExtensionBool,
                p_out: sys::GDExtensionStringPtr,
            ),
        >,

        /// Callback for other virtuals
        get_virtual_fn: unsafe extern "C" fn(
            p_userdata: *mut std::os::raw::c_void,
            p_name: sys::GDExtensionConstStringNamePtr,
        ) -> sys::GDExtensionClassCallVirtual,
    },
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct ClassRegistrationInfo {
    class_name: ClassName,
    parent_class_name: Option<ClassName>,
    generated_register_fn: Option<ErasedRegisterFn>,
    user_register_fn: Option<ErasedRegisterFn>,
    godot_params: sys::GDExtensionClassCreationInfo,
}

/// Registers a class with static type information.
pub fn register_class<T: GodotExt + cap::GodotInit + cap::ImplementsGodotExt>() {
    // TODO: provide overloads with only some trait impls

    out!("Manually register class {}", std::any::type_name::<T>());
    let class_name = ClassName::of::<T>();

    let godot_params = sys::GDExtensionClassCreationInfo {
        to_string_func: Some(callbacks::to_string::<T>),
        reference_func: Some(callbacks::reference::<T>),
        unreference_func: Some(callbacks::unreference::<T>),
        create_instance_func: Some(callbacks::create::<T>),
        free_instance_func: Some(callbacks::free::<T>),
        get_virtual_func: Some(callbacks::get_virtual::<T>),
        get_rid_func: None,
        class_userdata: ptr::null_mut(), // will be passed to create fn, but global per class
        ..default_creation_info()
    };

    register_class_raw(ClassRegistrationInfo {
        class_name,
        parent_class_name: Some(ClassName::of::<T::Base>()),
        generated_register_fn: None,
        user_register_fn: Some(ErasedRegisterFn {
            raw: callbacks::register_class_by_builder::<T>,
        }),
        godot_params,
    });
}

/// Lets Godot know about all classes that have self-registered through the plugin system.
pub fn auto_register_classes() {
    out!("Auto-register classes...");

    // Note: many errors are already caught by the compiler, before this runtime validation even takes place:
    // * missing #[derive(GodotClass)] or impl GodotClass for T
    // * duplicate impl GodotInit for T
    //

    let mut map = HashMap::<ClassName, ClassRegistrationInfo>::new();

    crate::private::iterate_plugins(|elem: &ClassPlugin| {
        //out!("* Plugin: {elem:#?}");

        let name