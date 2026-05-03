// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

#![no_std]

use qtty::Second;

pub fn build_samples() {
    let _samples = qtty::qtty_vec!(vec Second; 1.0, 2.0, 3.0);
}
