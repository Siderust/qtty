// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Minimal end-to-end example: convert angles and compute a velocity (length / time).

use qtty::velocity::Velocity;
use qtty::{Degree, Kilometer, Second};

fn main() {
    let a = Degree::new(180.0);
    let r = a.to::<qtty::unit::Radian>();
    assert!((r.value() - core::f64::consts::PI).abs() < 1e-12);

    let d = Kilometer::new(1_000.0);
    let t = Second::new(100.0);
    let v: Velocity<qtty::unit::Kilometer, qtty::unit::Second> = d / t;
    assert!((v.value() - 10.0).abs() < 1e-12);
}
