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
    interface: *const sys::GDExtensionInterface,
    library: sys::GDExtensionClassLibraryPtr,
    init: *mut sys::GDExtensionInitialization,
) -> sys::GDExtensionBool {
    let init_code = || {
        sys::initialize(interface, library);

        let mut handle = InitHandle::new();

        let success = E::load_library(&mut handle);
        // No early exit, unclear if Godot still requires output parameters to be set

        let godot_init_params = sys::GDExtensionInitialization {
            minimum_initialization_level: handle.lowest_init_level().to_sys(),
            userdata: std::ptr::null_mut(),
            initialize: Some(ffi_initialize_layer),
            deinitialize: Some(ffi_deinitialize_layer),
        };

        *init = godot_init_params;
        INIT_HANDLE = Some(handle);

        success as u8
    };

    let ctx = || 