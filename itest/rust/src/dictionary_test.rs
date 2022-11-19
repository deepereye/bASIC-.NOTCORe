/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::{HashMap, HashSet};

use crate::{expect_panic, itest};
use godot::builtin::{dict, varray, Dictionary, FromVariant, ToVariant, Variant};
use godot::obj::Share;

#[itest]
fn dictionary_default() {
    assert_eq!(Dictionary::default().len(), 0);
}

#[itest]
fn dictionary_new() {
    assert_eq!(Dictionary::new().len(), 0);
}

#[itest]
fn dictionary_from_iterator() {