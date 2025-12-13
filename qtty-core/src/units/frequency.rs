//! Angular frequency unit aliases (`Angular / Time`).
//!
//! This module provides a **dimension alias** [`Frequency`] and a small set of **type aliases**
//! built on [`Per`] for common angular-frequency units (e.g., `rad/s`, `deg/day`, `mas/yr`).
//!
//! Naming conventions:
//! - `XPerY` is the **unit type** (`Per<X, Y>`).
//! - `XsPerY` is the corresponding **quantity type** (`Quantity<XPerY>`).
//!
//! ```rust
//! use qtty_core::frequency::{DegreesPerDay, RadiansPerDay};
//!
//! let f: DegreesPerDay = qtty_core::Quantity::new(180.0);
//! let f_rad: RadiansPerDay = f.to();
//! assert!((f_rad.value() - core::f64::consts::PI).abs() < 1e-12);
//! ```

use crate::units::angular::{
    Angular, Arcsecond, Degree, Gradian, MicroArcsecond, MilliArcsecond, Radian, Turn,
};
use crate::units::time::{Day, Hour, Minute, Second, Time, Year};
use crate::{DivDim, Per, Quantity, Unit};

/// Dimension alias for angular frequency (`Angular / Time`).
pub type Frequency = DivDim<Angular, Time>;

/// Marker trait for any unit with frequency dimension (`Angular / Time`).
pub trait FrequencyUnit: Unit<Dim = Frequency> {}
impl<T: Unit<Dim = Frequency>> FrequencyUnit for T {}

macro_rules! freq_alias {
    // (UnitName, QuantityName, Numerator, Denominator, "short human label")
    ($u:ident, $q:ident, $num:ty, $den:ty, $label:literal) => {
        #[doc = $label]
        pub type $u = Per<$num, $den>;
        #[doc = $label]
        pub type $q = Quantity<$u>;
    };
}

// Radians
freq_alias!(
    RadianPerSecond,
    RadiansPerSecond,
    Radian,
    Second,
    "Radians per second (`rad/s`)."
);
freq_alias!(
    RadianPerMinute,
    RadiansPerMinute,
    Radian,
    Minute,
    "Radians per minute (`rad/min`)."
);
freq_alias!(
    RadianPerHour,
    RadiansPerHour,
    Radian,
    Hour,
    "Radians per hour (`rad/h`)."
);
freq_alias!(
    RadianPerDay,
    RadiansPerDay,
    Radian,
    Day,
    "Radians per day (`rad/d`)."
);
freq_alias!(
    RadianPerYear,
    RadiansPerYear,
    Radian,
    Year,
    "Radians per year (`rad/yr`)."
);

// Degrees
freq_alias!(
    DegreePerSecond,
    DegreesPerSecond,
    Degree,
    Second,
    "Degrees per second (`deg/s`)."
);
freq_alias!(
    DegreePerMinute,
    DegreesPerMinute,
    Degree,
    Minute,
    "Degrees per minute (`deg/min`)."
);
freq_alias!(
    DegreePerHour,
    DegreesPerHour,
    Degree,
    Hour,
    "Degrees per hour (`deg/h`)."
);
freq_alias!(
    DegreePerDay,
    DegreesPerDay,
    Degree,
    Day,
    "Degrees per day (`deg/d`)."
);
freq_alias!(
    DegreePerYear,
    DegreesPerYear,
    Degree,
    Year,
    "Degrees per year (`deg/yr`)."
);

// Turns (revolutions)
freq_alias!(
    TurnPerSecond,
    TurnsPerSecond,
    Turn,
    Second,
    "Turns per second (`turn/s`)."
);
freq_alias!(
    TurnPerMinute,
    TurnsPerMinute,
    Turn,
    Minute,
    "Turns per minute (`turn/min`)."
);
freq_alias!(
    TurnPerHour,
    TurnsPerHour,
    Turn,
    Hour,
    "Turns per hour (`turn/h`)."
);
freq_alias!(
    TurnPerDay,
    TurnsPerDay,
    Turn,
    Day,
    "Turns per day (`turn/d`)."
);
freq_alias!(
    TurnPerYear,
    TurnsPerYear,
    Turn,
    Year,
    "Turns per year (`turn/yr`)."
);

// Gradians (gon)
freq_alias!(
    GradianPerSecond,
    GradiansPerSecond,
    Gradian,
    Second,
    "Gradians per second (`gon/s`)."
);
freq_alias!(
    GradianPerMinute,
    GradiansPerMinute,
    Gradian,
    Minute,
    "Gradians per minute (`gon/min`)."
);
freq_alias!(
    GradianPerHour,
    GradiansPerHour,
    Gradian,
    Hour,
    "Gradians per hour (`gon/h`)."
);
freq_alias!(
    GradianPerDay,
    GradiansPerDay,
    Gradian,
    Day,
    "Gradians per day (`gon/d`)."
);
freq_alias!(
    GradianPerYear,
    GradiansPerYear,
    Gradian,
    Year,
    "Gradians per year (`gon/yr`)."
);

// Arcseconds and submultiples (common in astrometry)
freq_alias!(
    ArcsecondPerSecond,
    ArcsecondsPerSecond,
    Arcsecond,
    Second,
    "Arcseconds per second (`arcsec/s`)."
);
freq_alias!(
    ArcsecondPerDay,
    ArcsecondsPerDay,
    Arcsecond,
    Day,
    "Arcseconds per day (`arcsec/d`)."
);
freq_alias!(
    ArcsecondPerYear,
    ArcsecondsPerYear,
    Arcsecond,
    Year,
    "Arcseconds per year (`arcsec/yr`)."
);

freq_alias!(
    MilliArcsecondPerSecond,
    MilliArcsecondsPerSecond,
    MilliArcsecond,
    Second,
    "Milliarcseconds per second (`mas/s`)."
);
freq_alias!(
    MilliArcsecondPerDay,
    MilliArcsecondsPerDay,
    MilliArcsecond,
    Day,
    "Milliarcseconds per day (`mas/d`)."
);
freq_alias!(
    MilliArcsecondPerYear,
    MilliArcsecondsPerYear,
    MilliArcsecond,
    Year,
    "Milliarcseconds per year (`mas/yr`)."
);

freq_alias!(
    MicroArcsecondPerSecond,
    MicroArcsecondsPerSecond,
    MicroArcsecond,
    Second,
    "Microarcseconds per second (`µas/s`)."
);
freq_alias!(
    MicroArcsecondPerDay,
    MicroArcsecondsPerDay,
    MicroArcsecond,
    Day,
    "Microarcseconds per day (`µas/d`)."
);
freq_alias!(
    MicroArcsecondPerYear,
    MicroArcsecondsPerYear,
    MicroArcsecond,
    Year,
    "Microarcseconds per year (`µas/yr`)."
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::angular::Degrees;
    use crate::units::time::Days;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;
    use std::f64::consts::PI;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic frequency conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn deg_per_day_to_rad_per_day() {
        let f: DegreesPerDay = Quantity::new(180.0);
        let f_rad: RadiansPerDay = f.to();
        // 180 deg = π rad
        assert_abs_diff_eq!(f_rad.value(), PI, epsilon = 1e-12);
    }

    #[test]
    fn rad_per_day_to_deg_per_day() {
        let f: RadiansPerDay = Quantity::new(PI);
        let f_deg: DegreesPerDay = f.to();
        assert_abs_diff_eq!(f_deg.value(), 180.0, epsilon = 1e-12);
    }

    #[test]
    fn deg_per_day_to_deg_per_year() {
        let f: DegreesPerDay = Quantity::new(1.0);
        let f_year: DegreesPerYear = f.to();
        // 1 deg/day = 365.2425 deg/year (tropical year)
        assert_relative_eq!(f_year.value(), 365.2425, max_relative = 1e-6);
    }

    #[test]
    fn deg_per_year_to_deg_per_day() {
        let f: DegreesPerYear = Quantity::new(365.2425);
        let f_day: DegreesPerDay = f.to();
        assert_relative_eq!(f_day.value(), 1.0, max_relative = 1e-6);
    }

    #[test]
    fn mas_per_day_to_deg_per_day() {
        let f: MilliArcsecondsPerDay = Quantity::new(3_600_000.0);
        let f_deg: DegreesPerDay = f.to();
        // 3,600,000 mas = 1 deg
        assert_abs_diff_eq!(f_deg.value(), 1.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Per ratio behavior
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn per_ratio_deg_day() {
        // Degree::RATIO = 1.0, Day::RATIO = 86400.0
        // So Per<Degree, Day>::RATIO = 1.0 / 86400.0
        let ratio = <Per<Degree, Day>>::RATIO;
        assert_abs_diff_eq!(ratio, 1.0 / 86400.0, epsilon = 1e-12);
    }

    #[test]
    fn per_ratio_rad_day() {
        // Radian::RATIO = 180/π, Day::RATIO = 86400.0
        let ratio = <Per<Radian, Day>>::RATIO;
        assert_relative_eq!(ratio, (180.0 / PI) / 86400.0, max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Frequency * Time = Angle
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn frequency_times_time() {
        let f: DegreesPerDay = Quantity::new(360.0);
        let t: Days = Days::new(0.5);
        let angle: Degrees = f * t;
        assert_abs_diff_eq!(angle.value(), 180.0, epsilon = 1e-9);
    }

    #[test]
    fn time_times_frequency() {
        let f: DegreesPerDay = Quantity::new(360.0);
        let t: Days = Days::new(0.5);
        let angle: Degrees = t * f;
        assert_abs_diff_eq!(angle.value(), 180.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Angle / Time = Frequency
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn angle_div_time() {
        let angle: Degrees = Degrees::new(360.0);
        let t: Days = Days::new(1.0);
        let f: DegreesPerDay = angle / t;
        assert_abs_diff_eq!(f.value(), 360.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_deg_rad_per_day() {
        let original: DegreesPerDay = Quantity::new(90.0);
        let converted: RadiansPerDay = original.to();
        let back: DegreesPerDay = converted.to();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_deg_rad_per_day(f in 1e-6..1e6f64) {
            let original: DegreesPerDay = Quantity::new(f);
            let converted: RadiansPerDay = original.to();
            let back: DegreesPerDay = converted.to();
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * f.abs().max(1.0));
        }

        #[test]
        fn prop_frequency_time_roundtrip(
            f_val in 1e-3..1e3f64,
            t_val in 1e-3..1e3f64
        ) {
            let f: DegreesPerDay = Quantity::new(f_val);
            let t: Days = Days::new(t_val);
            let angle: Degrees = f * t;
            // angle / t should give back f
            let f_back: DegreesPerDay = angle / t;
            prop_assert!((f_back.value() - f.value()).abs() / f.value() < 1e-12);
        }
    }
}
