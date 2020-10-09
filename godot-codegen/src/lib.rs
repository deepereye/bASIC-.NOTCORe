
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