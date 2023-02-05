
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::time::{Duration, Instant};

use godot::bind::{godot_api, GodotClass};
use godot::builtin::{ToVariant, Variant, VariantArray};
use godot::engine::Node;
use godot::obj::Gd;

use crate::{RustTestCase, TestContext};

#[derive(GodotClass, Debug)]
#[class(init)]
pub(crate) struct IntegrationTests {
    total: i64,
    passed: i64,
    skipped: i64,
    focus_run: bool,
}

#[godot_api]
impl IntegrationTests {
    #[allow(clippy::uninlined_format_args)]
    #[func]
    fn run_all_tests(
        &mut self,
        gdscript_tests: VariantArray,
        gdscript_file_count: i64,
        allow_focus: bool,
        scene_tree: Gd<Node>,
    ) -> bool {
        println!("{}Run{} Godot integration tests...", FMT_CYAN_BOLD, FMT_END);

        let (rust_tests, rust_file_count, focus_run) = super::collect_rust_tests();
        self.focus_run = focus_run;
        if focus_run {
            println!("  {FMT_CYAN}Focused run{FMT_END} -- execute only selected Rust tests.")
        }
        println!(
            "  Rust: found {} tests in {} files.",
            rust_tests.len(),
            rust_file_count
        );
        if !focus_run {
            println!(
                "  GDScript: found {} tests in {} files.",
                gdscript_tests.len(),
                gdscript_file_count
            );
        }

        let clock = Instant::now();
        self.run_rust_tests(rust_tests, scene_tree);
        let rust_time = clock.elapsed();
        let gdscript_time = if !focus_run {
            self.run_gdscript_tests(gdscript_tests);
            Some(clock.elapsed() - rust_time)
        } else {
            None
        };

        self.conclude(rust_time, gdscript_time, allow_focus)
    }

    fn run_rust_tests(&mut self, tests: Vec<RustTestCase>, scene_tree: Gd<Node>) {
        let ctx = TestContext { scene_tree };

        let mut last_file = None;
        for test in tests {
            let outcome = run_rust_test(&test, &ctx);