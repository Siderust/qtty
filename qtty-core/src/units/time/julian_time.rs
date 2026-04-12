// SPDX-License-Identifier: AGPL-3.0-or-later
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
