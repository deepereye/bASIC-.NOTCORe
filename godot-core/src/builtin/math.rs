/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use super::real_consts::TAU;

use super::real;
use super::Vector2;

pub const CMP_EPSILON: real = 0.00001;

pub fn lerp(a: real, b: real, t: real) -> real {
    a + ((b - a) * t)
}

pub fn is_equal_approx(a: real, b: real) -> bool {
    if a == b {
        return true;
    }
    let mut tolerance = CMP_EPSILON * a.abs();
    if tolerance < CMP_EPSILON {
        tolerance = CM