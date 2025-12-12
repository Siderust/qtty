//! # Frequency Units Module
//!
//! Frequency units are represented as a ratio between an angular measure and a
//! time unit using the generic [`Per<N, D>`] type. This enables automatic
//! multiplication and division without bespoke implementations for every pair.

use super::*;

/// Dimension alias for angular frequency (`Angular / Time`).
pub type Frequency = DivDim<Angular, Time>;

/// Marker trait for any unit with frequency dimension.
pub trait FrequencyUnit: Unit<Dim = Frequency> {}
impl<T: Unit<Dim = Frequency>> FrequencyUnit for T {}

pub type RadianPerDay = Per<Radian, Day>;
pub type RadiansPerDay = Quantity<RadianPerDay>;

pub type DegreePerDay = Per<Degree, Day>;
pub type DegreesPerDay = Quantity<DegreePerDay>;

pub type DegreePerYear = Per<Degree, Year>;
pub type DegreesPerYear = Quantity<DegreePerYear>;

pub type MilliArcsecondPerDay = Per<MilliArcsecond, Day>;
pub type MilliArcsecondsPerDay = Quantity<MilliArcsecondPerDay>;

#[cfg(test)]
mod tests {
    use super::*;
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
        // Degree::RATIO = 1.0, Day::RATIO = 1.0
        // So Per<Degree, Day>::RATIO = 1.0 / 1.0 = 1.0
        let ratio = <Per<Degree, Day>>::RATIO;
        assert_abs_diff_eq!(ratio, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn per_ratio_rad_day() {
        // Radian::RATIO = 180/π, Day::RATIO = 1.0
        let ratio = <Per<Radian, Day>>::RATIO;
        assert_relative_eq!(ratio, 180.0 / PI, max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Frequency * Time = Angle
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn deg_per_day_times_days_equals_degrees() {
        let freq: DegreesPerDay = Quantity::new(10.0);
        let time = Days::new(36.0);
        let angle: Degrees = freq * time;
        assert_abs_diff_eq!(angle.value(), 360.0, epsilon = 1e-12);
    }

    #[test]
    fn rad_per_day_times_days_equals_radians() {
        let freq: RadiansPerDay = Quantity::new(PI / 2.0);
        let time = Days::new(4.0);
        let angle: Radians = freq * time;
        assert_abs_diff_eq!(angle.value(), 2.0 * PI, epsilon = 1e-12);
    }

    #[test]
    fn multiplication_commutative() {
        let freq: DegreesPerDay = Quantity::new(15.0);
        let time = Days::new(24.0);
        let a1: Degrees = freq * time;
        let a2: Degrees = time * freq;
        assert_abs_diff_eq!(a1.value(), a2.value(), epsilon = 1e-12);
    }

    #[test]
    fn mas_per_day_times_days_equals_mas() {
        let freq: MilliArcsecondsPerDay = Quantity::new(100.0);
        let time = Days::new(10.0);
        let angle: MilliArcseconds = freq * time;
        assert_abs_diff_eq!(angle.value(), 1000.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Angle / Time = Frequency
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn degrees_divided_by_days_equals_deg_per_day() {
        let angle = Degrees::new(360.0);
        let time = Days::new(10.0);
        let freq: DegreesPerDay = angle / time;
        assert_abs_diff_eq!(freq.value(), 36.0, epsilon = 1e-12);
    }

    #[test]
    fn radians_divided_by_days_equals_rad_per_day() {
        let angle = Radians::new(2.0 * PI);
        let time = Days::new(1.0);
        let freq: RadiansPerDay = angle / time;
        assert_abs_diff_eq!(freq.value(), 2.0 * PI, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip: angle -> frequency -> angle
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_angle_freq_angle() {
        let original_angle = Degrees::new(720.0);
        let time = Days::new(20.0);
        let freq: DegreesPerDay = original_angle / time;
        let recovered: Degrees = freq * time;
        assert_abs_diff_eq!(recovered.value(), original_angle.value(), epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display formatting
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_deg_per_day() {
        let f: DegreesPerDay = Quantity::new(0.985);
        let s = format!("{}", f);
        assert_eq!(s, "0.985 \"Deg\"/\"d\"");
    }

    #[test]
    fn display_rad_per_day() {
        let f: RadiansPerDay = Quantity::new(0.017);
        let s = format!("{}", f);
        assert_eq!(s, "0.017 \"Rad\"/\"d\"");
    }

    #[test]
    fn display_mas_per_day() {
        let f: MilliArcsecondsPerDay = Quantity::new(50.0);
        let s = format!("{}", f);
        assert_eq!(s, "50 \"Mas\"/\"d\"");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Astronomical application: Earth's rotation
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn earth_rotation_rate() {
        // Earth rotates 360°/day (sidereal day is slightly different)
        let earth_rot: DegreesPerDay = Quantity::new(360.0);
        let in_rad: RadiansPerDay = earth_rot.to();
        assert_abs_diff_eq!(in_rad.value(), 2.0 * PI, epsilon = 1e-12);
    }

    #[test]
    fn earth_mean_motion() {
        // Earth's mean motion ≈ 0.9856°/day
        let mean_motion: DegreesPerDay = Quantity::new(0.9856);
        let per_year: DegreesPerYear = mean_motion.to();
        // Should be about 360°/year
        assert_relative_eq!(per_year.value(), 360.0, max_relative = 0.01);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn zero_frequency() {
        let f: DegreesPerDay = Quantity::new(0.0);
        let f2: RadiansPerDay = f.to();
        assert_eq!(f2.value(), 0.0);
    }

    #[test]
    fn negative_frequency() {
        // Retrograde motion
        let f: DegreesPerDay = Quantity::new(-0.5);
        let f2: RadiansPerDay = f.to();
        assert!(f2.value() < 0.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_deg_rad_conversion(f in -360.0..360.0f64) {
            let deg: DegreesPerDay = Quantity::new(f);
            let rad: RadiansPerDay = deg.to();
            assert_relative_eq!(rad.value(), f * PI / 180.0, max_relative = 1e-12);
        }

        #[test]
        fn prop_freq_time_product(f in -360.0..360.0f64, t in 1e-6..1e3f64) {
            let freq: DegreesPerDay = Quantity::new(f);
            let time = Days::new(t);
            let angle: Degrees = freq * time;
            assert_relative_eq!(angle.value(), f * t, max_relative = 1e-12);
        }

        #[test]
        fn prop_roundtrip_angle_freq(a in -1e6..1e6f64, t in 1e-6..1e6f64) {
            let angle = Degrees::new(a);
            let time = Days::new(t);
            let freq: DegreesPerDay = angle / time;
            let recovered: Degrees = freq * time;
            assert_relative_eq!(recovered.value(), angle.value(), max_relative = 1e-12);
        }

        #[test]
        fn prop_conversion_roundtrip(f in -360.0..360.0f64) {
            let orig: DegreesPerDay = Quantity::new(f);
            let converted: RadiansPerDay = orig.to();
            let back: DegreesPerDay = converted.to();
            assert_relative_eq!(back.value(), orig.value(), max_relative = 1e-12);
        }
    }
}
