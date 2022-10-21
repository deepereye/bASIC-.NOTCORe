/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

use crate::util::{bail, ident, validate_impl, KvParser};
use crate::ParseResult;

pub fn transform(decl: Declaration) -> ParseResult<TokenStream> {
    let mut impl_decl = match decl {
        Declaration::Impl(item) => item,
        _ => return bail("#[gdextension] can only be applied to trait impls", &decl),
    };

    validate_impl(&impl_decl, Some("ExtensionLibrary"), "gdextension")?;
    if impl_decl.tk_unsafe.