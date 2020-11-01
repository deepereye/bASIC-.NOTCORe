/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::{Duration, Instant};

use std::io::Write;

pub struct StopWatch {
    last_instant: Instant,
    metrics: Vec<Metric>,
    lwidth: usize,
}

impl StopWatch {
    pub fn start() -> Self {
        Self {
            last_instant: Instant::now(),
            metrics