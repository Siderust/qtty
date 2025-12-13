//! Velocity unit aliases (`Length / Time`).
//!
//! This module defines velocity units as *pure type aliases* over [`Per`] using
//! length and time units already defined elsewhere in the crate.
//!
//! No standalone velocity units are introduced: every velocity is represented as
//! `Length / Time` at the type level.
//!
//! ## Design notes
//!
//! - The velocity *dimension* is [`Velocity`] = [`Length`] / [`Time`].
//! - All velocity units are zero-cost type aliases.
//! - Conversions are handled automatically via the underlying length and time units.
//! - No assumptions are made about reference frames, relativistic effects, or media.
//!
//! ## Examples
//!
//! ```rust
//! use qtty_core::length::{Kilometer, Kilometers};
//! use qtty_core::time::{Second, Seconds};
//! use qtty_core::velocity::Velocity;
//!
//! let d = Kilometers::new(42.0);
//! let t = Seconds::new(2.0);
//! let v: Velocity<Kilometer, Second> = d / t;
//! assert!((v.value() - 21.0).abs() < 1e-12);
//! ```
//!
//! ```rust
//! use qtty_core::length::{Meter, Meters};
//! use qtty_core::time::{Hour, Hours};
//! use qtty_core::velocity::Velocity;
//!
//! let v: Velocity<Meter, Hour> = Meters::new(3_600.0) / Hours::new(1.0);
//! assert!((v.value() - 3_600.0).abs() < 1e-12);
//! ```

use crate::units::length::Length;
use crate::units::time::Time;
use crate::{DivDim, Per, Quantity, Unit};

/// Dimension alias for velocities (`Length / Time`).
pub type VelocityDim = DivDim<Length, Time>;

/// Marker trait for any unit whose dimension is [`VelocityDim`].
pub trait VelocityUnit: Unit<Dim = VelocityDim> {}
impl<T: Unit<Dim = VelocityDim>> VelocityUnit for T {}

/// A velocity quantity parameterized by length and time units.
///
/// # Examples
///
/// ```rust
/// use qtty_core::length::{Kilometer, Meter};
/// use qtty_core::time::{Second, Hour};
/// use qtty_core::velocity::Velocity;
///
/// let v1: Velocity<Meter, Second> = Velocity::new(10.0);
/// let v2: Velocity<Kilometer, Hour> = Velocity::new(36.0);
/// ```
pub type Velocity<N, D> = Quantity<Per<N, D>>;

// Note: Specific velocity aliases (e.g. MetersPerSecond) have been removed
// in favor of using `Quantity<Per<Meter, Second>>` directly.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::length::{Au, Kilometer, Kilometers, Meter};
    use crate::units::time::{Day, Hour, Second, Seconds};
    use crate::Per;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic velocity conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn km_per_s_to_m_per_s() {
        let v: Velocity<Kilometer, Second> = Velocity::new(1.0);
        let v_mps: Velocity<Meter, Second> = v.to();
        assert_abs_diff_eq!(v_mps.value(), 1000.0, epsilon = 1e-9);
    }

    #[test]
    fn m_per_s_to_km_per_s() {
        let v: Velocity<Meter, Second> = Velocity::new(1000.0);
        let v_kps: Velocity<Kilometer, Second> = v.to();
        assert_abs_diff_eq!(v_kps.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn km_per_h_to_m_per_s() {
        let v: Velocity<Kilometer, Hour> = Velocity::new(3.6);
        let v_mps: Velocity<Meter, Second> = v.to();
        // 3.6 km/h = 1 m/s
        assert_abs_diff_eq!(v_mps.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn km_per_h_to_km_per_s() {
        let v: Velocity<Kilometer, Hour> = Velocity::new(3600.0);
        let v_kps: Velocity<Kilometer, Second> = v.to();
        // 3600 km/h = 1 km/s
        assert_abs_diff_eq!(v_kps.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn au_per_day_to_km_per_s() {
        let v: Velocity<Au, Day> = Velocity::new(1.0);
        let v_kps: Velocity<Kilometer, Second> = v.to();
        // 1 AU/day = 149,597,870.7 km / 86400 s ≈ 1731.5 km/s
        assert_relative_eq!(v_kps.value(), 1731.5, max_relative = 1e-3);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Per ratio behavior
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn per_ratio_km_s() {
        // Per<Kilometer, Second> should have RATIO = 1000 / 1 = 1000
        let ratio = <Per<Kilometer, Second>>::RATIO;
        // Kilometer::RATIO = 1000, Second::RATIO = 1.0
        // So Per ratio = 1000 / 1.0 = 1000
        assert_relative_eq!(ratio, 1000.0, max_relative = 1e-12);
    }

    #[test]
    fn per_ratio_m_s() {
        // Per<Meter, Second> has RATIO = 1 / 1 = 1
        let ratio = <Per<Meter, Second>>::RATIO;
        assert_relative_eq!(ratio, 1.0, max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Velocity * Time = Length
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn velocity_times_time() {
        let v: Velocity<Kilometer, Second> = Velocity::new(10.0);
        let t: Seconds = Seconds::new(5.0);
        let d: Kilometers = v * t;
        assert_abs_diff_eq!(d.value(), 50.0, epsilon = 1e-9);
    }

    #[test]
    fn time_times_velocity() {
        let v: Velocity<Kilometer, Second> = Velocity::new(10.0);
        let t: Seconds = Seconds::new(5.0);
        let d: Kilometers = t * v;
        assert_abs_diff_eq!(d.value(), 50.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Length / Time = Velocity
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn length_div_time() {
        let d: Kilometers = Kilometers::new(100.0);
        let t: Seconds = Seconds::new(10.0);
        let v: Velocity<Kilometer, Second> = d / t;
        assert_abs_diff_eq!(v.value(), 10.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_mps_kps() {
        let original: Velocity<Meter, Second> = Velocity::new(500.0);
        let converted: Velocity<Kilometer, Second> = original.to();
        let back: Velocity<Meter, Second> = converted.to();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_mps_kps(v in 1e-6..1e6f64) {
            let original: Velocity<Meter, Second> = Velocity::new(v);
            let converted: Velocity<Kilometer, Second> = original.to();
            let back: Velocity<Meter, Second> = converted.to();
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * v.abs().max(1.0));
        }

        #[test]
        fn prop_mps_kps_ratio(v in 1e-6..1e6f64) {
            let mps: Velocity<Meter, Second> = Velocity::new(v);
            let kps: Velocity<Kilometer, Second> = mps.to();
            // 1000 m/s = 1 km/s
            prop_assert!((mps.value() / kps.value() - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn prop_velocity_time_roundtrip(
            v_val in 1e-3..1e3f64,
            t_val in 1e-3..1e3f64
        ) {
            let v: Velocity<Kilometer, Second> = Velocity::new(v_val);
            let t: Seconds = Seconds::new(t_val);
            let d: Kilometers = v * t;
            // d / t should give back v
            let v_back: Velocity<Kilometer, Second> = d / t;
            prop_assert!((v_back.value() - v.value()).abs() / v.value() < 1e-12);
        }
    }
}
