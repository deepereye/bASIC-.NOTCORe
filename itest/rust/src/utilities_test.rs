/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::itest;

use godot::builtin::Variant;
use godot::engine::utilities::*;

#[itest]
fn utilities_abs() {
    let input = Variant::from(-7);
    let output = abs(input);

    assert_eq!(output, Variant::from(7));
}

#[itest]
fn utilities_sign() {
    let input = Variant::from(-7);
    let output = sign(input);

    asse