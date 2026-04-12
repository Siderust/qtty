// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Angular rate (angular displacement per unit time) unit aliases (`Angular / Time`).
//!
//! This module models **angular rate** — angular displacement per unit time
//! (e.g. radians/second, degrees/day). It does **not** model SI cycle frequency
//! (Hz = cycles/s), which would be `T⁻¹` with dimensionless cycles.
//!
//! If you need Hertz-style inverse-time frequency, model it directly as
//! `Per<Unitless, Second>` (or a named wrapper) — this module is the wrong place.
//!
//! ```rust
//! use qtty_core::angular::{Degree, Radian};
//! use qtty_core::time::Day;
//! use qtty_core::angular_rate::AngularRate;
//!
//! let f: AngularRate<Degree, Day> = AngularRate::new(180.0);
//! let f_rad: AngularRate<Radian, Day> = f.to();
//! assert!((f_rad.value() - core::f64::consts::PI).abs() < 1e-12);
//! ```

use crate::{Per, Quantity, Unit};

/// Re-export the angular rate dimension from the dimension module.
pub use crate::dimension::AngularRate as AngularRateDimension;

/// Marker trait for any unit with angular-rate dimension ([`AngularRate`](crate::AngularRate)).
pub trait AngularRateUnit: Unit<Dim = crate::AngularRate> {}
impl<T: Unit<Dim = crate::AngularRate>> AngularRateUnit for T {}

/// An angular-rate quantity parameterized by angular and time units.
///
/// # Examples
///
/// ```rust
/// use qtty_core::angular::{Degree, Radian};
/// use qtty_core::time::{Second, Day};
/// use qtty_core::angular_rate::AngularRate;
///
/// let f1: AngularRate<Degree, Second> = AngularRate::new(360.0);
/// let f2: AngularRate<Radian, Day> = AngularRate::new(6.28);
/// ```
pub type AngularRate<N, D> = Quantity<Per<N, D>>;

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    #[cfg(feature = "astro")]
    use crate::units::angular::MilliArcsecond;
    use crate::units::angular::{Degree, Degrees, Radian};
    use crate::units::time::{Day, Days, Year};
    use crate::Per;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;
    use std::f64::consts::PI;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic angular-rate conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn deg_per_day_to_rad_per_day() {
        let f: AngularRate<Degree, Day> = AngularRate::new(180.0);
        let f_rad: AngularRate<Radian, Day> = f.to();
        // 180 deg = π rad
        assert_abs_diff_eq!(f_rad.value(), PI, epsilon = 1e-12);
    }

    #[test]
    fn rad_per_day_to_deg_per_day() {
        let f: AngularRate<Radian, Day> = AngularRate::new(PI);
        let f_deg: AngularRate<Degree, Day> = f.to();
        assert_abs_diff_eq!(f_deg.value(), 180.0, epsilon = 1e-12);
    }

    #[test]
    fn deg_per_day_to_deg_per_year() {
        let f: AngularRate<Degree, Day> = AngularRate::new(1.0);
        let f_year: AngularRate<Degree, Year> = f.to();
        // 1 deg/day = 365.2425 deg/year (tropical year)
        assert_relative_eq!(f_year.value(), 365.2425, max_relative = 1e-6);
    }

    #[test]
    fn deg_per_year_to_deg_per_day() {
        let f: AngularRate<Degree, Year> = AngularRate::new(365.2425);
        let f_day: AngularRate<Degree, Day> = f.to();
        assert_relative_eq!(f_day.value(), 1.0, max_relative = 1e-6);
    }

    #[test]
    #[cfg(feature = "astro")]
    fn mas_per_day_to_deg_per_day() {
        let f: AngularRate<MilliArcsecond, Day> = AngularRate::new(3_600_000.0);
        let f_deg: AngularRate<Degree, Day> = f.to();
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
    // AngularRate * Time = Angle
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn angular_rate_times_time() {
        let f: AngularRate<Degree, Day> = AngularRate::new(360.0);
        let t: Days = Days::new(0.5);
        let angle: Degrees = (f * t).to();
        assert_abs_diff_eq!(angle.value(), 180.0, epsilon = 1e-9);
    }

    #[test]
    fn time_times_angular_rate() {
        let f: AngularRate<Degree, Day> = AngularRate::new(360.0);
        let t: Days = Days::new(0.5);
        let angle: Degrees = (t * f).to();
        assert_abs_diff_eq!(angle.value(), 180.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Angle / Time = AngularRate
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn angle_div_time() {
        let angle: Degrees = Degrees::new(360.0);
        let t: Days = Days::new(1.0);
        let f: AngularRate<Degree, Day> = angle / t;
        assert_abs_diff_eq!(f.value(), 360.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_deg_rad_per_day() {
        let original: AngularRate<Degree, Day> = AngularRate::new(90.0);
        let converted: AngularRate<Radian, Day> = original.to();
        let back: AngularRate<Degree, Day> = converted.to();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_deg_rad_per_day(f in 1e-6..1e6f64) {
            let original: AngularRate<Degree, Day> = AngularRate::new(f);
            let converted: AngularRate<Radian, Day> = original.to();
            let back: AngularRate<Degree, Day> = converted.to();
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * f.abs().max(1.0));
        }

        #[test]
        fn prop_angular_rate_time_roundtrip(
            f_val in 1e-3..1e3f64,
            t_val in 1e-3..1e3f64
        ) {
            let f: AngularRate<Degree, Day> = AngularRate::new(f_val);
            let t: Days = Days::new(t_val);
            let angle: Degrees = (f * t).to();
            // angle / t should give back f
            let f_back: AngularRate<Degree, Day> = angle / t;
            prop_assert!((f_back.value() - f.value()).abs() / f.value() < 1e-12);
        }
    }
}
