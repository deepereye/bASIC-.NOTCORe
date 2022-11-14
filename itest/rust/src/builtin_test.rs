/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::itest;
use godot::builtin::inner::*;
use godot::prelude::*;

#[itest]
fn test_builtins_vector2() {
    let vec = Vector2::new(3.0, -4.0);
    let inner: InnerVector2 = vec.as_inner();

    let len_sq = inner.length_squared();
    assert_eq!(len_sq, 25.0);

    let abs = inner.abs();
    assert_eq!(abs, Vector2::new(3.0, 4.0));

    let normalized = inner.is_normalized();
    assert!(!normalized);
}

#[itest]
fn test_builtins_array() {
    let array = VariantArray::default();
    let mut inner: InnerArray = array.as_inner();

    let a = 7.to_variant();
    