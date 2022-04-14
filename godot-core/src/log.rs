/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[macro_export]
macro_rules! godot_warn {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            let msg = format!("{}\0", format_args!($fmt $(, $args)*));

            $crate::sys::interface_fn!(print_warning)(
                msg.as_bytes().as_ptr() as *const _,
                "<function unset>\0".as_bytes().as_ptr() as *const _,
                concat!(file!(), "\0").as_ptr() as *const _,
                line!() as _,
                false as $crate::sys::GDExtensionBool, // whether to create a toast notification in editor
            );
        }
    };
}

#[macro_export]
macro_rules! godot_error {
    // FIXME expr needs to be parenthesised, see usages
    ($fmt:literal $(, $args:expr)* $(,)?) => {
    //($($args:tt),* $(,)?) => {
        unsafe {
            let msg = format!("{}\0", format_args!($fmt $(, $args)*));

            $crate::sys::interface_fn!(print_error)(
                msg.as_bytes().as_ptr() as *const _,
                "<function unset>\0".as_bytes().as_ptr() as *const _,
                concat!(file!(), "\0").as_ptr() as *const _,
                line!() as _,
                false as $crate::sys::GDExtensionBool, // whether to create a toast notification in editor
            );
        }
    };
}

#[macro_export]
macro_rules! godot_script_error {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            let msg = format!("{}\0", format_args!($fmt $(, $args)*));

            $crate::sys::interface_fn!(print_script_error)(
                msg.as_bytes().as_ptr() as *const _,
                "<function unset>\0".as_bytes().as_ptr() as *const _,
                concat!(file!(), "\0").as_ptr() as *const _,
                line!() as _,
                false as $crate::sys::GDExtensionBool, // whether to create a toast notification in editor
            );
        }
    };
}

#[macro_export]
macro_rules! godot_print {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        $crate::log::print(&[
            $crate::builtin::Variant::from(
                $crate::builtin::GodotString::from(
                    forma