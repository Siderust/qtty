//! # Time Units Module
//!
//! This module provides types and utilities for handling time-related calculations
//! in astronomical and scientific contexts. It includes representations for various
//! time systems and conversions between them.
//!
//! ## Features
//! - **Days**: A simple representation of time in days, with arithmetic operations.
//! - **Julian Year**: A standardized year of exactly 365.25 days, used in astronomy.
//! - **Centuries**: Representation of time intervals in Julian centuries (36525 days).
//! - **Years**: Representation of time intervals in years.

use super::*;

pub enum Time {}
impl Dimension for Time {}

pub trait TimeUnit: Unit<Dim = Time> {}
impl<T: Unit<Dim = Time>> TimeUnit for T {}

define_unit!("ms", Millisecond, Time, 1.0 / (24.0 * 3600.0 * 1_000.0));
pub type Milliseconds = Quantity<Millisecond>;
pub const MILLISEC: Milliseconds = Milliseconds::new(1.0);

define_unit!("sec", Second, Time, 1.0 / (24.0 * 3600.0));
pub type Seconds = Quantity<Second>;
pub const SEC: Seconds = Seconds::new(1.0);

define_unit!("min", Minute, Time, 1.0 / (24.0 * 60.0));
pub type Minutes = Quantity<Minute>;
pub const MIN: Minutes = Minutes::new(1.0);

define_unit!("h", Hour, Time, 1.0 / 24.0);
pub type Hours = Quantity<Hour>;
pub const HOUR: Hours = Hours::new(1.0);

// Mean solar day
define_unit!("d", Day, Time, 1.0);
pub type Days = Quantity<Day>;
pub const DAY: Days = Days::new(1.0);

// Week: 7 solar
define_unit!("wk", Week, Time, 7.0);
pub type Weeks = Quantity<Week>;
pub const WEEK: Weeks = Weeks::new(1.0);

// Mean tropical year (IAU 2015)
define_unit!("yr", Year, Time, 365.242_5);
pub type Years = Quantity<Year>;
pub const YEAR: Years = Years::new(1.0);

// Century: 100 mean tropical years
define_unit!("cent", Century, Time, 36_524.25);
pub type Centuries = Quantity<Century>;
pub const CENTURY: Centuries = Centuries::new(1.0);

define_unit!("JY", JulianYear, Time, 365.25);
pub type JulianYears = Quantity<JulianYear>;
pub const JULIAN_YEAR: JulianYears = JulianYears::new(1.0);

// Julian century: exactly 36,525 days (365.25 × 100).
define_unit!("JC", JulianCentury, Time, 36_525.0);
pub type JulianCenturies = Quantity<JulianCentury>;
pub const JULIAN_CENTURY: JulianCenturies = JulianCenturies::new(1.0);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
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
    fn day_to_hours() {
        let day = Days::new(1.0);
        let hr = day.to::<Hour>();
        assert_abs_diff_eq!(hr.value(), 24.0, epsilon = 1e-12);
    }

    #[test]
    fn day_to_minutes() {
        let day = Days::new(1.0);
        let min = day.to::<Minute>();
        assert_abs_diff_eq!(min.value(), 1440.0, epsilon = 1e-9);
    }

    #[test]
    fn week_to_days() {
        let wk = Weeks::new(1.0);
        let day = wk.to::<Day>();
        assert_abs_diff_eq!(day.value(), 7.0, epsilon = 1e-12);
    }

    #[test]
    fn milliseconds_to_seconds() {
        let ms = Milliseconds::new(1000.0);
        let sec = ms.to::<Second>();
        assert_abs_diff_eq!(sec.value(), 1.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Year conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn year_to_days() {
        let yr = Years::new(1.0);
        let day = yr.to::<Day>();
        // Mean tropical year = 365.2425 days
        assert_abs_diff_eq!(day.value(), 365.2425, epsilon = 1e-9);
    }

    #[test]
    fn julian_year_to_days() {
        let jy = JulianYears::new(1.0);
        let day = jy.to::<Day>();
        assert_abs_diff_eq!(day.value(), 365.25, epsilon = 1e-12);
    }

    #[test]
    fn century_to_days() {
        let cent = Centuries::new(1.0);
        let day = cent.to::<Day>();
        // 100 tropical years = 36524.25 days
        assert_abs_diff_eq!(day.value(), 36524.25, epsilon = 1e-9);
    }

    #[test]
    fn julian_century_to_days() {
        let jc = JulianCenturies::new(1.0);
        let day = jc.to::<Day>();
        assert_abs_diff_eq!(day.value(), 36525.0, epsilon = 1e-12);
    }

    #[test]
    fn julian_century_to_julian_years() {
        let jc = JulianCenturies::new(1.0);
        let jy = jc.to::<JulianYear>();
        assert_abs_diff_eq!(jy.value(), 100.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_day_second() {
        let original = Days::new(1.5);
        let sec = original.to::<Second>();
        let back = sec.to::<Day>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
    }

    #[test]
    fn roundtrip_year_day() {
        let original = Years::new(2.0);
        let day = original.to::<Day>();
        let back = day.to::<Year>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Unit constants
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unit_constants() {
        assert_eq!(MILLISEC.value(), 1.0);
        assert_eq!(SEC.value(), 1.0);
        assert_eq!(MIN.value(), 1.0);
        assert_eq!(HOUR.value(), 1.0);
        assert_eq!(DAY.value(), 1.0);
        assert_eq!(WEEK.value(), 1.0);
        assert_eq!(YEAR.value(), 1.0);
        assert_eq!(CENTURY.value(), 1.0);
        assert_eq!(JULIAN_YEAR.value(), 1.0);
        assert_eq!(JULIAN_CENTURY.value(), 1.0);
    }

    #[test]
    fn constant_arithmetic() {
        // 2 days in seconds
        let two_days = 2.0 * DAY;
        let two_days_sec = two_days.to::<Second>();
        assert_abs_diff_eq!(two_days_sec.value(), 172800.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_seconds() {
        let s = Seconds::new(42.5);
        assert_eq!(format!("{}", s), "42.5 \"sec\"");
    }

    #[test]
    fn display_days() {
        let d = Days::new(7.0);
        assert_eq!(format!("{}", d), "7 \"d\"");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn zero_time() {
        let zero = Seconds::new(0.0);
        assert_eq!(zero.to::<Day>().value(), 0.0);
        assert_eq!(zero.to::<Year>().value(), 0.0);
    }

    #[test]
    fn negative_time() {
        let neg = Days::new(-1.0);
        let sec = neg.to::<Second>();
        assert_abs_diff_eq!(sec.value(), -86400.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_conversion_roundtrip(days in -1e6..1e6f64) {
            let original = Days::new(days);
            let sec = original.to::<Second>();
            let back = sec.to::<Day>();
            assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
        }

        #[test]
        fn prop_hours_to_days_ratio(hours in -1e6..1e6f64) {
            let hr = Hours::new(hours);
            let day = hr.to::<Day>();
            assert_relative_eq!(day.value(), hours / 24.0, max_relative = 1e-12);
        }

        #[test]
        fn prop_seconds_to_days_ratio(days in -1e4..1e4f64) {
            let d = Days::new(days);
            let sec = d.to::<Second>();
            assert_relative_eq!(sec.value(), days * 86400.0, max_relative = 1e-12);
        }
    }
}
