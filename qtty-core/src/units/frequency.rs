//! Angular frequency unit aliases (`Angular / Time`).
//!
//! This module provides a **dimension alias** [`Frequency`].
//!
//! ```rust
//! use qtty_core::angular::{Degree, Radian};
//! use qtty_core::time::Day;
//! use qtty_core::frequency::Frequency;
//!
//! let f: Frequency<Degree, Day> = Frequency::new(180.0);
//! let f_rad: Frequency<Radian, Day> = f.to();
//! assert!((f_rad.value() - core::f64::consts::PI).abs() < 1e-12);
//! ```

use crate::units::angular::Angular;
use crate::units::time::Time;
use crate::{DivDim, Per, Quantity, Unit};

/// Dimension alias for angular frequency (`Angular / Time`).
pub type FrequencyDim = DivDim<Angular, Time>;

/// Marker trait for any unit with frequency dimension (`Angular / Time`).
pub trait FrequencyUnit: Unit<Dim = FrequencyDim> {}
impl<T: Unit<Dim = FrequencyDim>> FrequencyUnit for T {}

/// A frequency quantity parameterized by angular and time units.
///
/// # Examples
///
/// ```rust
/// use qtty_core::angular::{Degree, Radian};
/// use qtty_core::time::{Second, Day};
/// use qtty_core::frequency::Frequency;
///
/// let f1: Frequency<Degree, Second> = Frequency::new(360.0);
/// let f2: Frequency<Radian, Day> = Frequency::new(6.28);
/// ```
pub type Frequency<N, D> = Quantity<Per<N, D>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::angular::{Degree, Degrees, MilliArcsecond, Radian};
    use crate::units::time::{Day, Days, Year};
    use crate::Per;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;
    use std::f64::consts::PI;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic frequency conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn deg_per_day_to_rad_per_day() {
        let f: Frequency<Degree, Day> = Frequency::new(180.0);
        let f_rad: Frequency<Radian, Day> = f.to();
        // 180 deg = π rad
        assert_abs_diff_eq!(f_rad.value(), PI, epsilon = 1e-12);
    }

    #[test]
    fn rad_per_day_to_deg_per_day() {
        let f: Frequency<Radian, Day> = Frequency::new(PI);
        let f_deg: Frequency<Degree, Day> = f.to();
        assert_abs_diff_eq!(f_deg.value(), 180.0, epsilon = 1e-12);
    }

    #[test]
    fn deg_per_day_to_deg_per_year() {
        let f: Frequency<Degree, Day> = Frequency::new(1.0);
        let f_year: Frequency<Degree, Year> = f.to();
        // 1 deg/day = 365.2425 deg/year (tropical year)
        assert_relative_eq!(f_year.value(), 365.2425, max_relative = 1e-6);
    }

    #[test]
    fn deg_per_year_to_deg_per_day() {
        let f: Frequency<Degree, Year> = Frequency::new(365.2425);
        let f_day: Frequency<Degree, Day> = f.to();
        assert_relative_eq!(f_day.value(), 1.0, max_relative = 1e-6);
    }

    #[test]
    fn mas_per_day_to_deg_per_day() {
        let f: Frequency<MilliArcsecond, Day> = Frequency::new(3_600_000.0);
        let f_deg: Frequency<Degree, Day> = f.to();
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
        let f: Frequency<Degree, Day> = Frequency::new(360.0);
        let t: Days = Days::new(0.5);
        let angle: Degrees = f * t;
        assert_abs_diff_eq!(angle.value(), 180.0, epsilon = 1e-9);
    }

    #[test]
    fn time_times_frequency() {
        let f: Frequency<Degree, Day> = Frequency::new(360.0);
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
        let f: Frequency<Degree, Day> = angle / t;
        assert_abs_diff_eq!(f.value(), 360.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_deg_rad_per_day() {
        let original: Frequency<Degree, Day> = Frequency::new(90.0);
        let converted: Frequency<Radian, Day> = original.to();
        let back: Frequency<Degree, Day> = converted.to();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_deg_rad_per_day(f in 1e-6..1e6f64) {
            let original: Frequency<Degree, Day> = Frequency::new(f);
            let converted: Frequency<Radian, Day> = original.to();
            let back: Frequency<Degree, Day> = converted.to();
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * f.abs().max(1.0));
        }

        #[test]
        fn prop_frequency_time_roundtrip(
            f_val in 1e-3..1e3f64,
            t_val in 1e-3..1e3f64
        ) {
            let f: Frequency<Degree, Day> = Frequency::new(f_val);
            let t: Days = Days::new(t_val);
            let angle: Degrees = f * t;
            // angle / t should give back f
            let f_back: Frequency<Degree, Day> = angle / t;
            prop_assert!((f_back.value() - f.value()).abs() / f.value() < 1e-12);
        }
    }
}
