//! # Velocity Units Module
//!
//! Composite velocity units built from a length over a time. The underlying
//! implementation leverages the generic [`Per<N, D>`] type which composes two
//! phantom unit parameters. This allows the multiplication and division traits
//! to be implemented once for all unit pairs in [`Quantity`](super::Quantity).

use super::*;

/// Dimension alias for velocities (`Length / Time`).
pub type Velocity = DivDim<Length, Time>;

/// Marker trait for any unit with velocity dimension.
pub trait VelocityUnit: Unit<Dim = Velocity> {}
impl<T: Unit<Dim = Velocity>> VelocityUnit for T {}

pub type MeterPerSecond = Per<Meter, Second>;
pub type MetersPerSecond = Quantity<MeterPerSecond>;

pub type KilometerPerSecond = Per<Kilometer, Second>;
pub type KilometersPerSecond = Quantity<KilometerPerSecond>;

pub type KilometerPerHour = Per<Kilometer, Hour>;
pub type KilometersPerHour = Quantity<KilometerPerHour>;

pub type AuPerDay = Per<Au, Day>;
pub type AusPerDay = Quantity<AuPerDay>;

#[cfg(test)]
mod tests {
    use super::*;
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
        // Per<Kilometer, Second> should have RATIO = 1000 / (1/86400) = 86,400,000
        let ratio = <Per<Kilometer, Second>>::RATIO;
        // Kilometer::RATIO = 1000, Second::RATIO = 1/86400
        // So Per ratio = 1000 / (1/86400) = 1000 * 86400 = 86,400,000
        assert_relative_eq!(ratio, 1000.0 / (1.0 / 86400.0), max_relative = 1e-12);
    }

    #[test]
    fn per_ratio_m_s() {
        // Per<Meter, Second> has RATIO = 1 / (1/86400) = 86400
        let ratio = <Per<Meter, Second>>::RATIO;
        assert_relative_eq!(ratio, 86400.0, max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Velocity * Time = Length
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn km_per_s_times_seconds_equals_km() {
        let velocity: KilometersPerSecond = Quantity::new(10.0);
        let time = Seconds::new(5.0);
        let distance: Kilometers = velocity * time;
        assert_abs_diff_eq!(distance.value(), 50.0, epsilon = 1e-12);
    }

    #[test]
    fn m_per_s_times_seconds_equals_m() {
        let velocity: MetersPerSecond = Quantity::new(100.0);
        let time = Seconds::new(2.5);
        let distance: Meters = velocity * time;
        assert_abs_diff_eq!(distance.value(), 250.0, epsilon = 1e-12);
    }

    #[test]
    fn multiplication_commutative() {
        let velocity: KilometersPerSecond = Quantity::new(5.0);
        let time = Seconds::new(10.0);
        let d1: Kilometers = velocity * time;
        let d2: Kilometers = time * velocity;
        assert_abs_diff_eq!(d1.value(), d2.value(), epsilon = 1e-12);
    }

    #[test]
    fn au_per_day_times_days_equals_au() {
        let velocity: AusPerDay = Quantity::new(0.5);
        let time = Days::new(10.0);
        let distance: AstronomicalUnits = velocity * time;
        assert_abs_diff_eq!(distance.value(), 5.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Length / Time = Velocity
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn km_divided_by_s_equals_km_per_s() {
        let distance = Kilometers::new(100.0);
        let time = Seconds::new(10.0);
        let velocity: KilometersPerSecond = distance / time;
        assert_abs_diff_eq!(velocity.value(), 10.0, epsilon = 1e-12);
    }

    #[test]
    fn m_divided_by_s_equals_m_per_s() {
        let distance = Meters::new(500.0);
        let time = Seconds::new(5.0);
        let velocity: MetersPerSecond = distance / time;
        assert_abs_diff_eq!(velocity.value(), 100.0, epsilon = 1e-12);
    }

    #[test]
    fn au_divided_by_day_equals_au_per_day() {
        let distance = AstronomicalUnits::new(1.0);
        let time = Days::new(2.0);
        let velocity: AusPerDay = distance / time;
        assert_abs_diff_eq!(velocity.value(), 0.5, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip: distance -> velocity -> distance
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_distance_velocity_distance() {
        let original_dist = Kilometers::new(1000.0);
        let time = Seconds::new(50.0);
        let velocity: KilometersPerSecond = original_dist / time;
        let recovered_dist: Kilometers = velocity * time;
        assert_abs_diff_eq!(
            recovered_dist.value(),
            original_dist.value(),
            epsilon = 1e-12
        );
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display formatting for Per types
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_km_per_s() {
        let v: KilometersPerSecond = Quantity::new(42.5);
        let s = format!("{}", v);
        assert_eq!(s, "42.5 \"Km\"/\"sec\"");
    }

    #[test]
    fn display_m_per_s() {
        let v: MetersPerSecond = Quantity::new(100.0);
        let s = format!("{}", v);
        assert_eq!(s, "100 \"m\"/\"sec\"");
    }

    #[test]
    fn display_au_per_day() {
        let v: AusPerDay = Quantity::new(0.017);
        let s = format!("{}", v);
        assert_eq!(s, "0.017 \"Au\"/\"d\"");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Real-world velocities
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn speed_of_light_approx() {
        // c ≈ 299,792 km/s
        let c: KilometersPerSecond = Quantity::new(299_792.458);
        let c_mps: MetersPerSecond = c.to();
        assert_relative_eq!(c_mps.value(), 299_792_458.0, max_relative = 1e-6);
    }

    #[test]
    fn earth_orbital_velocity() {
        // Earth's orbital velocity ≈ 29.78 km/s
        let earth_v: KilometersPerSecond = Quantity::new(29.78);
        let earth_v_mps: MetersPerSecond = earth_v.to();
        assert_relative_eq!(earth_v_mps.value(), 29780.0, max_relative = 1e-3);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn zero_velocity() {
        let v: KilometersPerSecond = Quantity::new(0.0);
        let v2: MetersPerSecond = v.to();
        assert_eq!(v2.value(), 0.0);
    }

    #[test]
    fn negative_velocity() {
        let v: KilometersPerSecond = Quantity::new(-10.0);
        let v2: MetersPerSecond = v.to();
        assert_abs_diff_eq!(v2.value(), -10000.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_km_s_to_m_s_ratio(v in -1e6..1e6f64) {
            let km_s: KilometersPerSecond = Quantity::new(v);
            let m_s: MetersPerSecond = km_s.to();
            assert_relative_eq!(m_s.value(), v * 1000.0, max_relative = 1e-12);
        }

        #[test]
        fn prop_velocity_time_product(v in -1e6..1e6f64, t in 1e-6..1e6f64) {
            let velocity: KilometersPerSecond = Quantity::new(v);
            let time = Seconds::new(t);
            let distance: Kilometers = velocity * time;
            assert_relative_eq!(distance.value(), v * t, max_relative = 1e-12);
        }

        #[test]
        fn prop_roundtrip_dist_vel_dist(d in 1e-6..1e6f64, t in 1e-6..1e6f64) {
            let dist = Kilometers::new(d);
            let time = Seconds::new(t);
            let vel: KilometersPerSecond = dist / time;
            let recovered: Kilometers = vel * time;
            assert_relative_eq!(recovered.value(), dist.value(), max_relative = 1e-12);
        }

        #[test]
        fn prop_conversion_roundtrip(v in -1e6..1e6f64) {
            let orig: KilometersPerSecond = Quantity::new(v);
            let converted: MetersPerSecond = orig.to();
            let back: KilometersPerSecond = converted.to();
            assert_relative_eq!(back.value(), orig.value(), max_relative = 1e-12);
        }
    }
}
