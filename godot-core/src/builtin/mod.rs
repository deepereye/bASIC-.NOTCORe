/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Built-in types like `Vector2`, `GodotString` and `Variant`.
//!
//! # Background on the design of vector algebra types
//!
//! The basic vector algebra types like `Vector2`, `Matrix4` and `Quaternion` are re-implemented
//! here, with an API similar to that in the Godot engine itself. There are other approaches, but
//! they all have their disadvantages:
//!
//! - We could invoke API methods from the engine. The implementations could be generated, but it
//!   is slower and prevents inlining.
//!
//! - We could re-export types from an existing vector algebra crate, like `glam`. This removes the
//!   duplication, but it would create a strong dependency on a volatile API outside our control.
//!   The `gdnative` crate started out this way, using types from `euclid`, but [found it
//!   impractical](https://github.com/godot-rust/gdnative/issues/594#issue-705061720). Moreover,
//!   the API would not match Godot's own, which would make porting from GDScript 