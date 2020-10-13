/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::api_parser::Enum;
use crate::special_cases::is_builtin_scalar;
use crate::{Context, ModName, RustTy, TyName};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

pub fn make_enum_definition(enum_: &Enum) -> TokenStream {
    // TODO enums which have unique ords could be represented as Rust enums
    // This would allow exhaustive matches (or at least auto-completed matches + #[non_exhaustive]). But even without #[non_exhaustive],
    // this might be a forward compa