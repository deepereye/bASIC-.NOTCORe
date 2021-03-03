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
        tolerance = CMP_EPSILON;
    }
    (a - b).abs() < tolerance
}

/// Check if two angles are approximately equal, by comparing the distance
/// between the points on the unit circle with 0 using [`is_equal_approx`].
pub fn is_angle_equal_approx(a: real, b: real) -> bool {
    let (x1, y1) = a.sin_cos();
    let (x2, y2) = b.sin_cos();

    println!("({x1}, {y1}) ({x2}, {y2})");

    is_equal_approx(
        Vector2::distance_to(Vector2::new(x1, y1), Vector2::new(x2, y2)),
        0.0,
    )
}

pub fn is_zero_approx(s: real) -> bool {
    s.abs() < CMP_EPSILON
}

pub fn fposmod(x: real, y: real) -> real {
    let mut value = x % y;
    if ((value < 0.0) && (y > 0.0)) || ((value > 0.0) && (y < 0.0)) {
        value += y;
    }
    value += 0.0;
    value
}

pub fn snapped(mut value: real, step: real) -> real {
    if step != 0.0 {
        value = ((value / step + 0.5) * step).floor()
    }
    value
}

pub fn sign(value: real) -> real {
    if value == 0.0 {
        0.0
    } else if value < 0.0 {
        -1.0
    } else {
        1.0
    }
}

pub fn bezier_derivative(
    start: real,
    control_1: real,
    control_2: real,
    end: real,
    t: real,
) -> real {
    let omt = 1.0 - t;
    let omt2 = omt * omt;
    let t2 = t * t;
    (control_1 - start) * 3.0 * omt2
        + (control_2 - control_1) * 6.0 * omt * t
        + (end - control_2) * 3.0 * t2
}

pub fn bezier_interpolate(
    start: real,
    control_1: real,
    control_2: real,
    end: real,
    t: real,
) -> real {
    let omt = 1.0 - t;
    let omt2 = omt * omt;
    let omt3 = omt2 * omt;
    let t2 = t * t;
    let t3 = t2 * t;
    start * omt3 + control_1 * omt2 * t * 3.0 + control_2 * omt * t2 * 3.0 + end * t3
}

pub fn cubic_interpolate(from: real, to: real, pre: real