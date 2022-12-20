/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{itest, TestContext};
use godot::builtin::{NodePath, Variant};
use godot::engine::{global, node, Node, Node3D, NodeExt, PackedScene, SceneTree};
use godot::obj::Share;

#[itest]
fn node_get_node() {
    let mut child = Node3D::new_alloc();
    child.set_name("child".into());
    let child_id = child.instance_id();

    let mut parent = Node3D::new_alloc();
    parent.set_name("parent".into());
    parent.add_child(
        child.share().upcast(),
        false,
        node::Interna