use unit::*;

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
    let rotation_rate: DegreesPerDay = Quantity::new(360.0);

    // After 6 hours (0.25 days)
    let time = Days::new(0.25);
    let angle: Degrees = rotation_rate * time;

    assert_abs_diff_eq!(angle.value(), 90.0, epsilon = 1e-12);
}

#[test]
fn sun_mass() {
    let sun = SolarMasses::new(1.0);
    let kg: Kilograms = sun.to();

    // Sun's mass is about 2e30 kg
    assert_relative_eq!(kg.value(), 1.98847e30, max_relative = 1e-5);
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
    let velocity: KilometersPerSecond = distance_km / time_sec;

    // Should be approximately speed of light (299,792 km/s)
    assert_relative_eq!(velocity.value(), 299_792.458, max_relative = 0.001);
}

#[test]
fn mean_motion_conversion() {
    // Earth's mean motion ≈ 0.9856°/day
    let mean_motion: DegreesPerDay = Quantity::new(0.9856);

    // Convert to degrees per year
    let per_year: DegreesPerYear = mean_motion.to();

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
    assert_eq!(Kilometer::SYMBOL, "Km");
    assert_eq!(Second::SYMBOL, "sec");
    assert_eq!(Day::SYMBOL, "d");
    assert_eq!(Degree::SYMBOL, "Deg");
    assert_eq!(Radian::SYMBOL, "Rad");
}

#[test]
fn derive_macro_produces_correct_ratio() {
    // Verify ratios are correct
    assert_eq!(Meter::RATIO, 1.0);
    assert_eq!(Kilometer::RATIO, 1000.0);
    assert_eq!(Day::RATIO, 1.0);
    assert_eq!(Degree::RATIO, 1.0);
}

#[test]
fn derive_macro_display_formatting() {
    let m = Meters::new(42.0);
    assert_eq!(format!("{}", m), "42 m");

    let km = Kilometers::new(1.5);
    assert_eq!(format!("{}", km), "1.5 Km");

    let deg = Degrees::new(90.0);
    assert_eq!(format!("{}", deg), "90 Deg");
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
    let v: KilometersPerSecond = Quantity::new(10.0);
    let s = format!("{}", v);
    assert_eq!(s, "10 Km/sec");
}

#[test]
fn per_unit_multiplication_recovers_numerator() {
    let v: KilometersPerSecond = Quantity::new(100.0);
    let t: Seconds = Seconds::new(3600.0);
    let d: Kilometers = v * t;
    assert_abs_diff_eq!(d.value(), 360_000.0, epsilon = 1e-6);
}

#[test]
fn per_unit_division_creates_composite() {
    let d = Kilometers::new(100.0);
    let t = Seconds::new(10.0);
    let v: KilometersPerSecond = d / t;
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
