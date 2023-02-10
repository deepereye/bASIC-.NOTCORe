/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::itest;

use godot::prelude::{inner::InnerTransform2D, *};
use godot::private::class_macros::assert_eq_approx;

const TEST_TRANSFORM: Transform2D = Transform2D::from_cols(
    Vector2::new(1.0, 2.0),
    Vector2::new(3.0, 4.0),
    Vector2::new(5.0, 6.0),
);

#[itest]
fn transform2d_equiv() {
    let inner = InnerTransform2D::from_outer(&TEST_TRANSFORM);
    let outer = TEST_TRANSFORM;
    let vec = Vector2::new(1.0, 2.0);

    #[rustfmt::skip]
    let mappings_transform = [
        ("affine_inverse",   inner.affine_inverse(),                             outer.affine_inverse()                            ),
        ("orthonormalized",  inner.orthonormalized(),                            outer.orthonormalized()                           ),
        ("rotated",          in