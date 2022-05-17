
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::cell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// Immutably/shared bound reference guard for a [`Gd`][crate::obj::Gd] smart pointer.
///
/// See [`Gd::bind`][crate::obj::Gd::bind] for usage.
#[derive(Debug)]
pub struct GdRef<'a, T> {
    cell_ref: cell::Ref<'a, T>,
}

impl<'a, T> GdRef<'a, T> {
    pub(crate) fn from_cell(cell_ref: cell::Ref<'a, T>) -> Self {
        Self { cell_ref }
    }
}

impl<T> Deref for GdRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.cell_ref.deref()
    }
}

// TODO Clone or Share

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Mutably/exclusively bound reference guard for a [`Gd`][crate::obj::Gd] smart pointer.
///
/// See [`Gd::bind_mut`][crate::obj::Gd::bind_mut] for usage.
#[derive(Debug)]
pub struct GdMut<'a, T> {
    cell_ref: cell::RefMut<'a, T>,
}

impl<'a, T> GdMut<'a, T> {
    pub(crate) fn from_cell(cell_ref: cell::RefMut<'a, T>) -> Self {
        Self { cell_ref }
    }
}

impl<T> Deref for GdMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.cell_ref.deref()
    }
}

impl<T> DerefMut for GdMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.cell_ref.deref_mut()
    }
}