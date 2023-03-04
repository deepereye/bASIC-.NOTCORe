
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{expect_panic, itest};
use godot::builtin::{
    dict, varray, FromVariant, GodotString, NodePath, StringName, ToVariant, Variant, Vector2,
    Vector3,
};
use godot::engine::Node2D;
use godot::obj::InstanceId;
use godot::prelude::{Basis, Dictionary, VariantArray, VariantConversionError};
use godot::sys::{GodotFfi, VariantOperator, VariantType};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

const TEST_BASIS: Basis = Basis::from_rows(
    Vector3::new(1.0, 2.0, 3.0),
    Vector3::new(4.0, 5.0, 6.0),
    Vector3::new(7.0, 8.0, 9.0),
);

#[itest]
fn variant_nil() {
    let variant = Variant::nil();
    assert!(variant.is_nil());

    let variant = 12345i32.to_variant();
    assert!(!variant.is_nil());
}

#[itest]
fn variant_conversions() {
    roundtrip(false);
    roundtrip(true);
    roundtrip(InstanceId::from_nonzero(-9223372036854775808i64));
    // roundtrip(Some(InstanceId::from_nonzero(9223372036854775807i64)));
    // roundtrip(Option::<InstanceId>::None);

    // unsigned
    roundtrip(0u8);
    roundtrip(255u8);
    roundtrip(0u16);
    roundtrip(65535u16);
    roundtrip(0u32);
    roundtrip(4294967295u32);

    // signed
    roundtrip(127i8);
    roundtrip(-128i8);
    roundtrip(32767i16);
    roundtrip(-32768i16);
    roundtrip(2147483647i32);
    roundtrip(-2147483648i32);
    roundtrip(9223372036854775807i64);

    // string
    roundtrip(gstr("some string"));
    roundtrip(String::from("some other string"));
    let str_val = "abcdefghijklmnop";
    let back = String::from_variant(&str_val.to_variant());
    assert_eq!(str_val, back.as_str());

    // basis
    roundtrip(TEST_BASIS);
}

#[itest]
fn variant_forbidden_conversions() {
    truncate_bad::<i8>(128);
}

#[itest]
fn variant_get_type() {
    let variant = Variant::nil();
    assert_eq!(variant.get_type(), VariantType::Nil);

    let variant = 74i32.to_variant();
    assert_eq!(variant.get_type(), VariantType::Int);

    let variant = true.to_variant();
    assert_eq!(variant.get_type(), VariantType::Bool);

    let variant = gstr("hello").to_variant();
    assert_eq!(variant.get_type(), VariantType::String);

    let variant = TEST_BASIS.to_variant();
    assert_eq!(variant.get_type(), VariantType::Basis)
}

#[itest]
fn variant_equal() {
    assert_eq!(Variant::nil(), ().to_variant());
    assert_eq!(Variant::nil(), Variant::default());
    assert_eq!(Variant::from(77), 77.to_variant());

    equal(77, (), false);
    equal(77, 78, false);

    assert_ne!(77.to_variant(), Variant::nil());
    assert_ne!(77.to_variant(), 78.to_variant());

    //equal(77, 77.0, false)
    equal(Vector3::new(1.0, 2.0, 3.0), Vector2::new(1.0, 2.0), false);
    equal(1, true, false);
    equal(false, 0, false);
    equal(gstr("String"), 33, false);
}

#[itest]
fn variant_call() {
    use godot::obj::Share;
    let node2d = Node2D::new_alloc();
    let variant = Variant::from(node2d.share());

    // Object
    let position = Vector2::new(4.0, 5.0);
    let result = variant.call("set_position", &[position.to_variant()]);
    assert!(result.is_nil());

    let result = variant.call("get_position", &[]);
    assert_eq!(result.try_to::<Vector2>(), Ok(position));

    let result = variant.call("to_string", &[]);
    assert_eq!(result.get_type(), VariantType::String);

    // Array