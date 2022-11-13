
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::itest;
use godot::prelude::{inner::InnerBasis, *};
use godot::private::class_macros::assert_eq_approx;

const TEST_BASIS: Basis = Basis::from_rows(
    Vector3::new(0.942155, -0.270682, 0.197677),
    Vector3::new(0.294044, 0.950564, -0.099833),
    Vector3::new(-0.160881, 0.152184, 0.97517),
);

#[itest]
fn basis_multiply_same() {
    let rust_res = TEST_BASIS * Basis::IDENTITY;
    let godot_res = TEST_BASIS
        .to_variant()
        .evaluate(&Basis::IDENTITY.to_variant(), VariantOperator::Multiply)
        .unwrap()
        .to::<Basis>();
    assert_eq_approx!(rust_res, godot_res, |a, b| Basis::is_equal_approx(&a, &b));

    let rhs = Basis::from_axis_angle(Vector3::new(1.0, 2.0, 3.0).normalized(), 0.5);
    let rust_res = TEST_BASIS * rhs;
    let godot_res = TEST_BASIS
        .to_variant()
        .evaluate(&rhs.to_variant(), VariantOperator::Multiply)
        .unwrap()
        .to::<Basis>();
    assert_eq_approx!(rust_res, godot_res, |a, b| Basis::is_equal_approx(&a, &b));
}

#[itest]
fn basis_euler_angles_same() {
    let euler_order_to_test: Vec<EulerOrder> = vec![
        EulerOrder::XYZ,
        EulerOrder::XZY,
        EulerOrder::YZX,
        EulerOrder::YXZ,
        EulerOrder::ZXY,
        EulerOrder::ZYX,
    ];

    let vectors_to_test: Vec<Vector3> = vec![
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(-0.5, -0.5, -0.5),
        Vector3::new(40.0, 40.0, 40.0),
        Vector3::new(-40.0, -40.0, -40.0),
        Vector3::new(0.0, 0.0, -90.0),
        Vector3::new(0.0, -90.0, 0.0),
        Vector3::new(-90.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 90.0),
        Vector3::new(0.0, 90.0, 0.0),
        Vector3::new(90.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -30.0),
        Vector3::new(0.0, -30.0, 0.0),
        Vector3::new(-30.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 30.0),
        Vector3::new(0.0, 30.0, 0.0),
        Vector3::new(30.0, 0.0, 0.0),
        Vector3::new(0.5, 50.0, 20.0),
        Vector3::new(-0.5, -50.0, -20.0),
        Vector3::new(0.5, 0.0, 90.0),
        Vector3::new(0.5, 0.0, -90.0),
        Vector3::new(360.0, 360.0, 360.0),
        Vector3::new(-360.0, -360.0, -360.0),
        Vector3::new(-90.0, 60.0, -90.0),
        Vector3::new(90.0, 60.0, -90.0),
        Vector3::new(90.0, -60.0, -90.0),
        Vector3::new(-90.0, -60.0, -90.0),
        Vector3::new(-90.0, 60.0, 90.0),
        Vector3::new(90.0, 60.0, 90.0),
        Vector3::new(90.0, -60.0, 90.0),
        Vector3::new(-90.0, -60.0, 90.0),
        Vector3::new(60.0, 90.0, -40.0),
        Vector3::new(60.0, -90.0, -40.0),
        Vector3::new(-60.0, -90.0, -40.0),
        Vector3::new(-60.0, 90.0, 40.0),
        Vector3::new(60.0, 90.0, 40.0),
        Vector3::new(60.0, -90.0, 40.0),
        Vector3::new(-60.0, -90.0, 40.0),
        Vector3::new(-90.0, 90.0, -90.0),
        Vector3::new(90.0, 90.0, -90.0),
        Vector3::new(90.0, -90.0, -90.0),
        Vector3::new(-90.0, -90.0, -90.0),
        Vector3::new(-90.0, 90.0, 90.0),
        Vector3::new(90.0, 90.0, 90.0),