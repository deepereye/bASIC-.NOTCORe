/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builder::ClassBuilder;
use crate::builtin::GodotString;
use crate::obj::Base;
use crate::obj::GodotClass;

/// Extension API for Godot classes, used with `#[godot_api]`.
///
/// Helps with adding custom functionality:
/// * `init` constructors
/// * `to_string` method
/// * Custom register methods (builder style)
/// * All the lifecycle methods like `ready`, `process` etc.
///
/// This trait is special in that it needs to be used in combination with the `#[godot_api]`
/// proc-macro attribute to ensure proper registration of its methods. All methods have
/// default implementations, so you can select precisely which functionality you want to have.
/// Those default implementations are never called howe