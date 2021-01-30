
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// TODO this is experimental -- do not refactor existing types to this yet
// Need to see if ergonomics are worth the generic complexity.
//
// Nice:
//   self.glam2(&with, |a, b| a.dot(b))
//   self.glam2(&with, glam::f32::Quat::dot)
//
// Alternative with only conversions:
//   self.glam().dot(b.glam())
//   GlamType::dot(self.glam(), b.glam())

use super::real;

pub(crate) trait GlamConv {
    type Glam: GlamType<Mapped = Self>;

    fn to_glam(&self) -> Self::Glam {
        Self::Glam::from_front(self)
    }

    fn glam<F, R>(&self, unary_fn: F) -> R::Mapped
    where
        R: GlamType,
        F: FnOnce(Self::Glam) -> R,
    {
        let arg = Self::Glam::from_front(self);
        let result = unary_fn(arg);

        result.to_front()