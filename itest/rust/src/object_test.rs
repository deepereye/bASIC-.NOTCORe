/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use godot::bind::{godot_api, GodotClass, GodotExt};
use godot::builtin::{
    FromVariant, GodotString, StringName, ToVariant, Variant, VariantConversionError, Vector3,
};
use godot::engine::node::InternalMode;
use godot::engine::{file_access, Area2D, Camera3D, FileAccess, Node, Node3D, Object, RefCounted};
use godot::obj::{Base, Gd, InstanceId};
use godot::obj::{Inherits, Share};
use godot::sys::GodotFfi;

use crate::{expect_panic, itest, TestContext};

// TODO:
// * make sure that ptrcalls are used when possible (ie. when type info available; maybe GDScript integration test)
// * Deref impl for user-defined types

#[itest]
fn object_construct_default() {
    let obj = Gd::<ObjPayload>::new_default();
    assert_eq!(obj.bind().value, 111);
}

#[itest]
fn object_construct_value() {
    let obj = Gd::new(ObjPayload { value: 222 });
    assert_eq!(obj.bind().value, 222);
}

// TODO(#23): DerefMut on Gd pointer may be used to break subtyping relations
#[itest(skip)]
fn object_subtype_swap() {
    let mut a: Gd<Node> = Node::new_alloc();
    let mut b: Gd<Node3D> = Node3D::new_alloc();

    /*
    let a_id = a.instance_id();
    let b_id = b.instance_id();
    let a_class = a.get_class();
    let b_class = b.get_class();

    dbg!(a_id);
    dbg!(b_id);
    dbg!(&a_class);
    dbg!(&b_class);
    println!("..swap..");
    */

    mem::swap(&mut *a, &mut *b);

    /*
    dbg!(a_id);
    dbg!(b_id);
    dbg!(&a_class);
    dbg!(&b_class);
    */

    // This should not panic
    a.free();
    b.free();
}

#[itest]
fn object_user_roundtrip_return() {
    let value: i16 = 17943;
    let user = ObjPayload { value };

    let obj: Gd<ObjPayload> = Gd::new(user);
    assert_eq!(obj.bind().value, value);

    let ptr = obj.sys();
    std::mem::forget(obj);

    let obj2 = unsafe { Gd::<ObjPayload>::from_sys(ptr) };
    assert_eq!(obj2.bind().value, value);
} // drop

#[itest]
fn object_user_roundtrip_write() {
    let value: i16 = 17943;
    let user = ObjPayload { value };

    let obj: Gd<ObjPayload> = Gd::new(user);
    assert_eq!(obj.bind().value, value);

    let obj2 = unsafe { Gd::<ObjPayload>::from_sys_init(|ptr| obj.write_