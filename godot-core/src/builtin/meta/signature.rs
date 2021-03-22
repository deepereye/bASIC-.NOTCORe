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
        func: fn(&mut C, Self::Params) -> Self::Ret,
        method_name: &str,
    );

    // Note: this method imposes extra bounds on GodotFfi, which may not be implemented for user types.
    // We could fall back to varcalls in such cases, and not require GodotFfi categorically.
    unsafe fn ptrcall<C: GodotClass>(
        instance_ptr: sys::GDExtensionClassInstancePtr,
        args_ptr: *const sys::GDExtensionConstTypePtr,
        ret: sys::GDExtensionTypePtr,
        func: fn(&mut C, Self::Params) -> Self::Ret,
        method_name: &str,
    );
}

// impl<P, const N: usize> Sig for [P; N]
// impl<P, T0> Sig for (T0)
// where P: VariantMetadata {
//     fn variant_type(index: usize) -> sys::GDExtensionVariantType {
//           Self[index]::
//     }
//
//     fn param_metadata(index: usize) -> sys::GDExtensionClassMethodArgumentMetadata {
//         todo!()
//     }
//
//     fn property_info(index: usize, param_name: &str) -> sys::GDExtensionPropertyInfo {
//         todo!()
//     }
// }
//
use crate::builtin::meta::*;
use crate::builtin::{FromVariant, ToVariant, Variant};
use crate::obj::GodotClass;

macro_rules! impl_signature_for_tuple {
    (
        $R:ident
        $(, $Pn:ident : $n:literal)*
    ) => {
        #[allow(unused_variables)]
        impl<$R, $($Pn,)*> SignatureTuple for ($R, $($Pn,)*)
            where $R: VariantMetadata + ToVariant + sys::GodotFuncMarshal + Debug,
               $( $Pn: VariantMetadata + FromVariant + sys::GodotFuncMarshal + Debug, )*
        {
            type Params = ($($Pn,)*);
            type Ret = $R;

            #[inline]
            fn variant_type(index: i32) -> sys::VariantType {
                match index {
                    -1 => $R::variant_type(),
                    $(
                        $n => $Pn::variant_type(),
                    )*
                    _ => unreachable!("variant_type: unavailable for index {}", index),
                    //_ => sys::GDEXTENSION_VARIANT_TYPE_NIL
                }
            }

            #[inline]
            fn param_metadata(index: i32) -> sys::GDExtensionClassMethodArgumentMetadata {
                match