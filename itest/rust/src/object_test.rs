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
  