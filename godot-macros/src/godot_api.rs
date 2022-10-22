
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::util;
use crate::util::bail;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use venial::{AttributeValue, Declaration, Error, Function, Impl, ImplMember};

// Note: keep in sync with trait GodotExt
const VIRTUAL_METHOD_NAMES: [&str; 3] = ["ready", "process", "physics_process"];

pub fn transform(input_decl: Declaration) -> Result<TokenStream, Error> {
    let decl = match input_decl {
        Declaration::Impl(decl) => decl,
        _ => bail(
            "#[godot_api] can only be applied on impl blocks",
            input_decl,
        )?,
    };

    if decl.impl_generic_params.is_some() {
        bail(
            "#[godot_api] currently does not support generic parameters",
            &decl,
        )?;
    }

    if decl.self_ty.as_path().is_none() {
        return bail("invalid Self type for #[godot_api] impl", decl);
    };

    if decl.trait_ty.is_some() {
        transform_trait_impl(decl)
    } else {
        transform_inherent_impl(decl)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Attribute for user-declared function
enum BoundAttrType {
    Func(AttributeValue),
    Signal(AttributeValue),
}

struct BoundAttr {
    attr_name: Ident,
    index: usize,
    ty: BoundAttrType,
}

impl BoundAttr {
    fn bail<R>(self, msg: &str, method: &Function) -> Result<R, Error> {
        bail(format!("#[{}]: {}", self.attr_name, msg), &method.name)
    }
}

/// Codegen for `#[godot_api] impl MyType`
fn transform_inherent_impl(mut decl: Impl) -> Result<TokenStream, Error> {
    let class_name = util::validate_impl(&decl, None, "godot_api")?;
    let class_name_str = class_name.to_string();