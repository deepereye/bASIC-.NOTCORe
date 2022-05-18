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

// 