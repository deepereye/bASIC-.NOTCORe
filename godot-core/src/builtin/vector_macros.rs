
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![macro_use]

/// Implements a single unary operator for a vector type. Only used for `Neg` at the moment.
macro_rules! impl_vector_unary_operator {
    (
        // Name of the vector type.
        $Vector:ty,
        // Type of each individual component, for example `i32`.
        $Scalar:ty,
        // Names of the components, with parentheses, for example `(x, y)`.
        ($($components:ident),*),
        // Name of the operator trait, for example `Neg`.
        $Operator:ident,
        // Name of the function on the operator trait, for example `neg`.
        $func:ident
    ) => {
        impl std::ops::$Operator for $Vector {
            type Output = Self;
            fn $func(mut self) -> Self::Output {
                $(
                    self.$components = self.$components.$func();
                )*
                self
            }
        }
    }
}

/// Implements a component-wise single infix binary operator between two vectors.
macro_rules! impl_vector_vector_binary_operator {
    (
        // Name of the vector type.
        $Vector:ty,
        // Type of each individual component, for example `i32`.
        $Scalar:ty,
        // Names of the components, with parentheses, for example `(x, y)`.
        ($($components:ident),*),
        // Name of the operator trait, for example `Add`.
        $Operator:ident,
        // Name of the function on the operator trait, for example `add`.
        $func:ident
    ) => {
        impl std::ops::$Operator for $Vector {
            type Output = Self;
            fn $func(mut self, rhs: $Vector) -> Self::Output {
                $(
                    self.$components = self.$components.$func(rhs.$components);
                )*
                self
            }
        }
    }
}

/// Implements a component-wise single infix binary operator between a vector on the left and a
/// scalar on the right-hand side.
macro_rules! impl_vector_scalar_binary_operator {
    (