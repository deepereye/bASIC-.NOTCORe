
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod api_parser;
mod central_generator;
mod class_generator;
mod context;
mod godot_exe;
mod godot_version;
mod special_cases;
mod util;
mod utilities_generator;
mod watch;

#[cfg(test)]
mod tests;

use api_parser::{load_extension_api, ExtensionApi};
use central_generator::{
    generate_core_central_file, generate_core_mod_file, generate_sys_central_file,
    generate_sys_mod_file,
};
use class_generator::{generate_builtin_class_files, generate_class_files};
use context::Context;
use util::ident;
use utilities_generator::generate_utilities_file;
use watch::StopWatch;

use crate::util::{to_pascal_case, to_snake_case};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::path::{Path, PathBuf};

pub fn generate_sys_files(sys_gen_path: &Path) {
    let mut out_files = vec![];
    let mut watch = StopWatch::start();

    generate_sys_mod_file(sys_gen_path, &mut out_files);

    let (api, build_config) = load_extension_api(&mut watch);
    let mut ctx = Context::build_from_api(&api);
    watch.record("build_context");

    generate_sys_central_file(&api, &mut ctx, build_config, sys_gen_path, &mut out_files);
    watch.record("generate_central_file");

    rustfmt_if_needed(out_files);
    watch.record("rustfmt");
    watch.write_stats_to(&sys_gen_path.join("codegen-stats.txt"));
}

pub fn generate_core_files(core_gen_path: &Path) {
    let mut out_files = vec![];
    let mut watch = StopWatch::start();

    generate_core_mod_file(core_gen_path, &mut out_files);

    let (api, build_config) = load_extension_api(&mut watch);
    let mut ctx = Context::build_from_api(&api);
    watch.record("build_context");

    generate_core_central_file(&api, &mut ctx, build_config, core_gen_path, &mut out_files);
    watch.record("generate_central_file");

    generate_utilities_file(&api, &mut ctx, core_gen_path, &mut out_files);
    watch.record("generate_utilities_file");

    // Class files -- currently output in godot-core; could maybe be separated cleaner
    // Note: deletes entire generated directory!
    generate_class_files(
        &api,
        &mut ctx,
        build_config,
        &core_gen_path.join("classes"),
        &mut out_files,
    );
    watch.record("generate_class_files");

    generate_builtin_class_files(
        &api,
        &mut ctx,
        build_config,
        &core_gen_path.join("builtin_classes"),
        &mut out_files,
    );
    watch.record("generate_builtin_class_files");

    rustfmt_if_needed(out_files);
    watch.record("rustfmt");
    watch.write_stats_to(&core_gen_path.join("codegen-stats.txt"));
}

// #[cfg(feature = "codegen-fmt")]
fn rustfmt_if_needed(out_files: Vec<PathBuf>) {
    println!("Format {} generated files...", out_files.len());

    for files in out_files.chunks(20) {
        let mut process = std::process::Command::new("rustfmt");
        process.arg("--edition=2021");

        println!("  Format {} files...", files.len());
        for file in files {
            process.arg(file);
        }

        process
            .output()
            .unwrap_or_else(|err| panic!("during godot-rust codegen, rustfmt failed:\n   {err}"));
    }

    println!("Rustfmt completed.");
}
//
// #[cfg(not(feature = "codegen-fmt"))]
// fn rustfmt_if_needed(_out_files: Vec<PathBuf>) {}

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Shared utility types

#[derive(Clone)]
enum RustTy {
    /// `bool`, `Vector3i`
    BuiltinIdent(Ident),

    /// `TypedArray<i32>`
    BuiltinArray(TokenStream),

    /// `TypedArray<Gd<PhysicsBody3D>>`
    EngineArray {
        tokens: TokenStream,
        #[allow(dead_code)] // only read in minimal config
        elem_class: String,
    },

    /// `module::Enum`
    EngineEnum {
        tokens: TokenStream,
        /// `None` for globals
        #[allow(dead_code)] // only read in minimal config
        surrounding_class: Option<String>,
    },

    /// `Gd<Node>`
    EngineClass {
        tokens: TokenStream,
        #[allow(dead_code)] // currently not read
        class: String,
    },
}

impl RustTy {
    pub fn return_decl(&self) -> TokenStream {
        match self {
            Self::EngineClass { tokens, .. } => quote! { -> Option<#tokens> },
            other => quote! { -> #other },
        }
    }
}

impl ToTokens for RustTy {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RustTy::BuiltinIdent(ident) => ident.to_tokens(tokens),
            RustTy::BuiltinArray(path) => path.to_tokens(tokens),
            RustTy::EngineArray { tokens: path, .. } => path.to_tokens(tokens),
            RustTy::EngineEnum { tokens: path, .. } => path.to_tokens(tokens),
            RustTy::EngineClass { tokens: path, .. } => path.to_tokens(tokens),
            //RustTy::Other(path) => path.to_tokens(tokens),
        }
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Contains multiple naming conventions for types (classes, builtin classes, enums).
#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct TyName {
    godot_ty: String,
    rust_ty: Ident,
}

impl TyName {
    fn from_godot(godot_ty: &str) -> Self {
        Self {
            godot_ty: godot_ty.to_owned(),
            rust_ty: ident(&to_pascal_case(godot_ty)),
        }
    }

    fn description(&self) -> String {
        if self.rust_ty == self.godot_ty {
            self.godot_ty.clone()
        } else {
            format!("{}  [renamed {}]", self.godot_ty, self.rust_ty)
        }
    }
}

impl ToTokens for TyName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.rust_ty.to_tokens(tokens)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Contains naming conventions for modules.
pub(crate) struct ModName {
    // godot_mod: String,
    rust_mod: Ident,
}

impl ModName {
    fn from_godot(godot_ty: &str) -> Self {
        Self {
            // godot_mod: godot_ty.to_owned(),
            rust_mod: ident(&to_snake_case(godot_ty)),
        }
    }
}

impl ToTokens for ModName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.rust_mod.to_tokens(tokens)
    }
}

struct GeneratedClass {
    tokens: TokenStream,
    inherits_macro_ident: Ident,
    has_pub_module: bool,
}

struct GeneratedBuiltin {
    tokens: TokenStream,
}

struct GeneratedClassModule {
    class_name: TyName,
    module_name: ModName,
    inherits_macro_ident: Ident,
    is_pub: bool,
}

struct GeneratedBuiltinModule {
    class_name: TyName,
    module_name: ModName,
}

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Shared config

// Classes for minimal config
#[cfg(not(feature = "codegen-full"))]
const SELECTED_CLASSES: &[&str] = &[
    "AnimatedSprite2D",
    "Area2D",
    "AudioStreamPlayer",
    "BaseButton",
    "Button",
    "Camera2D",
    "Camera3D",
    "CanvasItem",
    "CanvasLayer",
    "CollisionObject2D",
    "CollisionShape2D",
    "Control",
    "FileAccess",
    "HTTPRequest",
    "Image",
    "ImageTextureLayered",
    "Input",
    "Label",
    "MainLoop",
    "Marker2D",
    "Node",
    "Node2D",
    "Node3D",
    "Node3DGizmo",
    "Object",
    "OS",
    "PackedScene",
    "PathFollow2D",
    "PhysicsBody2D",
    "RefCounted",
    "Resource",
    "ResourceLoader",
    "RigidBody2D",
    "SceneTree",
    "Sprite2D",
    "SpriteFrames",
    "Texture",
    "Texture2DArray",
    "TextureLayered",
    "Time",
    "Timer",
];