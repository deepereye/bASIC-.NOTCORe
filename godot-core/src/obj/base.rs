/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::obj::Gd;
use crate::obj::GodotClass;
use crate::{engine, sys};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

/// Restricted version of `Gd`, to hold the base instance inside a user's `GodotClass`.
///
/// Behaves similarly to [`Gd`][crate::obj::Gd], but is more constrained. Cannot be constructed by the user.
pub struct Base<T: GodotClass> {
    // Internal smart pointer is never dropped. It thus acts like a weak pointer and is needed to break reference cycles between Gd<T>
    // and the user instance owned by InstanceStorage.
    //
    // There is no data apart from the opaque bytes, so no memory or resources to deallocate.
    // When triggered by Godot/GDScript, the destruction order is as follows:
    // 1.    Most-derived Godot class (C++)
    //      ...
    // 2.  RefCounted (C++)
    // 3. Object (C++) -- this triggers InstanceStorage destruction
    // 4.   Base<T>
    // 5.  User struct (GodotClass implementa