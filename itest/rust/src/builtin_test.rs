/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::itest;
use godot::builtin::inner::*;
use godot::prelude::*;

#[itest]
fn test_builtins_vector2() {
    let vec = Vector2::new(3.0, -4.0);
    