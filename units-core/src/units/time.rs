//! Time units.
//!
//! The canonical scaling unit for this dimension is [`Day`] (`Day::RATIO == 1.0`). All other time unit ratios are
//! expressed in *days*.
//!
//! ```rust
//! use unit_core::time::{Days, Hour};
//!
//! let half_day = Days::new(0.5);
//! let hours = half_day.to::<Hour>();
//! assert!((hours.value() - 12.0).abs() < 1e-12);
//! ```

use crate::{Dimension, Quantity, Unit};
use unit_derive::Unit;

/// Dimension tag for time.
pub enum Time {}
impl Dimension for Time {}

/// Marker trait for any [`Unit`] whose dimension is [`Time`].
pub trait TimeUnit: Unit<Dim = Time> {}
impl<T: Unit<Dim = Time>> TimeUnit for T {}

/// Milliseconds (`1 ms = 1/1000 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ms", dimension = Time, ratio = 1.0 / (24.0 * 3600.0 * 1_000.0))]
pub struct Millisecond;
/// A quantity measured in milliseconds.
pub type Milliseconds = Quantity<Millisecond>;
/// One millisecond.
pub const MILLISEC: Milliseconds = Milliseconds::new(1.0);

/// Seconds (`1 s = 1/86400 day`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "sec", dimension = Time, ratio = 1.0 / (24.0 * 3600.0))]
pub struct Second;
/// A quantity measured in seconds.
pub type Seconds = Quantity<Second>;
/// One second.
pub const SEC: Seconds = Seconds::new(1.0);

/// Minutes (`60 s`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "min", dimension = Time, ratio = 1.0 / (24.0 * 60.0))]
pub struct Minute;
/// A quantity measured in minutes.
pub type Minutes = Quantity<Minute>;
/// One minute.
pub const MIN: Minutes = Minutes::new(1.0);

/// Hours (`60 min`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "h", dimension = Time, ratio = 1.0 / 24.0)]
pub struct Hour;
/// A quantity measured in hours.
pub type Hours = Quantity<Hour>;
/// One hour.
pub const HOUR: Hours = Hours::new(1.0);

/// Mean solar day.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "d", dimension = Time, ratio = 1.0)]
pub struct Day;
/// A quantity measured in days.
pub type Days = Quantity<Day>;
/// One day.
pub const DAY: Days = Days::new(1.0);

/// Week (`7 days`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "wk", dimension = Time, ratio = 7.0)]
pub struct Week;
/// A quantity measured in weeks.
pub type Weeks = Quantity<Week>;
/// One week.
pub const WEEK: Weeks = Weeks::new(1.0);

/// Mean tropical year (IAU 2015).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "yr", dimension = Time, ratio = 365.242_5)]
pub struct Year;
/// A quantity measured in mean tropical years.
pub type Years = Quantity<Year>;
/// One year.
pub const YEAR: Years = Years::new(1.0);

/// Century (`100` mean tropical years).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cent", dimension = Time, ratio = 36_524.25)]
pub struct Century;
/// A quantity measured in centuries.
pub type Centuries = Quantity<Century>;
/// One century.
pub const CENTURY: Centuries = Centuries::new(1.0);

/// Julian year (`365.25` days).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "JY", dimension = Time, ratio = 365.25)]
pub struct JulianYear;
/// A quantity measured in Julian years.
pub type JulianYears = Quantity<JulianYear>;
/// One Julian year.
pub const JULIAN_YEAR: JulianYears = JulianYears::new(1.0);

/// Julian century (`36525` days).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "JC", dimension = Time, ratio = 36_525.0)]
pub struct JulianCentury;
/// A quantity measured in Julian centuries.
pub type JulianCenturies = Quantity<JulianCentury>;
/// One Julian century.
pub const JULIAN_CENTURY: JulianCenturies = JulianCenturies::new(1.0);

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
        // Day::RATIO = 1.0, so Second::RATIO should be 1/(24*3600)
        assert_abs_diff_eq!(Second::RATIO, 1.0 / 86400.0, epsilon = 1e-15);
    }

    #[test]
    fn minute_ratio_sanity() {
        // 1 minute = 1/1440 day
        assert_abs_diff_eq!(Minute::RATIO, 1.0 / 1440.0, epsilon = 1e-15);
    }

    #[test]
    fn hour_ratio_sanity() {
        // 1 hour = 1/24 day
        assert_abs_diff_eq!(Hour::RATIO, 1.0 / 24.0, epsilon = 1e-15);
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
