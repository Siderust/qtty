//! Integration-level smoke tests for the `qtty` facade crate.

use qtty::*;

use approx::{assert_abs_diff_eq, assert_relative_eq};

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
    let v: velocity::Velocity<Kilometer, Second> = velocity::Velocity::new(1.0);
    let v_mps: velocity::Velocity<Meter, Second> = v.to();
    assert_abs_diff_eq!(v_mps.value(), 1000.0, epsilon = 1e-9);
}

#[test]
fn smoke_test_frequency() {
    let f: frequency::Frequency<Degree, Day> = frequency::Frequency::new(360.0);
    let f_rad: frequency::Frequency<Radian, Day> = f.to();
    assert_abs_diff_eq!(f_rad.value(), 2.0 * std::f64::consts::PI, epsilon = 1e-12);
}

#[test]
fn smoke_test_unitless() {
    let m = Meters::new(42.0);
    let u: Quantity<Unitless> = m.into();
    assert_eq!(u.value(), 42.0);
}

#[test]
fn orbital_distance_calculation() {
    // Earth's orbital velocity ≈ 29.78 km/s
    let earth_velocity: velocity::Velocity<Kilometer, Second> = velocity::Velocity::new(29.78);

    // Time: 1 day
    let time = Days::new(1.0);
    let time_sec: Seconds = time.to();

    // Distance = velocity × time
    let distance: Kilometers = (earth_velocity * time_sec).to();

    // Earth travels about 2.57 million km per day
    assert_relative_eq!(distance.value(), 2_573_395.2, max_relative = 1e-3);
}

#[test]
fn proxima_centauri_distance() {
    // Proxima Centauri is about 4.24 light years away
    let distance_ly = LightYears::new(4.24);

    // Convert to AU
    let distance_au: AstronomicalUnits = distance_ly.to();

    // Should be about 268,000 AU
    assert_relative_eq!(distance_au.value(), 268_000.0, max_relative = 0.01);
}

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

#[test]
fn earth_rotation() {
    // Earth rotates 360° per sidereal day (~23h 56m)
    let rotation_rate: frequency::Frequency<Degree, Day> = frequency::Frequency::new(360.0);

    // After 6 hours (0.25 days)
    let time = Days::new(0.25);
    let angle: Degrees = (rotation_rate * time).to();

    assert_abs_diff_eq!(angle.value(), 90.0, epsilon = 1e-12);
}

#[test]
fn sun_mass() {
    let sun = SolarMasses::new(1.0);
    let kg: Kilograms = sun.to();

    // Sun's mass is about 2e30 kg
    assert_relative_eq!(kg.value(), 1.988416e30, max_relative = 1e-5);
}

#[test]
fn sun_luminosity() {
    let sun = SolarLuminosities::new(1.0);
    let watts: Watts = sun.to();

    // Sun's luminosity is about 3.828e26 W
    assert_relative_eq!(watts.value(), 3.828e26, max_relative = 1e-9);
}

#[test]
fn calculate_velocity_from_distance_time() {
    // Light year to km
    let distance = LightYears::new(1.0);
    let distance_km: Kilometers = distance.to();

    // Julian year to seconds
    let time = JulianYears::new(1.0);
    let time_sec: Seconds = time.to();

    // Velocity = distance / time
    let velocity: velocity::Velocity<Kilometer, Second> = distance_km / time_sec;

    // Should be approximately speed of light (299,792 km/s)
    assert_relative_eq!(velocity.value(), 299_792.458, max_relative = 0.001);
}

#[test]
fn mean_motion_conversion() {
    // Earth's mean motion ≈ 0.9856°/day
    let mean_motion: frequency::Frequency<Degree, Day> = frequency::Frequency::new(0.9856);

    // Convert to degrees per year
    let per_year: frequency::Frequency<Degree, Year> = mean_motion.to();

    // Should be about 360°/year
    assert_relative_eq!(per_year.value(), 360.0, max_relative = 0.01);
}

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

#[test]
fn derive_macro_produces_correct_symbol() {
    // Verify that units defined with derive macro have correct symbols
    assert_eq!(Meter::SYMBOL, "m");
    assert_eq!(Kilometer::SYMBOL, "km");
    assert_eq!(Second::SYMBOL, "s");
    assert_eq!(Day::SYMBOL, "d");
    assert_eq!(Degree::SYMBOL, "°");
    assert_eq!(Radian::SYMBOL, "rad");
}

#[test]
fn derive_macro_produces_correct_ratio() {
    // Verify ratios are correct
    assert_eq!(Meter::RATIO, 1.0);
    assert_eq!(Kilometer::RATIO, 1000.0);
    assert_eq!(Second::RATIO, 1.0);
    assert_eq!(Degree::RATIO, 1.0);
}

#[test]
fn derive_macro_display_formatting() {
    let m = Meters::new(42.0);
    assert_eq!(format!("{}", m), "42 m");

    let km = Kilometers::new(1.5);
    assert_eq!(format!("{}", km), "1.5 km");

    let deg = Degrees::new(90.0);
    assert_eq!(format!("{}", deg), "90 °");
}

#[test]
fn quantity_basic_arithmetic() {
    let a = Meters::new(10.0);
    let b = Meters::new(5.0);

    assert_eq!((a + b).value(), 15.0);
    assert_eq!((a - b).value(), 5.0);
    assert_eq!((a * 2.0).value(), 20.0);
    assert_eq!((a / 2.0).value(), 5.0);
}

#[test]
fn quantity_conversion_chain() {
    // Convert through multiple units
    let au = AstronomicalUnits::new(1.0);
    let km: Kilometers = au.to();
    let m: Meters = km.to();

    // Direct conversion should match
    let m_direct: Meters = au.to();
    assert_abs_diff_eq!(m.value(), m_direct.value(), epsilon = 1e-3);
}

#[test]
fn quantity_negation() {
    let pos = Degrees::new(45.0);
    let neg = -pos;
    assert_eq!(neg.value(), -45.0);
}

#[test]
fn quantity_abs() {
    let neg = Degrees::new(-45.0);
    assert_eq!(neg.abs().value(), 45.0);
}

#[test]
fn per_unit_display() {
    let v: velocity::Velocity<Kilometer, Second> = velocity::Velocity::new(10.0);
    let s = format!("{}", v);
    assert_eq!(s, "10 km/s");
}

#[test]
fn per_unit_multiplication_recovers_numerator() {
    let v: velocity::Velocity<Kilometer, Second> = velocity::Velocity::new(100.0);
    let t: Seconds = Seconds::new(3600.0);
    let d: Kilometers = (v * t).to();
    assert_abs_diff_eq!(d.value(), 360_000.0, epsilon = 1e-6);
}

#[test]
fn qtty_values_macro_builds_seconds_array() {
    const DT: [Seconds; 3] = qtty::qtty_vec!(Seconds; 56.86, 63.83, 70.0);
    assert_abs_diff_eq!(DT[0].value(), 56.86, epsilon = 1e-12);
    assert_abs_diff_eq!(DT[1].value(), 63.83, epsilon = 1e-12);
    assert_abs_diff_eq!(DT[2].value(), 70.0, epsilon = 1e-12);
}

#[test]
fn qtty_values_macro_builds_seconds_vec() {
    let dt: Vec<Seconds> = qtty::qtty_vec!(vec Seconds; 56.86, 63.83, 70.0);
    assert_eq!(dt.len(), 3);
    assert_abs_diff_eq!(dt[1].value(), 63.83, epsilon = 1e-12);
}

#[test]
fn per_unit_division_creates_composite() {
    let d = Kilometers::new(100.0);
    let t = Seconds::new(10.0);
    let v: velocity::Velocity<Kilometer, Second> = d / t;
    assert_abs_diff_eq!(v.value(), 10.0, epsilon = 1e-12);
}

#[test]
fn unit_constants_have_value_one() {
    assert_eq!(AU.value(), 1.0);
    assert_eq!(LY.value(), 1.0);
    assert_eq!(KM.value(), 1.0);
    assert_eq!(DAY.value(), 1.0);
    assert_eq!(SEC.value(), 1.0);
    assert_eq!(DEG.value(), 1.0);
    assert_eq!(RAD.value(), 1.0);
}

#[test]
fn constants_can_be_multiplied() {
    let distance = 4.24 * LY;
    assert_eq!(distance.value(), 4.24);

    let time = 365.25 * DAY;
    assert_eq!(time.value(), 365.25);
}

#[test]
fn macro_generated_conversions() {
    // Test conversions that are now generated by impl_unit_conversions! macro
    // These weren't manually implemented before

    // Meter -> AstronomicalUnit (AU is exactly 149,597,870,700 m)
    let m = Meters::new(149_597_870_700.0);
    let au: AstronomicalUnits = m.into();
    assert_relative_eq!(au.value(), 1.0, max_relative = 1e-12);

    // Nominal SolarRadius -> Kilometer
    use qtty_core::length::nominal::SolarRadiuses;
    let sr = SolarRadiuses::new(1.0);
    let km: Kilometers = sr.into();
    assert_abs_diff_eq!(km.value(), 695_700.0, epsilon = 1e-6);

    // Parsec -> AstronomicalUnit
    let pc = Parsecs::new(1.0);
    let au: AstronomicalUnits = pc.into();
    // 1 pc = au * 648000 / π
    let expected = 648_000.0 / core::f64::consts::PI;
    assert_relative_eq!(au.value(), expected, max_relative = 1e-12);
}

#[test]
fn new_angular_units() {
    // Test the new angular units added via impl_unit_conversions! macro

    // Arcminute conversions
    let deg = Degrees::new(1.0);
    let arcm: Arcminutes = deg.into();
    assert_abs_diff_eq!(arcm.value(), 60.0, epsilon = 1e-12);

    // Microarcsecond conversions
    let arcs = Arcseconds::new(1.0);
    let uas: MicroArcseconds = arcs.into();
    assert_abs_diff_eq!(uas.value(), 1_000_000.0, epsilon = 1e-6);

    // Gradian conversions (1 full turn = 400 gradians)
    let turn = Turns::new(1.0);
    let gon: Gradians = turn.into();
    assert_abs_diff_eq!(gon.value(), 400.0, epsilon = 1e-12);

    // Turn conversions
    let deg = Degrees::new(180.0);
    let turn: Turns = deg.into();
    assert_abs_diff_eq!(turn.value(), 0.5, epsilon = 1e-12);

    // Test trig functions work with new units
    let right_angle = Gradians::new(100.0); // 90 degrees
    assert_abs_diff_eq!(right_angle.sin(), 1.0, epsilon = 1e-12);
    assert_abs_diff_eq!(right_angle.cos(), 0.0, epsilon = 1e-12);

    // Test wrapping with new units
    let turn = Turns::new(2.7);
    let wrapped = turn.wrap_pos();
    assert_abs_diff_eq!(wrapped.value(), 0.7, epsilon = 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// f32 scalar type tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn smoke_test_f32_angular() {
    use qtty::f32::{Degrees, Radians};

    let deg = Degrees::new(180.0_f32);
    let rad: Radians = deg.to();
    assert!((rad.value() - core::f32::consts::PI).abs() < 1e-5);
}

#[test]
fn smoke_test_f32_length() {
    use qtty::f32::{Kilometers, Meters};

    let km = Kilometers::new(1.0_f32);
    let m: Meters = km.to();
    assert!((m.value() - 1000.0).abs() < 1e-4);
}

#[test]
fn smoke_test_f32_time() {
    use qtty::f32::{Days, Seconds};

    let day = Days::new(1.0_f32);
    let sec: Seconds = day.to();
    assert!((sec.value() - 86400.0).abs() < 1.0);
}

#[test]
fn smoke_test_f32_arithmetic() {
    use qtty::f32::Meters;

    let a = Meters::new(10.0_f32);
    let b = Meters::new(5.0_f32);
    let sum = a + b;
    let diff = a - b;

    assert!((sum.value() - 15.0).abs() < 1e-6);
    assert!((diff.value() - 5.0).abs() < 1e-6);
}

#[test]
fn smoke_test_f32_trig() {
    use qtty::f32::Degrees;

    let angle = Degrees::new(90.0_f32);
    assert!((angle.sin() - 1.0).abs() < 1e-5);
    assert!(angle.cos().abs() < 1e-5);
}

// ─────────────────────────────────────────────────────────────────────────────
// Integer scalar module tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn smoke_test_i8_module_arithmetic_and_lossy_conversion() {
    use qtty::i8::{Meters, Minutes, Seconds};

    let a = Meters::new(50);
    let b = Meters::new(20);
    assert_eq!((a + b).value(), 70);
    assert_eq!((a - b).value(), 30);

    let sec = Seconds::new(125);
    let min: Minutes = sec.to_lossy();
    assert_eq!(min.value(), 2);
}

#[test]
fn smoke_test_i16_module_arithmetic_and_lossy_conversion() {
    use qtty::i16::{Kilometers, Meters};

    let a = Meters::new(1_500);
    let b = Meters::new(250);
    assert_eq!((a + b).value(), 1_750);

    let km: Kilometers = a.to_lossy();
    assert_eq!(km.value(), 1);
}

#[test]
fn smoke_test_i128_module_arithmetic_and_lossy_conversion() {
    use qtty::i128::{Days, Seconds};

    let a = Seconds::new(172_800);
    let b = Seconds::new(3_600);
    assert_eq!((a + b).value(), 176_400);

    let days: Days = a.to_lossy();
    assert_eq!(days.value(), 2);
}
