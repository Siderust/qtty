// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

// --- Astronomical mean units (explicitly approximate) ---

/// Mean sidereal day (Earth), expressed in SI seconds.
///
/// Convention used: `1 sidereal day ≈ 86_164.0905 s`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "sd", dimension = Time, ratio = 86_164.090_5)]
pub struct SiderealDay;
/// A quantity measured in sidereal days.
pub type SiderealDays = Quantity<SiderealDay>;
/// A constant representing one sidereal day.
pub const SIDEREAL_DAY: SiderealDays = SiderealDays::new(1.0);

/// Mean synodic month (lunar phase cycle), expressed in seconds.
///
/// Convention used: `1 synodic month ≈ 29.530590 d`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "synmo", dimension = Time, ratio = 29.530_590 * SECONDS_PER_DAY)]
pub struct SynodicMonth;
/// A quantity measured in synodic months.
pub type SynodicMonths = Quantity<SynodicMonth>;
/// A constant representing one synodic month.
pub const SYNODIC_MONTH: SynodicMonths = SynodicMonths::new(1.0);

/// Mean sidereal year (Earth), expressed in seconds.
///
/// Common convention: `1 sidereal year ≈ 365.256363004 d`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "syr", dimension = Time, ratio = 365.256_363_004 * SECONDS_PER_DAY)]
pub struct SiderealYear;
/// A quantity measured in sidereal years.
pub type SiderealYears = Quantity<SiderealYear>;
/// A constant representing one sidereal year.
pub const SIDEREAL_YEAR: SiderealYears = SiderealYears::new(1.0);

// ── Conversions: astro time ↔ base time ──────────────────────────────────────
crate::impl_unit_from_conversions_between!(
    Attosecond, Femtosecond, Picosecond, Nanosecond, Microsecond, Millisecond,
    Centisecond, Decisecond, Second, Decasecond, Hectosecond, Kilosecond,
    Megasecond, Gigasecond, Terasecond, Minute, Hour, Day, Week, Fortnight,
    Year, Decade, Century, Millennium;
    SiderealDay, SynodicMonth, SiderealYear
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Attosecond, Femtosecond, Picosecond, Nanosecond, Microsecond, Millisecond,
    Centisecond, Decisecond, Second, Decasecond, Hectosecond, Kilosecond,
    Megasecond, Gigasecond, Terasecond, Minute, Hour, Day, Week, Fortnight,
    Year, Decade, Century, Millennium;
    SiderealDay, SynodicMonth, SiderealYear
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! time_astro_units {
    ($cb:path) => {
        $cb!(SiderealDay, SynodicMonth, SiderealYear,);
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn sidereal_day_to_seconds() {
        let day = SiderealDays::new(1.0);
        let seconds: Seconds = day.to();
        assert_abs_diff_eq!(seconds.value(), 86_164.090_5, epsilon = 1e-9);
    }

    #[test]
    fn synodic_month_to_days() {
        let month = SynodicMonths::new(1.0);
        let days: Days = month.to();
        assert_abs_diff_eq!(days.value(), 29.530_590, epsilon = 1e-12);
    }

    proptest! {
        #[test]
        fn sidereal_year_second_roundtrip(v in -1.0e6_f64..1.0e6_f64) {
            let years = SiderealYears::new(v);
            let roundtrip: SiderealYears = years.to::<Second>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
