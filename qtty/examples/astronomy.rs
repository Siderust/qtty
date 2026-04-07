// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Astronomy-flavored example using AU, light-years, and an orbital velocity estimate.

use qtty::velocity::Velocity;
use qtty::{AstronomicalUnit, Day, Kilometer, LightYear, Second};

fn main() {
    let earth_velocity: Velocity<qtty::unit::Kilometer, qtty::unit::Second> = Velocity::new(29.78);
    let time = Day::new(1.0);
    let time_sec: Second = time.to();
    let distance: Kilometer = (earth_velocity * time_sec).to();

    assert!((distance.value() - 2_573_395.2).abs() < 5_000.0);

    let proxima = LightYear::new(4.24);
    let au: AstronomicalUnit = proxima.to();
    assert!(au.value() > 200_000.0);
}
