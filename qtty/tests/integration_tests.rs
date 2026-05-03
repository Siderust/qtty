// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

#![cfg(feature = "std")]
//! Integration-level smoke tests for the `qtty` facade crate.

use qtty::*;

use approx::{assert_abs_diff_eq, assert_relative_eq};

#[test]
fn smoke_test_angular() {
    let deg = Degree::new(180.0);
    let rad: Radian = deg.to();
    assert_abs_diff_eq!(rad.value(), std::f64::consts::PI, epsilon = 1e-12);
}

#[test]
fn smoke_test_time() {
    let day = Day::new(1.0);
    let sec: Second = day.to();
    assert_abs_diff_eq!(sec.value(), 86400.0, epsilon = 1e-9);
}

#[test]
fn smoke_test_length() {
    let km = Kilometer::new(1.0);
    let m: Meter = km.to();
    assert_abs_diff_eq!(m.value(), 1000.0, epsilon = 1e-9);
}

#[test]
fn smoke_test_mass() {
    let kg = Kilogram::new(1000.0);
    let g: Gram = kg.to();
    assert_abs_diff_eq!(g.value(), 1_000_000.0, epsilon = 1e-6);
}

#[test]
#[cfg(feature = "astro")]
fn smoke_test_power() {
    let sol = SolarLuminosity::new(1.0);
    let w: Watt = sol.to();
    assert_relative_eq!(w.value(), 3.828e26, max_relative = 1e-9);
}

#[test]
fn smoke_test_velocity() {
    let v: velocity::Velocity<qtty::unit::Kilometer, qtty::unit::Second> =
        velocity::Velocity::new(1.0);
    let v_mps: velocity::Velocity<qtty::unit::Meter, qtty::unit::Second> = v.to();
    assert_abs_diff_eq!(v_mps.value(), 1000.0, epsilon = 1e-9);
}

#[test]
fn smoke_test_angular_rate() {
    let f: angular_rate::AngularRate<qtty::unit::Degree, qtty::unit::Day> =
        angular_rate::AngularRate::new(360.0);
    let f_rad: angular_rate::AngularRate<qtty::unit::Radian, qtty::unit::Day> = f.to();
    assert_abs_diff_eq!(f_rad.value(), 2.0 * std::f64::consts::PI, epsilon = 1e-12);
}

#[test]
fn smoke_test_same_unit_division_gives_raw_scalar() {
    let a = Meter::new(42.0);
    let b = Meter::new(6.0);
    let u: f64 = a / b;
    assert_abs_diff_eq!(u, 7.0, epsilon = 1e-12);
}

#[test]
fn smoke_test_erase_unit_raw() {
    let m = Meter::new(42.0);
    let u: f64 = m.erase_unit_raw();
    assert_eq!(u, 42.0);
}

#[test]
fn orbital_distance_calculation() {
    // Earth's orbital velocity ≈ 29.78 km/s
    let earth_velocity: velocity::Velocity<qtty::unit::Kilometer, qtty::unit::Second> =
        velocity::Velocity::new(29.78);

    // Time: 1 day
    let time = Day::new(1.0);
    let time_sec: Second = time.to();

    // Distance = velocity × time (recovery: Per<Km, S> * S → Km)
    let distance: Kilometer = earth_velocity * time_sec;

    // Earth travels about 2.57 million km per day
    assert_relative_eq!(distance.value(), 2_573_395.2, max_relative = 1e-3);
}

#[test]
#[cfg(feature = "astro")]
fn proxima_centauri_distance() {
    // Proxima Centauri is about 4.24 light years away
    let distance_ly = LightYear::new(4.24);

    // Convert to AU
    let distance_au: AstronomicalUnit = distance_ly.to();

    // Should be about 268,000 AU
    assert_relative_eq!(distance_au.value(), 268_000.0, max_relative = 0.01);
}

#[test]
fn angular_separation() {
    // Two stars at different positions
    let star1_ra = Degree::new(45.0);
    let star2_ra = Degree::new(350.0);

    // Separation should wrap around
    let sep = star1_ra.abs_separation(star2_ra);

    // 45° to 350° is 55° the short way
    assert_abs_diff_eq!(sep.value(), 55.0, epsilon = 1e-12);
}

#[test]
fn earth_rotation() {
    // Earth rotates 360° per sidereal day (~23h 56m)
    let rotation_rate: angular_rate::AngularRate<qtty::unit::Degree, qtty::unit::Day> =
        angular_rate::AngularRate::new(360.0);

    // After 6 hours (0.25 days)
    let time = Day::new(0.25);
    // Recovery: Per<Degree, Day> * Day → Degree
    let angle: Degree = rotation_rate * time;

    assert_abs_diff_eq!(angle.value(), 90.0, epsilon = 1e-12);
}

#[test]
#[cfg(feature = "astro")]
fn sun_mass() {
    let sun = SolarMass::new(1.0);
    let kg: Kilogram = sun.to();

    // Sun's mass is about 2e30 kg
    assert_relative_eq!(kg.value(), 1.988416e30, max_relative = 1e-5);
}

#[test]
#[cfg(feature = "astro")]
fn sun_luminosity() {
    let sun = SolarLuminosity::new(1.0);
    let watts: Watt = sun.to();

    // Sun's luminosity is about 3.828e26 W
    assert_relative_eq!(watts.value(), 3.828e26, max_relative = 1e-9);
}

#[test]
#[cfg(all(feature = "astro", feature = "julian-time"))]
fn calculate_velocity_from_distance_time() {
    // Light year to km
    let distance = LightYear::new(1.0);
    let distance_km: Kilometer = distance.to();

    // Julian year to seconds
    let time = JulianYear::new(1.0);
    let time_sec: Second = time.to();

    // Velocity = distance / time
    let velocity: velocity::Velocity<qtty::unit::Kilometer, qtty::unit::Second> =
        distance_km / time_sec;

    // Should be approximately speed of light (299,792 km/s)
    assert_relative_eq!(velocity.value(), 299_792.458, max_relative = 0.001);
}

#[test]
fn mean_motion_conversion() {
    // Earth's mean motion ≈ 0.9856°/day
    let mean_motion: angular_rate::AngularRate<qtty::unit::Degree, qtty::unit::Day> =
        angular_rate::AngularRate::new(0.9856);

    // Convert to degrees per year
    let per_year: angular_rate::AngularRate<qtty::unit::Degree, qtty::unit::Year> =
        mean_motion.to();

    // Should be about 360°/year
    assert_relative_eq!(per_year.value(), 360.0, max_relative = 0.01);
}

#[test]
fn trigonometric_calculation() {
    // 30° angle
    let angle = Degree::new(30.0);

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
    assert_eq!(qtty::unit::Meter::SYMBOL, "m");
    assert_eq!(qtty::unit::Kilometer::SYMBOL, "km");
    assert_eq!(qtty::unit::Second::SYMBOL, "s");
    assert_eq!(qtty::unit::Day::SYMBOL, "d");
    assert_eq!(qtty::unit::Degree::SYMBOL, "°");
    assert_eq!(qtty::unit::Radian::SYMBOL, "rad");
}

#[test]
fn derive_macro_produces_correct_ratio() {
    // Verify ratios are correct
    assert_eq!(qtty::unit::Meter::RATIO, 1.0);
    assert_eq!(qtty::unit::Kilometer::RATIO, 1000.0);
    assert_eq!(qtty::unit::Second::RATIO, 1.0);
    assert_eq!(qtty::unit::Degree::RATIO, 1.0);
}

#[test]
fn derive_macro_display_formatting() {
    let m = Meter::new(42.0);
    assert_eq!(format!("{}", m), "42 m");

    let km = Kilometer::new(1.5);
    assert_eq!(format!("{}", km), "1.5 km");

    let deg = Degree::new(90.0);
    assert_eq!(format!("{}", deg), "90 °");
}

#[test]
fn display_format_precision() {
    let x = Second::new(1234.56789);
    // {:.2} → two decimal places
    assert_eq!(format!("{:.2}", x), "1234.57 s");
    // {:.0} → no decimal places (rounded)
    assert_eq!(format!("{:.0}", x), "1235 s");
    // {:.5} → five decimal places
    assert_eq!(format!("{:.5}", x), "1234.56789 s");
}

#[test]
fn display_lower_exp_formatting() {
    let x = Second::new(1234.56789);
    // {:e} → default scientific notation (lower-case e)
    assert_eq!(format!("{:e}", x), "1.23456789e3 s");
    // {:.4e} → precision + scientific
    assert_eq!(format!("{:.4e}", x), "1.2346e3 s");
    // {:.0e} → zero decimal places
    assert_eq!(format!("{:.0e}", x), "1e3 s");
}

#[test]
fn display_upper_exp_formatting() {
    let x = Second::new(1234.56789);
    // {:E} → scientific notation upper-case E
    assert_eq!(format!("{:E}", x), "1.23456789E3 s");
    // {:.4E} → precision + upper-case scientific
    assert_eq!(format!("{:.4E}", x), "1.2346E3 s");
}

#[test]
fn display_format_annotations_per_unit() {
    let v: velocity::Velocity<qtty::unit::Kilometer, qtty::unit::Second> =
        velocity::Velocity::new(1234.56789);
    // Default
    assert_eq!(format!("{}", v), "1234.56789 km/s");
    // Precision
    assert_eq!(format!("{:.2}", v), "1234.57 km/s");
    // Scientific lower
    assert_eq!(format!("{:.4e}", v), "1.2346e3 km/s");
    // Scientific upper
    assert_eq!(format!("{:.4E}", v), "1.2346E3 km/s");
}

#[test]
fn display_sign_and_negative_values() {
    let neg = Meter::new(-42.5);
    assert_eq!(format!("{}", neg), "-42.5 m");
    assert_eq!(format!("{:.1}", neg), "-42.5 m");
    assert_eq!(format!("{:.2e}", neg), "-4.25e1 m");
}

#[test]
fn quantity_basic_arithmetic() {
    let a = Meter::new(10.0);
    let b = Meter::new(5.0);

    assert_eq!((a + b).value(), 15.0);
    assert_eq!((a - b).value(), 5.0);
    assert_eq!((a * 2.0).value(), 20.0);
    assert_eq!((a / 2.0).value(), 5.0);
}

#[test]
#[cfg(feature = "astro")]
fn quantity_conversion_chain() {
    // Convert through multiple units
    let au = AstronomicalUnit::new(1.0);
    let km: Kilometer = au.to();
    let m: Meter = km.to();

    // Direct conversion should match
    let m_direct: Meter = au.to();
    assert_abs_diff_eq!(m.value(), m_direct.value(), epsilon = 1e-3);
}

#[test]
fn quantity_negation() {
    let pos = Degree::new(45.0);
    let neg = -pos;
    assert_eq!(neg.value(), -45.0);
}

#[test]
fn quantity_abs() {
    let neg = Degree::new(-45.0);
    assert_eq!(neg.abs().value(), 45.0);
}

#[test]
fn per_unit_display() {
    let v: velocity::Velocity<qtty::unit::Kilometer, qtty::unit::Second> =
        velocity::Velocity::new(10.0);
    let s = format!("{}", v);
    assert_eq!(s, "10 km/s");
}

#[test]
fn per_unit_multiplication_recovers_numerator() {
    let v: velocity::Velocity<qtty::unit::Kilometer, qtty::unit::Second> =
        velocity::Velocity::new(100.0);
    let t: Second = Second::new(3600.0);
    // UnitMul: Per<Km, S> * S → Km (recovery impl)
    let d: Kilometer = v * t;
    assert_abs_diff_eq!(d.value(), 360_000.0, epsilon = 1e-6);
}

#[test]
fn qtty_values_macro_builds_seconds_array() {
    const DT: [Second; 3] = qtty::qtty_vec!(Second; 56.86, 63.83, 70.0);
    assert_abs_diff_eq!(DT[0].value(), 56.86, epsilon = 1e-12);
    assert_abs_diff_eq!(DT[1].value(), 63.83, epsilon = 1e-12);
    assert_abs_diff_eq!(DT[2].value(), 70.0, epsilon = 1e-12);
}

#[test]
fn qtty_values_macro_builds_seconds_vec() {
    let dt: Vec<Second> = qtty::qtty_vec!(vec Second; 56.86, 63.83, 70.0);
    assert_eq!(dt.len(), 3);
    assert_abs_diff_eq!(dt[1].value(), 63.83, epsilon = 1e-12);
}

#[test]
fn per_unit_division_creates_composite() {
    let d = Kilometer::new(100.0);
    let t = Second::new(10.0);
    let v: velocity::Velocity<qtty::unit::Kilometer, qtty::unit::Second> = d / t;
    assert_abs_diff_eq!(v.value(), 10.0, epsilon = 1e-12);
}

#[test]
fn unit_constants_have_value_one() {
    #[cfg(feature = "astro")]
    {
        assert_eq!(AU.value(), 1.0);
        assert_eq!(LY.value(), 1.0);
    }
    assert_eq!(KM.value(), 1.0);
    assert_eq!(DAY.value(), 1.0);
    assert_eq!(SEC.value(), 1.0);
    assert_eq!(DEG.value(), 1.0);
    assert_eq!(RAD.value(), 1.0);
}

#[test]
fn constants_can_be_multiplied() {
    #[cfg(feature = "astro")]
    {
        let distance = 4.24 * LY;
        assert_eq!(distance.value(), 4.24);
    }

    let time = 365.25 * DAY;
    assert_eq!(time.value(), 365.25);
}

#[test]
#[cfg(feature = "astro")]
fn macro_generated_conversions() {
    // Test conversions that are now generated by impl_unit_conversions! macro
    // These weren't manually implemented before

    // Meter -> AstronomicalUnit (AU is exactly 149,597,870,700 m)
    let m = Meter::new(149_597_870_700.0);
    let au: AstronomicalUnit = m.into();
    assert_relative_eq!(au.value(), 1.0, max_relative = 1e-12);

    // Nominal SolarRadius -> Kilometer
    let sr = SolarRadius::new(1.0);
    let km: Kilometer = sr.into();
    assert_abs_diff_eq!(km.value(), 695_700.0, epsilon = 1e-6);

    // Parsec -> AstronomicalUnit
    let pc = Parsec::new(1.0);
    let au: AstronomicalUnit = pc.into();
    // 1 pc = au * 648000 / π
    let expected = 648_000.0 / core::f64::consts::PI;
    assert_relative_eq!(au.value(), expected, max_relative = 1e-12);
}

#[test]
#[cfg(all(feature = "astro", feature = "navigation"))]
fn new_angular_units() {
    // Test the new angular units added via impl_unit_conversions! macro

    // Arcminute conversions
    let deg = Degree::new(1.0);
    let arcm: Arcminute = deg.into();
    assert_abs_diff_eq!(arcm.value(), 60.0, epsilon = 1e-12);

    // Microarcsecond conversions
    let arcs = Arcsecond::new(1.0);
    let uas: MicroArcsecond = arcs.into();
    assert_abs_diff_eq!(uas.value(), 1_000_000.0, epsilon = 1e-6);

    // Gradian conversions (1 full turn = 400 gradians)
    let turn = Turn::new(1.0);
    let gon: Gradian = turn.into();
    assert_abs_diff_eq!(gon.value(), 400.0, epsilon = 1e-12);

    // Turn conversions
    let deg = Degree::new(180.0);
    let turn: Turn = deg.into();
    assert_abs_diff_eq!(turn.value(), 0.5, epsilon = 1e-12);

    // Test trig functions work with new units
    let right_angle = Gradian::new(100.0); // 90 degrees
    assert_abs_diff_eq!(right_angle.sin(), 1.0, epsilon = 1e-12);
    assert_abs_diff_eq!(right_angle.cos(), 0.0, epsilon = 1e-12);

    // Test wrapping with new units
    let turn = Turn::new(2.7);
    let wrapped = turn.wrap_pos();
    assert_abs_diff_eq!(wrapped.value(), 0.7, epsilon = 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// f32 scalar type tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn smoke_test_f32_angular() {
    use qtty::f32::{Degree, Radian};

    let deg = Degree::new(180.0_f32);
    let rad: Radian = deg.to();
    assert!((rad.value() - core::f32::consts::PI).abs() < 1e-5);
}

#[test]
fn smoke_test_f32_length() {
    use qtty::f32::{Kilometer, Meter};

    let km = Kilometer::new(1.0_f32);
    let m: Meter = km.to();
    assert!((m.value() - 1000.0).abs() < 1e-4);
}

#[test]
fn smoke_test_f32_time() {
    use qtty::f32::{Day, Second};

    let day = Day::new(1.0_f32);
    let sec: Second = day.to();
    assert!((sec.value() - 86400.0).abs() < 1.0);
}

#[test]
fn smoke_test_f32_arithmetic() {
    use qtty::f32::Meter;

    let a = Meter::new(10.0_f32);
    let b = Meter::new(5.0_f32);
    let sum = a + b;
    let diff = a - b;

    assert!((sum.value() - 15.0).abs() < 1e-6);
    assert!((diff.value() - 5.0).abs() < 1e-6);
}

#[test]
fn smoke_test_f32_trig() {
    use qtty::f32::Degree;

    let angle = Degree::new(90.0_f32);
    assert!((angle.sin() - 1.0).abs() < 1e-5);
    assert!(angle.cos().abs() < 1e-5);
}

// ─────────────────────────────────────────────────────────────────────────────
// Integer scalar module tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn smoke_test_i8_module_arithmetic_and_lossy_conversion() {
    use qtty::i8::{Meter, Minute, Second};

    let a = Meter::new(50);
    let b = Meter::new(20);
    assert_eq!((a + b).value(), 70);
    assert_eq!((a - b).value(), 30);

    let sec = Second::new(125);
    let min: Minute = sec.to_lossy();
    assert_eq!(min.value(), 2);
}

#[test]
fn smoke_test_i16_module_arithmetic_and_lossy_conversion() {
    use qtty::i16::{Kilometer, Meter};

    let a = Meter::new(1_500);
    let b = Meter::new(250);
    assert_eq!((a + b).value(), 1_750);

    let km: Kilometer = a.to_lossy();
    assert_eq!(km.value(), 1);
}

#[test]
fn smoke_test_i128_module_arithmetic_and_lossy_conversion() {
    use qtty::i128::{Day, Second};

    let a = Second::new(172_800);
    let b = Second::new(3_600);
    assert_eq!((a + b).value(), 176_400);

    let days: Day = a.to_lossy();
    assert_eq!(days.value(), 2);
}
