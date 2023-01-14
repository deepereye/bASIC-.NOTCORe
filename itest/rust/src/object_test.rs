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

    let obj2 = unsafe { Gd::<ObjPayload>::from_sys_init(|ptr| obj.write_sys(ptr)) };
    std::mem::forget(obj);
    assert_eq!(obj2.bind().value, value);
} // drop

#[itest]
fn object_engine_roundtrip() {
    let pos = Vector3::new(1.0, 2.0, 3.0);

    let mut obj: Gd<Node3D> = Node3D::new_alloc();
    obj.set_position(pos);
    assert_eq!(obj.get_position(), pos);

    let ptr = obj.sys();

    let obj2 = unsafe { Gd::<Node3D>::from_sys(ptr) };
    assert_eq!(obj2.get_position(), pos);
    obj.free();
}

#[itest]
fn object_display() {
    let obj = Node3D::new_alloc();
    let id = obj.instance_id();

    let actual = format!(".:{obj}:.");
    let expected = format!(".:<Node3D#{id}>:.");

    assert_eq!(actual, expected);
    obj.free();
}

#[itest]
fn object_debug() {
    let obj = Node3D::new_alloc();
    let id = obj.instance_id();

    let actual = format!(".:{obj:?}:.");
    let expected = format!(".:Gd {{ id: {id}, class: Node3D }}:.");

    assert_eq!(actual, expected);
    obj.free();
}

#[itest]
fn object_instance_id() {
    let value: i16 = 17943;
    let user = ObjPayload { value };

    let obj: Gd<ObjPayload> = Gd::new(user);
    let id = obj.instance_id();

    let obj2 = Gd::<ObjPayload>::from_instance_id(id);
    assert_eq!(obj2.bind().value, value);
}

#[itest]
fn object_instance_id_when_freed() {
    let node: Gd<Node3D> = Node3D::new_alloc();
    assert!(node.is_instance_valid());

    node.share().free(); // destroys object without moving out of reference
    assert!(!node.is_instance_valid());

    expect_panic("instance_id() on dead object", move || {
        node.instance_id();
    });
}

#[itest]
fn object_from_invalid_instance_id() {
    let id = InstanceId::try_from_i64(0xDEADBEEF).unwrap();

    let obj2 = Gd::<ObjPayload>::try_from_instance_id(id);
    assert!(obj2.is_none());
}

#[itest]
fn object_from_instance_id_inherits_type() {
    let descr = GodotString::from("some very long description");

    let mut node: Gd<Node3D> = Node3D::new_alloc();
    node.set_editor_description(descr.clone());

    let id = node.instance_id();

    let node_as_base = Gd::<Node>::from_instance_id(id);
    assert_eq!(node_as_base.instance_id(), id);
    assert_eq!(node_as_base.get_editor_description(), descr);

    node_as_base.free();
}

#[itest]
fn object_from_instance_id_unrelated_type() {
    let node: Gd<Node3D> = Node3D::new_alloc();
    let id = node.instance_id();

    let obj = Gd::<RefCounted>::try_from_instance_id(id);
    assert!(
        obj.is_none(),
        "try_from_instance_id() with bad type must fail"
    );

    node.free();
}

#[itest]
fn object_user_eq() {
    let value: i16 = 17943;
    let a = ObjPayload { value };
    let b = ObjPayload { value };

    let a1 = Gd::new(a);
    let a2 = a1.share();
    let b1 = Gd::new(b);

    assert_eq!(a1, a2);
    assert_ne!(a1, b1);
    assert_ne!(a2, b1);
}

#[itest]
fn object_engine_eq() {
    let a1 = Node3D::new_alloc();
    let a2 = a1.share();
    let b1 = Node3D::new_alloc();

    assert_eq!(a1, a2);
    assert_ne!(a1, b1);
    assert_ne!(a2, b1);

    a1.free();
    b1.free();
}

#[itest]
fn object_dead_eq() {
    let a = Node3D::new_alloc();
    let b = Node3D::new_alloc();
    let b2 = b.share();

    // Destroy b1 without consuming it
    b.share().free();

    {
        let lhs = a.share();
        expect_panic("Gd::eq() panics when one operand is dead", move || {
            let _ = lhs == b;
        });
    }
    {
        let rhs = a.share();
        expect_panic("Gd::ne() panics when one operand is dead", move || {
            let _ = b2 != rhs;
        });
    }

    a.free();
}

#[itest]
fn object_user_convert_variant() {
    let value: i16 = 17943;
    let user = ObjPayload { value };

    let obj: Gd<ObjPayload> = Gd::new(user);
    let variant = obj.to_variant();
    let obj2 = Gd::<ObjPayload>::from_variant(&variant);

    assert_eq!(obj2.bind().value, value);
}

#[itest]
fn object_engine_convert_variant() {
    let pos = Vector3::new(1.0, 2.0, 3.0);

    let mut obj: Gd<Node3D> = Node3D::new_alloc();
    obj.set_position(pos);

    let variant = obj.to_variant();
    let obj2 = Gd::<Node3D>::from_variant(&variant);

    assert_eq!(obj2.get_position(), pos);
    obj.free();
}

#[itest]
fn object_user_convert_variant_refcount() {
    let obj: Gd<ObjPayload> = Gd::new(ObjPayload { value: -22222 });
    let obj = obj.upcast::<RefCounted>();
    check_convert_variant_refcount(obj)
}

#[itest]
fn object_engine_convert_variant_refcount() {
    let obj = RefCounted::new();
    check_convert_variant_refcount(obj);
}

/// Converts between Object <-> Variant and verifies the reference counter at each stage.
fn check_convert_variant_refcount(obj: Gd<RefCounted>) {
    // Freshly created -> refcount 1
    assert_eq!(obj.get_reference_count(), 1);

    {
        // Variant created from object -> increment
        let variant = obj.to_variant();
        assert_eq!(obj.get_reference_count(), 2);

        {
            // Yet another object created *from* variant -> increment
            let another_object = variant.to::<Gd<RefCounted>>();
            assert_eq!(obj.get_reference_count(), 3);
            assert_eq!(another_object.get_reference_count(), 3);
        }

        // `another_object` destroyed -> decrement
        assert_eq!(obj.get_reference_count(), 2);
    }

    // `variant` destroyed -> decrement
    assert_eq!(obj.get_reference_count(), 1);
}

#[itest]
fn object_engine_convert_variant_nil() {
    let nil = Variant::nil();

    assert_eq!(
        Gd::<Area2D>::try_from_variant(&nil),
        Err(VariantConversionError),
        "try_from_variant(&nil)"
    );

    expect_panic("from_variant(&nil)", || {
        Gd::<Area2D>::from_variant(&nil);
    });
}

#[itest]
fn object_engine_returned_refcount() {
    let Some(file) = FileAccess::open("res://itest.gdextension".into(), file_access::ModeFlags::READ) else {
        panic!("failed to open file used to test FileAccess")
    };
    assert!(file.is_open());

    // There was a bug which incremented ref-counts of just-returned objects, keep this as regression test.
    assert_eq!(file.get_reference_count(), 1);
}

#[itest]
fn object_engine_up_deref() {
    let node3d: Gd<Node3D> = Node3D::new_alloc();
    let id = node3d.instance_id();

    // Deref chain: Gd<Node3D> -> &Node3D -> &Node -> &Object
    assert_eq!(node3d.instance_id(), id);
    assert_eq!(node3d.get_class(), GodotString::from("Node3D"));

    node3d.free();
}

#[itest]
fn object_engine_up_deref_mut() {
    let mut node3d: Gd<Node3D> = Node3D::new_alloc();

    // DerefMut chain: Gd<Node3D> -> &mut Node3D -> &mut Node -> &mut Object
    node3d.set_message_translation(true);
    assert!(node3d.can_translate_messages());

    // DerefMut chain: &mut Node3D -> ...
    let node3d_ref = &mut *node3d;
    node3d_ref.set_message_translation(false);
    assert!(!node3d_ref.can_translate_messages());

    node3d.free();
}

#[itest]
fn object_engine_upcast() {
    let node3d: Gd<Node3D> = Node3D::new_alloc();
    let id = node3d.instance_id();

    let object = node3d.upcast::<Object>();
    assert_eq!(object.instance_id(), id);
    assert_eq!(object.get_class(), GodotString::from("Node3D"));

    // Deliberate free on upcast object
    object.free();
}

#[itest]
fn object_engine_upcast_reflexive() {
    let node3d: Gd<Node3D> = Node3D::new_alloc();
    let id = node3d.instance_id();

    let object = node3d.upcast::<Node3D>();
    assert_eq!(object.instance_id(), id);
    assert_eq!(object.get_class(), GodotString::from("Node3D"));

    object.free();
}

#[itest]
fn object_engine_downcast() {
    let pos = Vector3::new(1.0, 2.0, 3.0);
    let mut node3d: Gd<Node3D> = Node3D::new_alloc();
    node3d.set_position(pos);
    let id = node3d.instance_id();

    let object = node3d.upcast::<Object>();
    let node: Gd<Node> = object.cast::<Node>();
    let node3d: Gd<Node3D> = node.try_cast::<Node3D>().expect("try_cast");

    assert_eq!(node3d.instance_id(), id);
    assert_eq!(node3d.get_position(), pos);

    node3d.free();
}

#[itest]
fn object_engine_downcast_reflexive() {
    let node3d: Gd<Node3D> = Node3D::new_alloc();
    let id = node3d.instance_id();

    let node3d: Gd<Node3D> = node3d.cast::<Node3D>();
    assert_eq!(node3d.instance_id(), id);

    node3d.free();
}

#[itest]
fn object_engine_bad_downcast() {
    let object: Gd<Object> = Object::new_alloc();
    let free_ref = object.share();
    let node3d: Option<Gd<Node3D>> = object.try_cast::<Node3D>();

    assert!(node3d.is_none());
    free_ref.free();
}

#[itest]
fn object_engine_accept_polymorphic() {
    let mut node = Camera3D::new_alloc();
    let expected_name = StringName::from("Node name");
    let expected_class = GodotString::from("Camera3D");

    node.set_name(GodotString::from(&expected_name));

    let actual_name = accept_node(node.share());
    assert_eq!(actual_name, expected_name);

    let actual_class = accept_object(node.share());
    assert_eq!(actual_class, expected_class);

    node.free();
}

#[itest]
fn object_user_accept_polymorphic() {
    let obj = Gd::new(ObjPayload { value: 123 });
    let expected_class = GodotString::from("ObjPayload");

    let actual_class = accept_refcounted(obj.share());
    assert_eq!(actual_class, expected_class);

    let actual_class = accept_object(obj);
    assert_eq!(actual_class, expected_class);
}

fn accept_node<T>(node: Gd<T>) -> StringName
where
    T: Inherits<Node>,
{
    let up = node.upcast();
    up.get_name()
}

fn accept_refcounted<T>(node: Gd<T>) -> GodotString
where
    T: Inherits<RefCounted>,
{
    let up = node.upcast();
    up.get_class()
}

fn accept_object<T>(node: Gd<T>) -> GodotString
where
    T: Inherits<Object>,
{
    let up = node.upcast();
    up.get_class()
}

#[itest]
fn object_user_upcast() {
    l