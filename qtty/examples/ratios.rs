// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Derived-unit example: ratios and `Simplify` to recover `Unitless`.

use qtty::{Meter, Second, Simplify, Unitless};

fn main() {
    let half = Meter::new(1.0) / Meter::new(2.0);
    let unitless: Unitless = half.simplify();
    assert!((unitless.value() - 0.5).abs() < 1e-12);

    let ratio = Second::new(1.0) / Second::new(1.0);
    assert_eq!(ratio.asin(), core::f64::consts::FRAC_PI_2);
}
