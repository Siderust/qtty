// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Astronomical velocity constants and units.
//!
//! This module provides velocity-related constants derived from astronomical definitions,
//! such as the speed of light in AU/day.

use super::Velocity;
use crate::units::length::{AstronomicalUnit, LightYear, Meter};
use crate::units::time::{Day, JulianYear, Second};
use crate::Unit;

/// Exact speed of light in vacuum (`299_792_458 m/s`).
pub const C: Velocity<Meter, Second> = Velocity::<Meter, Second>::new(299_792_458.0);

/// Speed of light in AU/day derived from the canonical `LightYear` and `AstronomicalUnit`
/// definitions in this crate.
///
/// A light-year is defined as the distance light travels in one Julian year.
/// Thus: `c (AU/day) = LightYear::RATIO / AstronomicalUnit::RATIO / (JulianYear::RATIO / Day::RATIO)`.
pub const AU_PER_DAY_C: Velocity<AstronomicalUnit, Day> =
    Velocity::new(LightYear::RATIO / AstronomicalUnit::RATIO / (JulianYear::RATIO / Day::RATIO));

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::units::length::{Au, Kilometer};
    use crate::units::time::Second;
    use approx::assert_relative_eq;

    #[test]
    fn au_per_day_to_km_per_s() {
        let v: Velocity<Au, Day> = Velocity::new(1.0);
        let v_kps: Velocity<Kilometer, Second> = v.to();
        // 1 AU/day = 149,597,870.7 km / 86400 s ≈ 1731.5 km/s
        assert_relative_eq!(v_kps.value(), 1731.5, max_relative = 1e-3);
    }
}
