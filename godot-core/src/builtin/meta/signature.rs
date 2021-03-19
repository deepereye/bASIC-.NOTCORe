/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi as sys;
use godot_ffi::VariantType;
use std::fmt::Debug;

#[doc(hidden)]
pub trait SignatureTuple {
    type Params;
    type Ret;

    fn variant_type(index: i32) -> VariantType;
    fn property_info(index: i32, param_name: &str) -> PropertyInfo;
    fn param_metadata(index: i32) -> sys::GDExtensionClassMethodArgumentMetadata;

    unsafe fn varcall<C: GodotClass>(
        instance_ptr: sys::GDExtensionClassInstancePtr,
        args_ptr: *const sys::GDExtensionConstVariantPtr,
        ret: sys::GDExtensionVariantPtr,
        err: *mut sys::GDExtensionCallError,
        fun