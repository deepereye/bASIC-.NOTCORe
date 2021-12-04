
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use super::*;
use crate::builtin::meta::VariantMetadata;
use crate::builtin::*;
use crate::obj::EngineEnum;
use godot_ffi as sys;
use sys::GodotFfi;

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Macro definitions

macro_rules! impl_variant_metadata {
    ($T:ty, $variant_type:ident $( ; $($extra:tt)* )?) => {
        impl VariantMetadata for $T {
            fn variant_type() -> VariantType {
                VariantType::$variant_type
            }

            $($($extra)*)?
        }
    };
}

macro_rules! impl_variant_traits {
    ($T:ty, $from_fn:ident, $to_fn:ident, $variant_type:ident) => {
        impl_variant_traits!(@@ $T, $from_fn, $to_fn, $variant_type;);
    };

    ($T:ty, $from_fn:ident, $to_fn:ident, $variant_type:ident, $param_metadata:ident) => {
        impl_variant_traits!(@@ $T, $from_fn, $to_fn, $variant_type;
            fn param_metadata() -> sys::GDExtensionClassMethodArgumentMetadata {
                sys::$param_metadata
            }
        );
    };

    (@@ $T:ty, $from_fn:ident, $to_fn:ident, $variant_type:ident; $($extra:tt)*) => {
        impl ToVariant for $T {
            fn to_variant(&self) -> Variant {
                let variant = unsafe {
                    Variant::from_var_sys_init(|variant_ptr| {
                        let converter = sys::builtin_fn!($from_fn);
                        converter(variant_ptr, self.sys());
                    })
                };

                variant
            }
        }

        impl FromVariant for $T {
            fn try_from_variant(variant: &Variant) -> Result<Self, VariantConversionError> {
                // Type check -- at the moment, a strict match is required.
                if variant.get_type() != Self::variant_type() {
                    return Err(VariantConversionError)
                }

                // In contrast to T -> Variant, the conversion Variant -> T assumes
                // that the destination is initialized (at least for some T). For example:
                // void String::operator=(const String &p_str) { _cowdata._ref(p_str._cowdata); }
                // does a copy-on-write and explodes if this->_cowdata is not initialized.
                // We can thus NOT use Self::from_sys_init().
                let result = unsafe {
                    Self::from_sys_init_default(|self_ptr| {
                        let converter = sys::builtin_fn!($to_fn);
                        converter(self_ptr, variant.var_sys());
                    })
                };

                Ok(result)
            }
        }

        impl_variant_metadata!($T, $variant_type; $($extra)*);
    };
}

macro_rules! impl_variant_traits_int {
    ($T:ty, $param_metadata:ident) => {
        impl ToVariant for $T {
            fn to_variant(&self) -> Variant {
                i64::from(*self).to_variant()
            }
        }

        impl FromVariant for $T {
            fn try_from_variant(v: &Variant) -> Result<Self, VariantConversionError> {
                i64::try_from_variant(v)
                    .and_then(|i| <$T>::try_from(i).map_err(|_e| VariantConversionError))
            }
        }

        impl VariantMetadata for $T {
            fn variant_type() -> VariantType {
                VariantType::Int
            }

            fn param_metadata() -> sys::GDExtensionClassMethodArgumentMetadata {
                sys::$param_metadata
            }
        }
    };
}