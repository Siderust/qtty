// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Derived-unit example: ratios and direct scalar from same-unit division.

use qtty::{Meter, Radian, Second};

fn main() {
    // Same-unit division now directly yields the raw scalar type.
    let half: f64 = Meter::new(1.0) / Meter::new(2.0);
    assert!((half - 0.5).abs() < 1e-12);

    // Typed angle from inverse trig on a raw ratio
    let ratio: f64 = Second::new(1.0) / Second::new(1.0);
    let angle: Radian = Radian::new(ratio.asin());
    assert!((angle.value() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);
}
