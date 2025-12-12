//! Integration tests for siderust-units
//!
//! These tests exercise the public API across modules, simulating real-world usage
//! patterns and ensuring the prelude-style imports work correctly.

use siderust_units::*;

use approx::{assert_abs_diff_eq, assert_relative_eq};

// ─────────────────────────────────────────────────────────────────────────────────
// Smoke tests: basic functionality across all modules
// ─────────────────────────────────────────────────────────────────────────────────

#[test]
fn smoke_test_angular() {
    let deg = Degrees::new(180.0);
    let rad: Radians = deg.to();
    assert_abs_diff_eq!(rad.value(), std::f64::consts::PI, epsilon = 1e-12);
}

#[test]
fn smoke_test_time() {
    let day = Days::new(1.0);
    let sec: Seconds = day.to();
    assert_abs_diff_eq!(sec.value(), 86400.0, epsilon = 1e-9);
}

#[test]
fn smoke_test_length() {
    let km = Kilometers::new(1.0);
    let m: Meters = km.to();
    assert_abs_diff_eq!(m.value(), 1000.0, epsilon = 1e-9);
}

#[test]
fn smoke_test_mass() {
    let kg = Kilograms::new(1000.0);
    let g: Grams = kg.to();
    assert_abs_diff_eq!(g.value(), 1_000_000.0, epsilon = 1e-6);
}

#[test]
fn smoke_test_power() {
    let sol = SolarLuminosities::new(1.0);
    let w: Watts = sol.to();
    assert_relative_eq!(w.value(), 3.828e26, max_relative = 1e-9);
}

#[test]
fn smoke_test_velocity() {
    let v: KilometersPerSecond = Quantity::new(1.0);
    let v_mps: MetersPerSecond = v.to();
    assert_abs_diff_eq!(v_mps.value(), 1000.0, epsilon = 1e-9);
}

#[test]
fn smoke_test_frequency() {
    let f: DegreesPerDay = Quantity::new(360.0);
    let f_rad: RadiansPerDay = f.to();
    assert_abs_diff_eq!(f_rad.value(), 2.0 * std::f64::consts::PI, epsilon = 1e-12);
}

#[test]
fn smoke_test_unitless() {
    let m = Meters::new(42.0);
    let u: Quantity<Unitless> = m.into();
    assert_eq!(u.value(), 42.0);
}

// ─────────────────────────────────────────────────────────────────────────────────
// Real-world astronomical calculations
// ─────────────────────────────────────────────────────────────────────────────────

/// Calculate distance traveled given velocity and time
#[test]
fn orbital_distance_calculation() {
    // Earth's orbital velocity ≈ 29.78 km/s
    let earth_velocity: KilometersPerSecond = Quantity::new(29.78);

    // Time: 1 day
    let time = Days::new(1.0);
    let time_sec: Seconds = time.to();

    // Distance = velocity × time
    let distance: Kilometers = earth_velocity * time_sec;

    // Earth travels about 2.57 million km per day
    assert_relative_eq!(distance.value(), 2_573_395.2, max_relative = 1e-3);
}

/// Convert astronomical distances
#[test]
fn proxima_centauri_distance() {
    // Proxima Centauri is about 4.24 light years away
    let distance_ly = LightYears::new(4.24);

    // Convert to AU
    let distance_au: AstronomicalUnits = distance_ly.to();

    // Should be about 268,000 AU
    assert_relative_eq!(distance_au.value(), 268_000.0, max_relative = 0.01);
}

/// Angular separation calculation
#[test]
fn angular_separation() {
    // Two stars at different positions
    let star1_ra = Degrees::new(45.0);
    let star2_ra = Degrees::new(350.0);

    // Separation should wrap around
    let sep = star1_ra.abs_separation(star2_ra);

    // 45° to 350° is 55° the short way
    assert_abs_diff_eq!(sep.value(), 55.0, epsilon = 1e-12);
}

/// Earth's rotation calculation
#[test]
fn earth_rotation() {
    // Earth rotates 360° per sidereal day (~23h 56m)
    let rotation_rate: DegreesPerDay = Quantity::new(360.0);

    // After 6 hours (0.25 days)
    let time = Days::new(0.25);
    let angle: Degrees = rotation_rate * time;

    assert_abs_diff_eq!(angle.value(), 90.0, epsilon = 1e-12);
}

/// Solar mass to kg conversion
#[test]
fn sun_mass() {
    let sun = SolarMasses::new(1.0);
    let kg: Kilograms = sun.to();

    // Sun's mass is about 2e30 kg
    assert_relative_eq!(kg.value(), 1.98847e30, max_relative = 1e-5);
}

/// Solar luminosity to watts
#[test]
fn sun_luminosity() {
    let sun = SolarLuminosities::new(1.0);
    let watts: Watts = sun.to();

    // Sun's luminosity is about 3.828e26 W
    assert_relative_eq!(watts.value(), 3.828e26, max_relative = 1e-9);
}

// ─────────────────────────────────────────────────────────────────────────────────
// Cross-module interactions
// ─────────────────────────────────────────────────────────────────────────────────

/// Velocity from distance and time
#[test]
fn calculate_velocity_from_distance_time() {
    // Light year to km
    let distance = LightYears::new(1.0);
    let distance_km: Kilometers = distance.to();

    // Julian year to seconds
    let time = JulianYears::new(1.0);
    let time_sec: Seconds = time.to();

    // Velocity = distance / time
    let velocity: KilometersPerSecond = distance_km / time_sec;

    // Should be approximately speed of light (299,792 km/s)
    assert_relative_eq!(velocity.value(), 299_792.458, max_relative = 0.001);
}

/// Mean motion calculation (deg/day to deg/year)
#[test]
fn mean_motion_conversion() {
    // Earth's mean motion ≈ 0.9856°/day
    let mean_motion: DegreesPerDay = Quantity::new(0.9856);

    // Convert to degrees per year
    let per_year: DegreesPerYear = mean_motion.to();

    // Should be about 360°/year
    assert_relative_eq!(per_year.value(), 360.0, max_relative = 0.01);
}

/// Trigonometry on angles
#[test]
fn trigonometric_calculation() {
    // 30° angle
    let angle = Degrees::new(30.0);

    // sin(30°) = 0.5
    assert_abs_diff_eq!(angle.sin(), 0.5, epsilon = 1e-12);

    // cos(30°) = √3/2
    assert_abs_diff_eq!(angle.cos(), 3.0_f64.sqrt() / 2.0, epsilon = 1e-12);

    // tan(30°) = 1/√3
    assert_abs_diff_eq!(angle.tan(), 1.0 / 3.0_f64.sqrt(), epsilon = 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────────
// Unit constant arithmetic
// ─────────────────────────────────────────────────────────────────────────────────

#[test]
fn unit_constant_multiplication() {
    // 5 AU
    let five_au = 5.0 * AU;
    assert_eq!(five_au.value(), 5.0);

    // 3 days
    let three_days = 3.0 * DAY;
    assert_eq!(three_days.value(), 3.0);

    // 2 kg
    let two_kg = 2.0 * KG;
    assert_eq!(two_kg.value(), 2.0);
}

#[test]
fn unit_constant_division() {
    let half_ly = LY / 2.0;
    assert_eq!(half_ly.value(), 0.5);
}

#[test]
fn unit_constant_addition() {
    let combined = DAY + DAY;
    assert_eq!(combined.value(), 2.0);
}

// ─────────────────────────────────────────────────────────────────────────────────
// Composite unit behavior
// ─────────────────────────────────────────────────────────────────────────────────

/// Per<N,D> multiplication recovers numerator type
#[test]
fn per_multiplication_type_recovery() {
    let velocity: KilometersPerSecond = Quantity::new(10.0);
    let time = Seconds::new(5.0);

    // (km/s) * s = km
    let distance: Kilometers = velocity * time;
    assert_eq!(distance.value(), 50.0);
}

/// Simplify Per<U,U> to Unitless
#[test]
fn simplify_same_unit_ratio() {
    let km1 = Kilometers::new(100.0);
    let km2 = Kilometers::new(50.0);

    let ratio: Quantity<Per<Kilometer, Kilometer>> = km1 / km2;
    let unitless: Quantity<Unitless> = ratio.simplify();

    assert_eq!(unitless.value(), 2.0);
}

/// asin on unitless ratio
#[test]
fn asin_on_ratio() {
    let m1 = Meters::new(1.0);
    let m2 = Meters::new(2.0);

    let ratio: Quantity<Per<Meter, Meter>> = m1 / m2;
    let angle = ratio.asin();

    // asin(0.5) = π/6
    assert_abs_diff_eq!(angle, std::f64::consts::FRAC_PI_6, epsilon = 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────────
// Display formatting verification
// ─────────────────────────────────────────────────────────────────────────────────

#[test]
fn display_simple_units() {
    assert!(format!("{}", Degrees::new(45.0)).contains("45"));
    assert!(format!("{}", Meters::new(100.0)).contains("100"));
    assert!(format!("{}", Days::new(7.0)).contains("7"));
}

#[test]
fn display_composite_units() {
    let v: KilometersPerSecond = Quantity::new(10.0);
    let s = format!("{}", v);
    // Should contain both numerator and denominator symbols
    assert!(s.contains("10"));
    assert!(s.contains("/"));
}

#[test]
fn display_unitless() {
    let u: Quantity<Unitless> = Quantity::new(1.23456);
    let s = format!("{}", u);
    // Unitless should just show the number
    assert!(s.contains("1.23456"));
}

// ─────────────────────────────────────────────────────────────────────────────────
// Edge cases and boundary conditions
// ─────────────────────────────────────────────────────────────────────────────────

#[test]
fn zero_values() {
    let zero_deg = Degrees::new(0.0);
    let zero_m = Meters::new(0.0);
    let zero_s = Seconds::new(0.0);

    assert_eq!(zero_deg.to::<Radian>().value(), 0.0);
    assert_eq!(zero_m.to::<Kilometer>().value(), 0.0);
    assert_eq!(zero_s.to::<Day>().value(), 0.0);
}

#[test]
fn negative_values() {
    let neg_deg = Degrees::new(-45.0);
    let neg_m = Meters::new(-100.0);

    // Negative angles convert correctly
    let neg_rad: Radians = neg_deg.to();
    assert!(neg_rad.value() < 0.0);

    // Negative lengths convert correctly
    let neg_km: Kilometers = neg_m.to();
    assert!(neg_km.value() < 0.0);
}

#[test]
fn wrap_boundary_180() {
    // Exactly 180° should be handled correctly
    let angle = Degrees::new(180.0);
    let wrapped = angle.wrap_signed();
    assert_abs_diff_eq!(wrapped.value(), 180.0, epsilon = 1e-12);
}

#[test]
fn wrap_boundary_360() {
    let angle = Degrees::new(360.0);
    let wrapped = angle.wrap_pos();
    assert_abs_diff_eq!(wrapped.value(), 0.0, epsilon = 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────────
// Doctest verification (ensures module docs compile and work)
// ─────────────────────────────────────────────────────────────────────────────────

/// This test mirrors the main lib.rs doctest example
#[test]
fn doctest_example() {
    // Angular Units
    let degrees = Degrees::new(180.0);
    let radians = degrees.to::<Radian>();
    let dms = Degrees::from_dms(12, 34, 56.0);

    // Mass Units
    let mass_kg = Kilograms::new(5.0);
    let _mass_solar = SolarMasses::new(2.0);

    // Conversions
    let _dms_to_decimal = dms.value();

    assert_eq!(radians.value(), std::f64::consts::PI);
    assert_eq!(mass_kg.value(), 5.0);
}
