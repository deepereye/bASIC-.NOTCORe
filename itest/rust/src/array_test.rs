
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{expect_panic, itest};
use godot::prelude::*;

#[itest]
fn array_default() {
    assert_eq!(VariantArray::default().len(), 0);
}

#[itest]
fn array_new() {
    assert_eq!(VariantArray::new().len(), 0);
}

#[itest]
fn array_eq() {
    let a = array![1, 2];
    let b = array![1, 2];
    assert_eq!(a, b);

    let c = array![2, 1];
    assert_ne!(a, c);
}

#[itest]
fn typed_array_from_to_variant() {
    let array = array![1, 2];
    let variant = array.to_variant();
    let result = Array::try_from_variant(&variant);
    assert_eq!(result, Ok(array));
}

#[itest]
fn untyped_array_from_to_variant() {
    let array = varray![1, 2];
    let variant = array.to_variant();
    let result = VariantArray::try_from_variant(&variant);
    assert_eq!(result, Ok(array));
}

#[itest]
fn array_from_packed_array() {
    let packed_array = PackedInt32Array::from(&[42]);
    let mut array = VariantArray::from(&packed_array);
    // This tests that the resulting array doesn't secretly have a runtime type assigned to it,
    // which is not reflected in our static type. It would make sense if it did, but Godot decided
    // otherwise: we get an untyped array.
    array.push(GodotString::from("hi").to_variant());
    assert_eq!(array, varray![42, "hi"]);
}

#[itest]
fn array_from_iterator() {
    let array = Array::from_iter([1, 2]);

    assert_eq!(array.len(), 2);
    assert_eq!(array.get(0), 1);
    assert_eq!(array.get(1), 2);
}

#[itest]
fn array_from_slice() {
    let array = Array::from(&[1, 2]);

    assert_eq!(array.len(), 2);
    assert_eq!(array.get(0), 1);
    assert_eq!(array.get(1), 2);
}

#[itest]
fn array_try_into_vec() {
    let array = array![1, 2];
    let result = Vec::<i64>::try_from(&array);
    assert_eq!(result, Ok(vec![1, 2]));
}

#[itest]
fn array_iter_shared() {
    let array = array![1, 2];
    let mut iter = array.iter_shared();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

#[itest]
fn array_hash() {
    let array = array![1, 2];
    // Just testing that it converts successfully from i64 to u32.
    array.hash();
}

#[itest]
fn array_share() {
    let mut array = array![1, 2];
    let shared = array.share();
    array.set(0, 3);
    assert_eq!(shared.get(0), 3);
}

#[itest]
fn array_duplicate_shallow() {
    let subarray = array![2, 3];
    let array = varray![1, subarray];
    let duplicate = array.duplicate_shallow();
    Array::<i64>::try_from_variant(&duplicate.get(1))
        .unwrap()
        .set(0, 4);
    assert_eq!(subarray.get(0), 4);
}

#[itest]
fn array_duplicate_deep() {
    let subarray = array![2, 3];
    let array = varray![1, subarray];
    let duplicate = array.duplicate_deep();
    Array::<i64>::try_from_variant(&duplicate.get(1))
        .unwrap()
        .set(0, 4);
    assert_eq!(subarray.get(0), 2);
}

#[itest]
fn array_slice_shallow() {
    let array = array![0, 1, 2, 3, 4, 5];
    let slice = array.slice_shallow(5, 1, Some(-2));
    assert_eq!(slice, array![5, 3]);

    let subarray = array![2, 3];
    let array = varray![1, subarray];
    let slice = array.slice_shallow(1, 2, None);
    Array::<i64>::try_from_variant(&slice.get(0))
        .unwrap()
        .set(0, 4);
    assert_eq!(subarray.get(0), 4);
}

#[itest]
fn array_slice_deep() {
    let array = array![0, 1, 2, 3, 4, 5];
    let slice = array.slice_deep(5, 1, Some(-2));
    assert_eq!(slice, array![5, 3]);

    let subarray = array![2, 3];
    let array = varray![1, subarray];
    let slice = array.slice_deep(1, 2, None);
    Array::<i64>::try_from_variant(&slice.get(0))
        .unwrap()
        .set(0, 4);
    assert_eq!(subarray.get(0), 2);
}

#[itest]
fn array_get() {
    let array = array![1, 2];

    assert_eq!(array.get(0), 1);
    assert_eq!(array.get(1), 2);
    expect_panic("Array index 2 out of bounds: length is 2", || {
        array.get(2);