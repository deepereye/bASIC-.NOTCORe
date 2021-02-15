/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![macro_use]

macro_rules! impl_builtin_traits_inner {
    ( Default for $Type:ty => $gd_method:ident ) => {
        impl Default for $Type {
            #[inline]
            fn default() -> Self {
                // Note: can't use from_sys_init(), as that calls the default constructor
                // (because most assignments expect initialized target type)

                let mut uninit = std::mem::MaybeUninit::<$Type>::uninit();

                unsafe {
                    let self_ptr = (*uninit.as_mut_ptr()).sys_mut();
                    sys::builtin_call! {
                        $gd_method(self_ptr, std::ptr::null_mut())
                    };

                    uninit.assume_init()
                }
            }
        }
    };

    ( Clone for $Type:ty => $gd_method:ident ) => {
        impl Clone for $Type {
            #[inline]
            fn clone(&self) -> Self {
                unsafe {
                    Self::from_sys_init_default(|self_ptr| {
                        let ctor = ::godot_ffi::builti