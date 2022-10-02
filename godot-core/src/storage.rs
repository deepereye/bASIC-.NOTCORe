/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::obj::GodotClass;
use crate::out;
use godot_ffi as sys;

use std::any::type_name;
use std::cell;

/// Manages storage and lifecycle of user's extension class instances.
pub struct InstanceStorage<T: GodotClass> {
    user_instance: cell::RefCell<T>,

    // Declared after `user_instance`, is dropped last
    pub lifecycle: Lifecycle,
    godot_ref_count: i32,
}

#[derive(Copy, Clone, Debug)]
pub enum Lifecycle {
    Alive,
    Destroying,
    Dead, // reading this would typically already be too late, only best-effort in case of UB
}

/// For all Godot extension classes
impl<T: GodotClass> InstanceStorage<T> {
    pub fn construct(user_instance: T) -> Self {
        out!("    Storage::construct             <{}>", type_name::<T>());

        Self {
            user_instance: cell::RefCell::new(user_instance),
            lifecycle: Lifecycle::Alive,
            godot_ref_count: 1,
        }
    }

    pub(crate) fn on_inc_ref(&mut self) {
        self.godot_ref_count += 1;
        out!(
            "    Storage::on_inc_ref (rc={})     <{}>", // -- {:?}",
            self.godot_ref_count,
            type_name::<T>(),
            //self.user_instance
        );
    }

    pub(crate) fn on_dec_ref(&mut self) {
        self.godot_ref_count -= 1;
        out!(
            "  | Storage::on_dec_ref (rc={})     <{}>", // -- {:?}",
            self.godot_ref_count,
            type_name::<T>(),
            //self.user_instance
        );
    }

    /* pub fn destroy(&mut self) {
        assert!(
            self.user_instance.is_some(),
            "Cannot destroy user instance which is not yet initialized"
        );
        assert!(
            !self.destroyed,
            "Cannot destroy user instance multiple times"
        );
        self.user_instance = None; // drops T
                                   // TODO drop entire Storage
    }*/

    #[must_use]
    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn get(&self) -> cell::Ref<T> {
        self.user_instance.try_borrow().unwrap_or_else(|_e| {
            panic!(
                "Gd<T>::bind() failed, already bound; T = {}.\n  \
                 Make sure there is no &mut T live at the time.\n  \
                 This often occurs when calling a GDScript function/signal from Rust, which then calls again Rust code.",
                type_name::<T>()
            )
        })
    }

    pub fn get_mut(&mut self) -> cell::RefMut<T> {
        self.user_instance.try_borrow_mut().unwrap_or_else(|_e| {
            panic!(
                "Gd<T>::bind_mut() failed, already bound; T = {}.\n  \
                 Make sure there is no &T or &mut T live at the time.\n  \
                 This often occurs when calling a GDScript function/signal from Rust, which then calls again Rust code.",
                type_name::<T>()
            )
        })
    }

    pub fn mark_destroyed_by_godot(&mut self) {
        out!(
            "    Storage::mark_destroyed_by_godot", // -- {:?}",
                                                    //self.user_instance
        );
        self.lifecycle = Lifecycle::Destroying;
        out!(
            "    mark;  self={:?}, val={:?}",
            self as *mut _,
            self.lifecycle
        );
    }

    #[inline(always)]
    pub fn destroyed_by_godot(&self) -> bool {
        out!(
            "    is_d;  self={:?}, val={:?}",
            self as *const _,
            self.lifecycle
        );
        matches!(self.lifecycle, Lifecycle::Destroying | Lifecycle::Dead)
    }
}

impl<T: GodotClass> Drop for InstanceStorage<T> {
    fn drop(&mut self) {
        out!(
            "    Storage::drop (rc={})           <{}>", // -- {:?}",
            self.godot_ref_count,
            type_name::<T>(),
            //self.user_instance
        );
        //let _ = mem::take(&mut self.user_instance);
        out!(
            "    Storage::drop end              <{}>", //  -- {:?}",
            type_name::<T>(),
            //self.user_instance
        );
    }
}

/// Interprets the opaque pointer as pointing to `InstanceStorage<T>`.
///
/// Note: returns reference with unbounded lifetime; intended for local usage
///
/// # Safety
/// `instance_ptr` is assumed to point to a valid i