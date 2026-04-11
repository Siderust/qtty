// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Derived-unit example: ratios and direct `Unitless` from same-unit division.

use qtty::{Meter, Second, Unitless};

fn main() {
    // Same-unit division now directly yields Quantity<Unitless>.
    let half: Unitless = Meter::new(1.0) / Meter::new(2.0);
    assert!((half.value() - 0.5).abs() < 1e-12);

    // Typed angle from inverse trig
    let ratio: Unitless = Second::new(1.0) / Second::new(1.0);
    let angle: qtty::angular::Radians = ratio.asin_angle().to();
    assert!((angle.value() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);
}
