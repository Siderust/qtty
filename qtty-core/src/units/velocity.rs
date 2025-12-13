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
//! use qtty_core::length::Kilometers;
//! use qtty_core::time::Seconds;
//! use qtty_core::velocity::KilometersPerSecond;
//!
//! let d = Kilometers::new(42.0);
//! let t = Seconds::new(2.0);
//! let v: KilometersPerSecond = d / t;
//! assert!((v.value() - 21.0).abs() < 1e-12);
//! ```
//!
//! ```rust
//! use qtty_core::length::Meters;
//! use qtty_core::time::Hours;
//! use qtty_core::velocity::MetersPerHour;
//!
//! let v = Meters::new(3_600.0) / Hours::new(1.0);
//! assert!((v.value() - 3_600.0).abs() < 1e-12);
//! ```

use crate::units::length::{Au, Kilometer, Length, Meter};
use crate::units::time::{Day, Hour, Second, Time};
use crate::{DivDim, Per, Quantity, Unit};

/// Dimension alias for velocities (`Length / Time`).
pub type Velocity = DivDim<Length, Time>;

/// Marker trait for any unit whose dimension is [`Velocity`].
pub trait VelocityUnit: Unit<Dim = Velocity> {}
impl<T: Unit<Dim = Velocity>> VelocityUnit for T {}


// --- SI and metric velocities ---

/// Metres per second (`m / s`).
///
/// Canonical SI velocity unit.
pub type MeterPerSecond = Per<Meter, Second>;
/// A quantity measured in metres per second.
pub type MetersPerSecond = Quantity<MeterPerSecond>;

/// Kilometres per second (`km / s`).
///
/// Common in astronomy, spaceflight, and high-speed physics.
pub type KilometerPerSecond = Per<Kilometer, Second>;
/// A quantity measured in kilometres per second.
pub type KilometersPerSecond = Quantity<KilometerPerSecond>;

/// Metres per hour (`m / h`).
///
/// Useful for low-speed processes and engineering contexts.
pub type MeterPerHour = Per<Meter, Hour>;
/// A quantity measured in metres per hour.
pub type MetersPerHour = Quantity<MeterPerHour>;

/// Kilometres per hour (`km / h`).
///
/// Common civil and transportation velocity unit.
pub type KilometerPerHour = Per<Kilometer, Hour>;
/// A quantity measured in kilometres per hour.
pub type KilometersPerHour = Quantity<KilometerPerHour>;

/// Metres per day (`m / d`).
///
/// Useful in geoscience, biology, and slow drift phenomena.
pub type MeterPerDay = Per<Meter, Day>;
/// A quantity measured in metres per day.
pub type MetersPerDay = Quantity<MeterPerDay>;

/// Kilometres per day (`km / d`).
///
/// Often used for large-scale migrations and orbital approximations.
pub type KilometerPerDay = Per<Kilometer, Day>;
/// A quantity measured in kilometres per day.
pub type KilometersPerDay = Quantity<KilometerPerDay>;


// --- Astronomical velocities ---

/// Astronomical units per second (`AU / s`).
///
/// Extremely large velocity scale, mainly theoretical.
pub type AuPerSecond = Per<Au, Second>;
/// A quantity measured in astronomical units per second.
pub type AusPerSecond = Quantity<AuPerSecond>;

/// Astronomical units per hour (`AU / h`).
///
/// Occasionally useful for coarse interplanetary motion.
pub type AuPerHour = Per<Au, Hour>;
/// A quantity measured in astronomical units per hour.
pub type AusPerHour = Quantity<AuPerHour>;

/// Astronomical units per day (`AU / d`).
///
/// Common in orbital mechanics and ephemerides.
pub type AuPerDay = Per<Au, Day>;
/// A quantity measured in astronomical units per day.
pub type AusPerDay = Quantity<AuPerDay>;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::length::Kilometers;
    use crate::units::time::Seconds;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic velocity conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn km_per_s_to_m_per_s() {
        let v: KilometersPerSecond = Quantity::new(1.0);
        let v_mps: MetersPerSecond = v.to();
        assert_abs_diff_eq!(v_mps.value(), 1000.0, epsilon = 1e-9);
    }

    #[test]
    fn m_per_s_to_km_per_s() {
        let v: MetersPerSecond = Quantity::new(1000.0);
        let v_kps: KilometersPerSecond = v.to();
        assert_abs_diff_eq!(v_kps.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn km_per_h_to_m_per_s() {
        let v: KilometersPerHour = Quantity::new(3.6);
        let v_mps: MetersPerSecond = v.to();
        // 3.6 km/h = 1 m/s
        assert_abs_diff_eq!(v_mps.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn km_per_h_to_km_per_s() {
        let v: KilometersPerHour = Quantity::new(3600.0);
        let v_kps: KilometersPerSecond = v.to();
        // 3600 km/h = 1 km/s
        assert_abs_diff_eq!(v_kps.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn au_per_day_to_km_per_s() {
        let v: AusPerDay = Quantity::new(1.0);
        let v_kps: KilometersPerSecond = v.to();
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
        let v: KilometersPerSecond = Quantity::new(10.0);
        let t: Seconds = Seconds::new(5.0);
        let d: Kilometers = v * t;
        assert_abs_diff_eq!(d.value(), 50.0, epsilon = 1e-9);
    }

    #[test]
    fn time_times_velocity() {
        let v: KilometersPerSecond = Quantity::new(10.0);
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
        let v: KilometersPerSecond = d / t;
        assert_abs_diff_eq!(v.value(), 10.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_mps_kps() {
        let original: MetersPerSecond = Quantity::new(500.0);
        let converted: KilometersPerSecond = original.to();
        let back: MetersPerSecond = converted.to();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_mps_kps(v in 1e-6..1e6f64) {
            let original: MetersPerSecond = Quantity::new(v);
            let converted: KilometersPerSecond = original.to();
            let back: MetersPerSecond = converted.to();
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * v.abs().max(1.0));
        }

        #[test]
        fn prop_mps_kps_ratio(v in 1e-6..1e6f64) {
            let mps: MetersPerSecond = Quantity::new(v);
            let kps: KilometersPerSecond = mps.to();
            // 1000 m/s = 1 km/s
            prop_assert!((mps.value() / kps.value() - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn prop_velocity_time_roundtrip(
            v_val in 1e-3..1e3f64,
            t_val in 1e-3..1e3f64
        ) {
            let v: KilometersPerSecond = Quantity::new(v_val);
            let t: Seconds = Seconds::new(t_val);
            let d: Kilometers = v * t;
            // d / t should give back v
            let v_back: KilometersPerSecond = d / t;
            prop_assert!((v_back.value() - v.value()).abs() / v.value() < 1e-12);
        }
    }
}
