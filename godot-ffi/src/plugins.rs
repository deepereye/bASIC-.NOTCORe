/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Distributed self-registration of "plugins" without central list

// Note: code in this file is safe, however it seems that some annotations fall into the "unsafe" category.
// For example, adding #![forbid(unsafe_code)] causes this error:
//   note: the program's behavior with overridden link sections on items is unpredictable
//   and Rust cannot provide guarantees when you manually override them

/// Declare a global registry for plugins with a given name
#[doc(hidden)]
#[macro_export]
macro_rules! plugin_registry {
    ($vis:vis $registry:ident: $Type:ty) => {
        $crate::paste::paste! {
            #[used]
            #[allow(non_upper_case_globals)]
            #[doc(hidden)]
            $vis static [< __godot_rust_plugin_ $registry >]:
                std::sync::Mutex<Vec<$Type>> = std::sync::Mutex::new(Vec::new());
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[allow(clippy::deprecated_cfg_attr)]
#[cfg_attr(rustfmt, rustfmt::skip)]
// ^ skip: paste's [< >] syntax chokes fmt
//   cfg_attr: workaround for https://github.com/rust-lang/rust/pull/52234#issuecomm