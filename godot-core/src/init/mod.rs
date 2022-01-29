/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi as sys;
use std::collections::btree_map::BTreeMap;

#[doc(hidden)]
// TODO consider body safe despite unsafe function, and explicitly mark unsafe {} locations
pub unsafe fn __gdext_load_library<E: ExtensionLibrary>(
    interface: *const sys::GDExtensionInter