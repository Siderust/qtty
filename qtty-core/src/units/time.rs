//! Time units.
//!
//! The canonical scaling unit for this dimension is [`Second`] (`Second::RATIO == 1.0`). All other time unit ratios are
//! expressed in *seconds*.
//!
//! ## Precision and conventions
//!
//! - The **SI second** is the canonical unit.
//! - Civil units such as [`Day`] are expressed using the conventional mapping
//!   `1 day = 86_400 s` (mean solar day; leap seconds ignored).
//! - “Mean” astronomical units (e.g., [`SiderealDay`], [`SynodicMonth`], [`SiderealYear`]) are **approximations**
//!   that vary slightly with epoch/definition. Each unit documents the convention used.
//!
//! ```rust
//! use qtty_core::time::{Hours, Second, Hour};
//!
//! let half_hour = Hours::new(0.5);
//! let seconds = half_hour.to::<Second>();
//! assert!((seconds.value() - 1800.0).abs() < 1e-12);
//!
//! let two_hours = seconds.to::<Hour>();
//! assert!((two_hours.value() - 0.5).abs() < 1e-12);
//! ```

use crate::{Dimension, Quantity, Unit};
use qtty_derive::Unit;

/// Dimension tag for time.
pub enum Time {}
impl Dimension for Time {}

/// Marker trait for any [`Unit`] whose dimension is [`Time`].
pub trait TimeUnit: Unit<Dim = Time> {}
impl<T: Unit<Dim = Time>> TimeUnit for T {}

/// Conventional civil mapping used by this module: seconds per mean solar day.
pub const SECONDS_PER_DAY: f64 = 86_400.0;

// --- SI submultiples of the second ---

/// Attoseconds (`1 as = 10^-18 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "as", dimension = Time, ratio = 1e-18)]
pub struct Attosecond;
/// A quantity measured in attoseconds.
pub type Attoseconds = Quantity<Attosecond>;
/// A constant representing one attosecond.
pub const ATTOSEC: Attoseconds = Attoseconds::new(1.0);

/// Femtoseconds (`1 fs = 10^-15 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "fs", dimension = Time, ratio = 1e-15)]
pub struct Femtosecond;
/// A quantity measured in femtoseconds.
pub type Femtoseconds = Quantity<Femtosecond>;
/// A constant representing one femtosecond.
pub const FEMTOSEC: Femtoseconds = Femtoseconds::new(1.0);

/// Picoseconds (`1 ps = 10^-12 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ps", dimension = Time, ratio = 1e-12)]
pub struct Picosecond;
/// A quantity measured in picoseconds.
pub type Picoseconds = Quantity<Picosecond>;
/// A constant representing one picosecond.
pub const PICOSEC: Picoseconds = Picoseconds::new(1.0);

/// Nanoseconds (`1 ns = 10^-9 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ns", dimension = Time, ratio = 1e-9)]
pub struct Nanosecond;
/// A quantity measured in nanoseconds.
pub type Nanoseconds = Quantity<Nanosecond>;
/// A constant representing one nanosecond.
pub const NANOSEC: Nanoseconds = Nanoseconds::new(1.0);

/// Microseconds (`1 µs = 10^-6 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "µs", dimension = Time, ratio = 1e-6)]
pub struct Microsecond;
/// A quantity measured in microseconds.
pub type Microseconds = Quantity<Microsecond>;
/// A constant representing one microsecond.
pub const MICROSEC: Microseconds = Microseconds::new(1.0);

/// Milliseconds (`1 ms = 10^-3 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ms", dimension = Time, ratio = 1e-3)]
pub struct Millisecond;
/// A quantity measured in milliseconds.
pub type Milliseconds = Quantity<Millisecond>;
/// A constant representing one millisecond.
pub const MILLISEC: Milliseconds = Milliseconds::new(1.0);

/// Centiseconds (`1 cs = 10^-2 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cs", dimension = Time, ratio = 1e-2)]
pub struct Centisecond;
/// A quantity measured in centiseconds.
pub type Centiseconds = Quantity<Centisecond>;
/// A constant representing one centisecond.
pub const CENTISEC: Centiseconds = Centiseconds::new(1.0);

/// Deciseconds (`1 ds = 10^-1 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ds", dimension = Time, ratio = 1e-1)]
pub struct Decisecond;
/// A quantity measured in deciseconds.
pub type Deciseconds = Quantity<Decisecond>;
/// A constant representing one decisecond.
pub const DECISEC: Deciseconds = Deciseconds::new(1.0);

/// Seconds (SI base unit).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "s", dimension = Time, ratio = 1.0)]
pub struct Second;
/// A quantity measured in seconds.
pub type Seconds = Quantity<Second>;
/// A constant representing one second.
pub const SEC: Seconds = Seconds::new(1.0);

// --- SI multiples of the second ---

/// Decaseconds (`1 das = 10 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "das", dimension = Time, ratio = 10.0)]
pub struct Decasecond;
/// A quantity measured in decaseconds.
pub type Decaseconds = Quantity<Decasecond>;
/// A constant representing one decasecond.
pub const DECASEC: Decaseconds = Decaseconds::new(1.0);

/// Hectoseconds (`1 hs = 100 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "hs", dimension = Time, ratio = 100.0)]
pub struct Hectosecond;
/// A quantity measured in hectoseconds.
pub type Hectoseconds = Quantity<Hectosecond>;
/// A constant representing one hectosecond.
pub const HECTOSEC: Hectoseconds = Hectoseconds::new(1.0);

/// Kiloseconds (`1 ks = 1_000 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ks", dimension = Time, ratio = 1_000.0)]
pub struct Kilosecond;
/// A quantity measured in kiloseconds.
pub type Kiloseconds = Quantity<Kilosecond>;
/// A constant representing one kilosecond.
pub const KILOSEC: Kiloseconds = Kiloseconds::new(1.0);

/// Megaseconds (`1 Ms = 10^6 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Ms", dimension = Time, ratio = 1e6)]
pub struct Megasecond;
/// A quantity measured in megaseconds.
pub type Megaseconds = Quantity<Megasecond>;
/// A constant representing one megasecond.
pub const MEGASEC: Megaseconds = Megaseconds::new(1.0);

/// Gigaseconds (`1 Gs = 10^9 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Gs", dimension = Time, ratio = 1e9)]
pub struct Gigasecond;
/// A quantity measured in gigaseconds.
pub type Gigaseconds = Quantity<Gigasecond>;
/// A constant representing one gigasecond.
pub const GIGASEC: Gigaseconds = Gigaseconds::new(1.0);

/// Teraseconds (`1 Ts = 10^12 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Ts", dimension = Time, ratio = 1e12)]
pub struct Terasecond;
/// A quantity measured in teraseconds.
pub type Teraseconds = Quantity<Terasecond>;
/// A constant representing one terasecond.
pub const TERASEC: Teraseconds = Teraseconds::new(1.0);

// --- Common civil units ---

/// Minutes (`60 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "min", dimension = Time, ratio = 60.0)]
pub struct Minute;
/// A quantity measured in minutes.
pub type Minutes = Quantity<Minute>;
/// A constant representing one minute.
pub const MIN: Minutes = Minutes::new(1.0);

/// Hours (`3_600 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "h", dimension = Time, ratio = 3_600.0)]
pub struct Hour;
/// A quantity measured in hours.
pub type Hours = Quantity<Hour>;
/// A constant representing one hour.
pub const HOUR: Hours = Hours::new(1.0);

/// Mean solar day (`86_400 s` by convention; leap seconds ignored).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "d", dimension = Time, ratio = SECONDS_PER_DAY)]
pub struct Day;
/// A quantity measured in days.
pub type Days = Quantity<Day>;
/// A constant representing one day.
pub const DAY: Days = Days::new(1.0);

/// Week (`7 d = 604_800 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "wk", dimension = Time, ratio = 7.0 * SECONDS_PER_DAY)]
pub struct Week;
/// A quantity measured in weeks.
pub type Weeks = Quantity<Week>;
/// A constant representing one week.
pub const WEEK: Weeks = Weeks::new(1.0);

/// Fortnight (`14 d = 1_209_600 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "fn", dimension = Time, ratio = 14.0 * SECONDS_PER_DAY)]
pub struct Fortnight;
/// A quantity measured in fortnights.
pub type Fortnights = Quantity<Fortnight>;
/// A constant representing one fortnight.
pub const FORTNIGHT: Fortnights = Fortnights::new(1.0);

/// Mean tropical year, as a conventional mean length.
///
/// Convention used: `365.2425 d`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "yr", dimension = Time, ratio = 365.242_5 * SECONDS_PER_DAY)]
pub struct Year;
/// A quantity measured in years.
pub type Years = Quantity<Year>;
/// A constant representing one year.
pub const YEAR: Years = Years::new(1.0);

/// Decade (`10` mean tropical years).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "dec", dimension = Time, ratio = 10.0 * 365.242_5 * SECONDS_PER_DAY)]
pub struct Decade;
/// A quantity measured in decades.
pub type Decades = Quantity<Decade>;
/// A constant representing one decade.
pub const DECADE: Decades = Decades::new(1.0);

/// Century (`100` mean tropical years).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cent", dimension = Time, ratio = 100.0 * 365.242_5 * SECONDS_PER_DAY)]
pub struct Century;
/// A quantity measured in centuries.
pub type Centuries = Quantity<Century>;
/// A constant representing one century.
pub const CENTURY: Centuries = Centuries::new(1.0);

/// Millennium (`1000` mean tropical years).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mill", dimension = Time, ratio = 1000.0 * 365.242_5 * SECONDS_PER_DAY)]
pub struct Millennium;
/// A quantity measured in millennia.
pub type Millennia = Quantity<Millennium>;
/// A constant representing one millennium.
pub const MILLENNIUM: Millennia = Millennia::new(1.0);

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
/// Convention used: `1 synodic month ≈ 29.530588 d`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "synmo", dimension = Time, ratio = 29.530_588 * SECONDS_PER_DAY)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn seconds_to_minutes() {
        let sec = Seconds::new(60.0);
        let min = sec.to::<Minute>();
        assert_abs_diff_eq!(min.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn minutes_to_hours() {
        let min = Minutes::new(60.0);
        let hr = min.to::<Hour>();
        assert_abs_diff_eq!(hr.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn hours_to_days() {
        let hr = Hours::new(24.0);
        let day = hr.to::<Day>();
        assert_abs_diff_eq!(day.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn seconds_86400_equals_one_day() {
        let sec = Seconds::new(86400.0);
        let day = sec.to::<Day>();
        assert_abs_diff_eq!(day.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn day_to_seconds() {
        let day = Days::new(1.0);
        let sec = day.to::<Second>();
        assert_abs_diff_eq!(sec.value(), 86400.0, epsilon = 1e-9);
    }

    #[test]
    fn days_to_weeks() {
        let day = Days::new(7.0);
        let week = day.to::<Week>();
        assert_abs_diff_eq!(week.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn julian_year_to_days() {
        let jy = JulianYears::new(1.0);
        let day = jy.to::<Day>();
        assert_abs_diff_eq!(day.value(), 365.25, epsilon = 1e-9);
    }

    #[test]
    fn julian_century_to_days() {
        let jc = JulianCenturies::new(1.0);
        let day = jc.to::<Day>();
        assert_abs_diff_eq!(day.value(), 36525.0, epsilon = 1e-9);
    }

    #[test]
    fn julian_century_to_julian_years() {
        let jc = JulianCenturies::new(1.0);
        let jy = jc.to::<JulianYear>();
        assert_abs_diff_eq!(jy.value(), 100.0, epsilon = 1e-9);
    }

    #[test]
    fn tropical_year_to_days() {
        let y = Years::new(1.0);
        let day = y.to::<Day>();
        assert_abs_diff_eq!(day.value(), 365.2425, epsilon = 1e-9);
    }

    #[test]
    fn century_to_days() {
        let c = Centuries::new(1.0);
        let day = c.to::<Day>();
        assert_abs_diff_eq!(day.value(), 36524.25, epsilon = 1e-9);
    }

    #[test]
    fn milliseconds_to_seconds() {
        let ms = Milliseconds::new(1000.0);
        let sec = ms.to::<Second>();
        assert_abs_diff_eq!(sec.value(), 1.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_day_second() {
        let original = Days::new(1.5);
        let converted = original.to::<Second>();
        let back = converted.to::<Day>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
    }

    #[test]
    fn roundtrip_julian_year_day() {
        let original = JulianYears::new(2.5);
        let converted = original.to::<Day>();
        let back = converted.to::<JulianYear>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Ratio sanity checks
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn second_ratio_sanity() {
        // Second::RATIO = 1.0 (canonical unit)
        assert_abs_diff_eq!(Second::RATIO, 1.0, epsilon = 1e-15);
    }

    #[test]
    fn minute_ratio_sanity() {
        // 1 minute = 60 seconds
        assert_abs_diff_eq!(Minute::RATIO, 60.0, epsilon = 1e-15);
    }

    #[test]
    fn hour_ratio_sanity() {
        // 1 hour = 3600 seconds
        assert_abs_diff_eq!(Hour::RATIO, 3_600.0, epsilon = 1e-15);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_day_second(d in -1e6..1e6f64) {
            let original = Days::new(d);
            let converted = original.to::<Second>();
            let back = converted.to::<Day>();
            prop_assert!((back.value() - original.value()).abs() < 1e-9);
        }

        #[test]
        fn prop_day_second_ratio(d in 1e-6..1e6f64) {
            let day = Days::new(d);
            let sec = day.to::<Second>();
            // 1 day = 86400 seconds
            prop_assert!((sec.value() / day.value() - 86400.0).abs() < 1e-9);
        }

        #[test]
        fn prop_julian_year_day_ratio(y in 1e-6..1e6f64) {
            let jy = JulianYears::new(y);
            let day = jy.to::<Day>();
            // 1 Julian year = 365.25 days
            prop_assert!((day.value() / jy.value() - 365.25).abs() < 1e-9);
        }
    }
}
