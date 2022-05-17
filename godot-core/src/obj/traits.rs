
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::obj::Base;

use godot_ffi as sys;

/// Makes `T` eligible to be managed by Godot and stored in [`Gd<T>`][crate::obj::Gd] pointers.
///
/// The behavior of types implementing this trait is influenced by the associated types; check their documentation for information.
pub trait GodotClass: 'static
where
    Self: Sized,
{
    /// The immediate superclass of `T`. This is always a Godot engine class.
    type Base: GodotClass; // not EngineClass because it can be ()

    /// Whether this class is a core Godot class provided by the engine, or declared by the user as a Rust struct.
    // TODO what about GDScript user classes?
    type Declarer: dom::Domain;

    /// Defines the memory strategy.
    type Mem: mem::Memory;

    /// The name of the class, under which it is registered in Godot.
    ///
    /// This may deviate from the Rust struct name: `HttpRequest::CLASS_NAME == "HTTPRequest"`.
    const CLASS_NAME: &'static str;
}

/// Unit impl only exists to represent "no base", and is used for exactly one class: `Object`.
impl GodotClass for () {
    type Base = ();
    type Declarer = dom::EngineDomain;
    type Mem = mem::ManualMemory;

    const CLASS_NAME: &'static str = "(no base)";
}

/// Trait to create more references from a smart pointer or collection.
pub trait Share {
    /// Creates a new reference that points to the same object.
    ///
    /// If the referred-to object is reference-counted, this will increment the count.
    fn share(&self) -> Self;
}

/// Non-strict inheritance relationship in the Godot class hierarchy.
///
/// `Derived: Inherits<Base>` means that either `Derived` is a subclass of `Base`, or the class `Base` itself (hence "non-strict").
///
/// This trait is automatically implemented for all Godot engine classes and user-defined classes that derive [`GodotClass`].
/// It has `GodotClass` as a supertrait, allowing your code to have bounds solely on `Derived: Inherits<Base>` rather than
/// `Derived: Inherits<Base> + GodotClass`.
///
/// Inheritance is transitive across indirect base classes: `Node3D` implements `Inherits<Node>` and `Inherits<Object>`.
///
/// The trait is also reflexive: `T` always implements `Inherits<T>`.
///
/// # Usage
///
/// The primary use case for this trait is polymorphism: you write a function that accepts anything that derives from a certain class
/// (including the class itself):
/// ```no_run
/// # use godot::prelude::*;
/// fn print_node<T>(node: Gd<T>)
/// where
///     T: Inherits<Node>,
/// {
///     let up = node.upcast(); // type Gd<Node> inferred
///     println!("Node #{} with name {}", up.instance_id(), up.get_name());
///     up.free();
/// }
///
/// // Call with different types
/// print_node(Node::new_alloc());   // works on T=Node as well
/// print_node(Node2D::new_alloc()); // or derived classes
/// print_node(Node3D::new_alloc());
/// ```
///
/// A variation of the above pattern works without `Inherits` or generics, if you move the `upcast()` into the call site:
/// ```no_run
/// # use godot::prelude::*;
/// fn print_node(node: Gd<Node>) { /* ... */ }
///
/// // Call with different types
/// print_node(Node::new_alloc());            // no upcast needed
/// print_node(Node2D::new_alloc().upcast());
/// print_node(Node3D::new_alloc().upcast());
/// ```
///
pub trait Inherits<Base>: GodotClass {}

impl<T: GodotClass> Inherits<T> for T {}

/// Auto-implemented for all engine-provided classes
pub trait EngineClass: GodotClass {
    fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr;
    fn as_type_ptr(&self) -> sys::GDExtensionTypePtr;
}

/// Auto-implemented for all engine-provided enums
pub trait EngineEnum: Copy {
    fn try_from_ord(ord: i32) -> Option<Self>;

    /// Ordinal value of the enumerator, as specified in Godot.
    /// This is not necessarily unique.
    fn ord(self) -> i32;

    fn from_ord(ord: i32) -> Self {
        Self::try_from_ord(ord)
            .unwrap_or_else(|| panic!("ordinal {ord} does not map to any enumerator"))
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Capability traits, providing dedicated functionalities for Godot classes
pub mod cap {
    use super::*;

    /// Trait for all classes that are constructible from the Godot engine.
    ///
    /// Godot can only construct user-provided classes in one way: with the default
    /// constructor. This is what happens when you write `MyClass.new()` in GDScript.
    /// You can disable this constructor by not providing an `init` method for your
    /// class; in that case construction fails.
    ///
    /// This trait is not manually implemented, and you cannot call any methods.
    /// Instead, the trait will be provided to you by the proc macros, and you can
    /// use it as a bound.
    pub trait GodotInit: GodotClass {
        #[doc(hidden)]
        fn __godot_init(base: Base<Self::Base>) -> Self;
    }

    /// Auto-implemented for `#[godot_api] impl MyClass` blocks
    pub trait ImplementsGodotApi: GodotClass {
        #[doc(hidden)]
        fn __register_methods();
    }

    pub trait ImplementsGodotExports: GodotClass {
        #[doc(hidden)]
        fn __register_exports();
    }

    /// Auto-implemented for `#[godot_api] impl GodotExt for MyClass` blocks
    pub trait ImplementsGodotExt: GodotClass {
        #[doc(hidden)]
        fn __virtual_call(_name: &str) -> sys::GDExtensionClassCallVirtual;
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Domain + Memory classifiers

mod private {
    pub trait Sealed {}