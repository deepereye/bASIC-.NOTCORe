/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::api_parser::*;
use crate::util::{to_pascal_case, to_rust_type, to_snake_case};
use crate::{ident, util, Context};

struct CentralItems {
    opaque_types: Vec<TokenStream>,
    variant_ty_enumerators_pascal: Vec<Ident>,
    variant_ty_enumerators_rust: Vec<TokenStream>,
    variant_ty_enumerators_ord: Vec<Literal>,
    variant_op_enumerators_pascal: Vec<Ident>,
    variant_op_enumerators_ord: Vec<Literal>,
    variant_fn_decls: Vec<TokenStream>,
    variant_fn_inits: Vec<TokenStream>,
    global_enum_defs: Vec<TokenStream>,
}

pub(crate) struct TypeNames {
    /// Name in JSON: "int" or "PackedVector2Array"
    pub json_builtin_name: String,

    /// "packed_vector2_array"
    pub snake_case: String,

    /// "PACKED_VECTOR2_ARRAY"
    //pub shout_case: String,

    /// GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY
    pub sys_variant_type: Ident,
}

/// Allows collecting all builtin TypeNames before generating methods
pub(crate) struct BuiltinTypeInfo<'a> {
    pub value: i32,
    pub type_names: TypeNames,

    /// If `variant_get_ptr_destructor` returns a non-null function pointer for this type.
    /// List is directly sourced from extension_api.json (information would also be in variant_destruct.cpp).
    pub has_destructor: bool,
    pub constructors: Option<&'a Vec<Constructor>>,
    pub operators: Option<&'a Vec<Operator>>,
}

pub(crate) fn generate_sys_central_file(
    api: &ExtensionApi,
    ctx: &mut Context,
    build_config: &str,
    sys_gen_path: &Path,
    out_files: &mut Vec<PathBuf>,
) {
    let central_items = make_central_items(api, build_config, ctx);
    let sys_code = make_sys_code(&central_items);

    write_file(sys_gen_path, "central.rs", sys_code, out_files);
}

pub(crate) fn generate_sys_mod_file(core_gen_path: &Path, out_files: &mut Vec<PathBuf>) {
    let code = quote! {
        pub mod central;
        pub mod gdextension_interface;
    };

    write_file(core_gen_path, "mod.rs", code.to_string(), out_files);
}

pub(crate) fn generate_core_mod_file(core_gen_path: &Path, out_files: &mut Vec<PathBuf>) {
    // When invoked by another crate during unit-test (not integration test), don't run generator
    let code = quote! {
        pub mod central;
        pub mod classes;
        pub mod builtin_classes;
        pub mod utilities;
    };

    write_file(core_gen_path, "mod.rs", code.to_string(), out_files);
}

pub(crate) fn generate_core_central_file(
    api: &ExtensionApi,
    ctx: &mut Context,
    build_config: &str,
    core_gen_path: &Path,
    out_files: &mut Vec<PathBuf>,
) {
    let central_items = make_central_items(api, build_config, ctx);
    let core_code = make_core_code(&central_items);

    write_file(core_gen_path, "central.rs", core_code, out_files);
}

pub(crate) fn write_file(
    gen_path: &Path,
    filename: &str,
    code: String,
    out_files: &mut Vec<PathBuf>,
) {
    let _ = std::fs::create_dir_all(gen_path);
    let out_path = gen_path.join(filename);

    std::fs::write(&out_path, code).unwrap_or_else(|e| {
        panic!(
            "failed to write code file to {};\n\t{}",
            out_path.display(),
            e
        )
    });
    out_files.push(out_path);
}

fn make_sys_code(central_items: &CentralItems) -> String {
    let CentralItems {
        opaque_types,
        variant_ty_enumerators_pascal,
        variant_ty_enumerators_ord,
        variant_op_enumerators_pascal,
        variant_op_enumerators_ord,
        variant_fn_decls,
        variant_fn_inits,
        ..
    } = central_items;

    let sys_tokens = quote! {
        use crate::{GDExtensionVariantPtr, GDExtensionTypePtr, GDExtensionConstTypePtr, GodotFfi, ffi_methods};

        pub mod types {
            #(#opaque_types)*
        }

        pub struct GlobalMethodTable {
            #(#variant_fn_decls)*
        }

        impl GlobalMethodTable {
            pub(crate) unsafe fn new(interface: &crate::GDExtensionInterface) -> Self {
                Self {
                    #(#variant_fn_inits)*
                }
            }
        }

        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[repr(i32)]
        pub enum VariantType {
            Nil = 0,
            #(
                #variant_ty_enumerators_pascal = #variant_ty_enumerators_ord,
            )*
        }

        impl VariantType {
            #[doc(hidden)]
            pub fn from_sys(enumerator: crate::GDExtensionVariantType) -> Self {
                // Annoying, but only stable alternative is transmute(), which dictates enum size
                match enumerator {
                    0 => Self::Nil,
                    #(
                        #variant_ty_enumerators_ord => Self::#variant_ty_enumerators_pascal,
            