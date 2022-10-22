
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::util;
use crate::util::bail;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use venial::{AttributeValue, Declaration, Error, Function, Impl, ImplMember};

// Note: keep in sync with trait GodotExt
const VIRTUAL_METHOD_NAMES: [&str; 3] = ["ready", "process", "physics_process"];

pub fn transform(input_decl: Declaration) -> Result<TokenStream, Error> {
    let decl = match input_decl {
        Declaration::Impl(decl) => decl,
        _ => bail(
            "#[godot_api] can only be applied on impl blocks",
            input_decl,
        )?,
    };

    if decl.impl_generic_params.is_some() {
        bail(
            "#[godot_api] currently does not support generic parameters",
            &decl,
        )?;
    }

    if decl.self_ty.as_path().is_none() {
        return bail("invalid Self type for #[godot_api] impl", decl);
    };

    if decl.trait_ty.is_some() {
        transform_trait_impl(decl)
    } else {
        transform_inherent_impl(decl)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Attribute for user-declared function
enum BoundAttrType {
    Func(AttributeValue),
    Signal(AttributeValue),
}

struct BoundAttr {
    attr_name: Ident,
    index: usize,
    ty: BoundAttrType,
}

impl BoundAttr {
    fn bail<R>(self, msg: &str, method: &Function) -> Result<R, Error> {
        bail(format!("#[{}]: {}", self.attr_name, msg), &method.name)
    }
}

/// Codegen for `#[godot_api] impl MyType`
fn transform_inherent_impl(mut decl: Impl) -> Result<TokenStream, Error> {
    let class_name = util::validate_impl(&decl, None, "godot_api")?;
    let class_name_str = class_name.to_string();

    //let register_fn = format_ident!("__godot_rust_register_{}", class_name_str);
    //#[allow(non_snake_case)]

    let (funcs, signals) = process_godot_fns(&mut decl)?;
    let signal_name_strs = signals.into_iter().map(|ident| ident.to_string());

    let prv = quote! { ::godot::private };

    let result = quote! {
        #decl

        impl ::godot::obj::cap::ImplementsGodotApi for #class_name {
            //fn __register_methods(_builder: &mut ::godot::builder::ClassBuilder<Self>) {
            fn __register_methods() {
                #(
                    ::godot::private::gdext_register_method!(#class_name, #funcs);
                )*

                unsafe {
                    let class_name = ::godot::builtin::StringName::from(#class_name_str);
                    use ::godot::sys;
                    #(
                        let signal_name = ::godot::builtin::StringName::from(#signal_name_strs);
                        sys::interface_fn!(classdb_register_extension_class_signal)(
                            sys::get_library(),
                            class_name.string_sys(),
                            signal_name.string_sys(),
                            std::ptr::null(), // NULL only valid for zero parameters, in current impl; maybe better empty slice
                            0,
                        );
                    )*
                }
            }
        }

        ::godot::sys::plugin_add!(__GODOT_PLUGIN_REGISTRY in #prv; #prv::ClassPlugin {
            class_name: #class_name_str,
            component: #prv::PluginComponent::UserMethodBinds {
                generated_register_fn: #prv::ErasedRegisterFn {
                    raw: #prv::callbacks::register_user_binds::<#class_name>,
                },
            },
        });
    };

    Ok(result)
}

fn process_godot_fns(decl: &mut Impl) -> Result<(Vec<Function>, Vec<Ident>), Error> {
    let mut func_signatures = vec![];
    let mut signal_idents = vec![]; // TODO consider signature

    let mut removed_indexes = vec![];
    for (index, item) in decl.body_items.iter_mut().enumerate() {
        let method = if let ImplMember::Method(method) = item {
            method
        } else {
            continue;
        };

        if let Some(attr) = extract_attributes(method)? {
            // Remaining code no longer has attribute -- rest stays
            method.attributes.remove(attr.index);

            if method.qualifiers.tk_default.is_some()
                || method.qualifiers.tk_const.is_some()
                || method.qualifiers.tk_async.is_some()
                || method.qualifiers.tk_unsafe.is_some()
                || method.qualifiers.tk_extern.is_some()
                || method.qualifiers.extern_abi.is_some()
            {
                return attr.bail("fn qualifiers are not allowed", method);
            }

            if method.generic_params.is_some() {
                return attr.bail("generic fn parameters are not supported", method);
            }

            match attr.ty {
                BoundAttrType::Func(_attr) => {
                    // Signatures are the same thing without body
                    let sig = util::reduce_to_signature(method);
                    func_signatures.push(sig);
                }
                BoundAttrType::Signal(ref _attr_val) => {
                    if !method.params.is_empty() || method.return_ty.is_some() {
                        return attr.bail("parameters and return types not yet supported", method);
                    }

                    signal_idents.push(method.name.clone());
                    removed_indexes.push(index);
                }
            }
        }
    }

    // Remove some elements (e.g. signals) from impl.
    // O(n^2); alternative: retain(), but elements themselves don't have the necessary information.
    for index in removed_indexes.into_iter().rev() {
        decl.body_items.remove(index);
    }

    Ok((func_signatures, signal_idents))
}

fn extract_attributes(method: &Function) -> Result<Option<BoundAttr>, Error> {
    let mut found = None;
    for (index, attr) in method.attributes.iter().enumerate() {
        let attr_name = attr
            .get_single_path_segment()
            .expect("get_single_path_segment");

        // Note: can't use match without constructing new string, because ident
        let new_found = if attr_name == "func" {
            Some(BoundAttr {
                attr_name: attr_name.clone(),
                index,
                ty: BoundAttrType::Func(attr.value.clone()),
            })
        } else if attr_name == "signal" {
            // TODO once parameters are supported, this should probably be moved to the struct definition
            // E.g. a zero-sized type Signal<(i32, String)> with a provided emit(i32, String) method
            // This could even be made public (callable on the struct obj itself)
            Some(BoundAttr {
                attr_name: attr_name.clone(),
                index,
                ty: BoundAttrType::Signal(attr.value.clone()),
            })
        } else {
            None
        };

        // Validate at most 1 attribute
        if found.is_some() && new_found.is_some() {
            bail(
                "at most one #[func] or #[signal] attribute per method allowed",
                &method.name,
            )?;
        }

        found = new_found;
    }

    Ok(found)
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Codegen for `#[godot_api] impl GodotExt for MyType`
fn transform_trait_impl(original_impl: Impl) -> Result<TokenStream, Error> {
    let class_name = util::validate_impl(&original_impl, Some("GodotExt"), "godot_api")?;
    let class_name_str = class_name.to_string();

    let mut godot_init_impl = TokenStream::new();
    let mut register_fn = quote! { None };
    let mut create_fn = quote! { None };
    let mut to_string_fn = quote! { None };
    let mut virtual_methods = vec![];
    let mut virtual_method_names = vec![];

    let prv = quote! { ::godot::private };

    for item in original_impl.body_items.iter() {
        let method = if let ImplMember::Method(f) = item {
            f
        } else {
            continue;
        };

        let method_name = method.name.to_string();
        match method_name.as_str() {
            "register_class" => {
                register_fn = quote! { Some(#prv::ErasedRegisterFn {
                    raw: #prv::callbacks::register_class_by_builder::<#class_name>
                }) };
            }

            "init" => {
                godot_init_impl = quote! {
                    impl ::godot::obj::cap::GodotInit for #class_name {
                        fn __godot_init(base: ::godot::obj::Base<Self::Base>) -> Self {
                            <Self as ::godot::bind::GodotExt>::init(base)
                        }
                    }
                };
                create_fn = quote! { Some(#prv::callbacks::create::<#class_name>) };
            }

            "to_string" => {
                to_string_fn = quote! { Some(#prv::callbacks::to_string::<#class_name>) };
            }

            // Other virtual methods, like ready, process etc.
            known_name if VIRTUAL_METHOD_NAMES.contains(&known_name) => {
                let method = util::reduce_to_signature(method);

                // Godot-facing name begins with underscore
                virtual_method_names.push(format!("_{method_name}"));
                virtual_methods.push(method);
            }

            // Unknown methods which are declared inside trait impl are not supported (possibly compiler catches those first anyway)
            other_name => {
                return bail(
                    format!("Unsupported GodotExt method: {other_name}"),
                    &method.name,
                )
            }
        }
    }

    let result = quote! {
        #original_impl
        #godot_init_impl

        impl ::godot::private::You_forgot_the_attribute__godot_api for #class_name {}

        impl ::godot::obj::cap::ImplementsGodotExt for #class_name {
            fn __virtual_call(name: &str) -> ::godot::sys::GDExtensionClassCallVirtual {
                //println!("virtual_call: {}.{}", std::any::type_name::<Self>(), name);

                match name {
                    #(
                       #virtual_method_names => #prv::gdext_virtual_method_callback!(#class_name, #virtual_methods),
                    )*
                    _ => None,
                }
            }
        }

        ::godot::sys::plugin_add!(__GODOT_PLUGIN_REGISTRY in #prv; #prv::ClassPlugin {
            class_name: #class_name_str,
            component: #prv::PluginComponent::UserVirtuals {
                user_register_fn: #register_fn,
                user_create_fn: #create_fn,
                user_to_string_fn: #to_string_fn,
                get_virtual_fn: #prv::callbacks::get_virtual::<#class_name>,
            },
        });
    };

    Ok(result)
}