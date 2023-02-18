/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::itest;

use godot::prelude::{inner::InnerTransform3D, *};
use godot::private::class_macros::assert_eq_approx;

const TEST_TRANSFORM: Transform3D = Transform3D::new(
    Basis::from_cols(
        Vector3::new(1.0, 2.0, 3.0),
        Vector3::new(4.0, 5.0, 6.0),
        Vector3::new(7.0, 8.0, -9.0),
    ),
    Vector3::new(10.0, 11.0, 12.0),
);

#[itest]
fn transfo