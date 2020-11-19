
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::obj::GodotClass;
use std::marker::PhantomData;

mod method;

pub struct ClassBuilder<C> {
    _c: PhantomData<C>,
}

impl<C> ClassBuilder<C>
where
    C: GodotClass,
{
    pub(crate) fn new() -> Self {
        Self { _c: PhantomData }
    }

    pub fn virtual_method<'cb, F>(&'cb mut self, name: &'cb str, method: F) -> MethodBuilder<C, F> {
        MethodBuilder::new(self, name, method)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)] // TODO rm
#[must_use]
pub struct MethodBuilder<'cb, C, F> {
    class_builder: &'cb mut ClassBuilder<C>,
    name: &'cb str,
    method: F,
}

impl<'cb, C, F> MethodBuilder<'cb, C, F> {
    pub(super) fn new(class_builder: &'cb mut ClassBuilder<C>, name: &'cb str, method: F) -> Self {
        Self {
            class_builder,
            name,
            method,
        }
    }

    pub fn done(self) {}
}