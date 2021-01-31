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
  