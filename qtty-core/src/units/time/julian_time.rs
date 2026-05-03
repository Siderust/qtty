// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

// --- Julian conventions (useful in astronomy/ephemerides) ---

/// Julian year (`365.25 d`), expressed in seconds.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "a", dimension = Time, ratio = 365.25 * SECONDS_PER_DAY)]
pub struct JulianYear;
/// A quantity measured in Julian years.
pub type JulianYears = Quantity<JulianYear>;
/// A constant representing one Julian year.
pub const JULIAN_YEAR: JulianYears = JulianYears::new(1.0);

/// Julian century (`36_525 d`), expressed in seconds.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "JC", dimension = Time, ratio = 36_525.0 * SECONDS_PER_DAY)]
pub struct JulianCentury;
/// A quantity measured in Julian centuries.
pub type JulianCenturies = Quantity<JulianCentury>;
/// A constant representing one Julian century.
pub const JULIAN_CENTURY: JulianCenturies = JulianCenturies::new(1.0);

// ── Conversions: julian time ↔ base time ─────────────────────────────────────
crate::impl_unit_from_conversions_between!(
    Attosecond, Femtosecond, Picosecond, Nanosecond, Microsecond, Millisecond,
    Centisecond, Decisecond, Second, Decasecond, Hectosecond, Kilosecond,
    Megasecond, Gigasecond, Terasecond, Minute, Hour, Day, Week, Fortnight,
    Year, Decade, Century, Millennium;
    JulianYear, JulianCentury
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Attosecond, Femtosecond, Picosecond, Nanosecond, Microsecond, Millisecond,
    Centisecond, Decisecond, Second, Decasecond, Hectosecond, Kilosecond,
    Megasecond, Gigasecond, Terasecond, Minute, Hour, Day, Week, Fortnight,
    Year, Decade, Century, Millennium;
    JulianYear, JulianCentury
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! time_julian_time_units {
    ($cb:path) => {
        $cb!(JulianYear, JulianCentury,);
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn julian_year_to_days() {
        let year = JulianYears::new(1.0);
        let days: Days = year.to();
        assert_abs_diff_eq!(days.value(), 365.25, epsilon = 1e-12);
    }

    #[test]
    fn julian_century_to_julian_years() {
        let century = JulianCenturies::new(1.0);
        let years: JulianYears = century.to();
        assert_abs_diff_eq!(years.value(), 100.0, epsilon = 1e-12);
    }

    proptest! {
        #[test]
        fn julian_year_second_roundtrip(v in -1.0e6_f64..1.0e6_f64) {
            let years = JulianYears::new(v);
            let roundtrip: JulianYears = years.to::<Second>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
