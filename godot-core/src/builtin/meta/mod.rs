/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod class_name;
mod signature;

pub use class_name::*;
pub use signature::*;

use crate::builtin::*;
use crate::engine::global;

use godot_ffi as sys;

/// Stores meta-information about registered types or properties.
///
/// Filling this information properly is important so that Godot can use ptrcalls instead of varcalls
/// (requires typed GDScript + sufficient information from the extension side)
pub trait VariantMetadata {
    fn variant_type() -> VariantType;

    fn class_name() -> ClassName {
        ClassName::of::<()>() // FIXME Option or so
    }

    fn property_info(property_name: &str) -> PropertyInfo {
        PropertyInfo::new(
            Self::variant_type(),
            Self::class_name(),
            StringName::from(property_name),
            global::PropertyHint::PROPERTY_HINT_NONE,
            GodotString::new(),
        )
    }

    fn param_metadata() -> sys::GDExtensionClassMethodArgumentMetadata {
        sys::GDEXTENSION_METHOD_ARGUMENT_METADATA_NONE
    }
}

impl<T: VariantMetadata> VariantMetadata for Option<T> {
    fn variant_type() -> VariantType {
        T::variant_type()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Rusty abstraction of sys::GDExtensionPropertyInfo
/// Keeps the actual allocated values (the sys equivalent only keeps pointers, which fall out of scope)
#[derive(Debug)]
pub struct PropertyInfo {
    variant_type: VariantType,
    class_name: ClassName,
    property_name: StringName,
    hint: global::PropertyHint,
    hint_string: GodotString,
    usage: global::PropertyUsageFlags,
}

impl PropertyInfo {
    pub fn new(
        variant_type: VariantType,
        class_name: ClassName,
        property_name: StringName,
        hint: global::PropertyHint,
        hint_string: GodotString,
    ) -> Self {
        Self {
            variant_type,
            class_name,
            property_name,
            hint,
            hint_string,
            usage: global::PropertyUsageFlags::PROPERTY_USAGE_DEFAULT,
        }
    }

    /// Converts to the FFI type. Keep this object allocated while u