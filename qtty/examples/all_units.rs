// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 Vallés Puig, Ramon

//! Full unit survey: every dimension, integer scalars, `f32` scalars,
//! and the `qtty_vec!` macro.
//!
//! Demonstrates:
//! - Area and volume from dimensional multiplication
//! - Mass and power units (including solar constants)
//! - Angular frequency (`Frequency<A, T>` = angular / time)
//! - Unitless ratio via `Simplify`
//! - Integer-scalar quantities (`i64`) with `to_lossy()`
//! - `f32`-scalar quantities
//! - The `qtty_vec!` macro for typed arrays and vectors
//!
//! Run with:
//! ```sh
//! cargo run -p qtty --example all_units
//! ```

use qtty::area::SquareMeters;
use qtty::frequency::Frequency;
use qtty::mass::{Kilogram, Kilograms, SolarMasses};
use qtty::power::{SolarLuminosities, Watts};
use qtty::velocity::Velocity;
use qtty::volume::CubicMeters;
use qtty::{
    AstronomicalUnits, Day, Degree, Kilometer, Kilometers, LightYears, Meters, Radian, Second,
    Seconds, Simplify, Unitless,
};

fn main() {
    // ── 1. Area ──────────────────────────────────────────────────────────────
    println!("1. Area (Length × Length):");
    let width = Meters::new(4.0);
    let height = Meters::new(5.0);
    let area: SquareMeters = (width * height).to();
    println!(
        "   {} × {} = {} m²",
        width.value(),
        height.value(),
        area.value()
    );
    assert!((area.value() - 20.0).abs() < 1e-12);

    // ── 2. Volume ─────────────────────────────────────────────────────────────
    println!("\n2. Volume (Area × Length):");
    let depth = Meters::new(3.0);
    let vol: CubicMeters = (area * depth).to();
    println!(
        "   {} m² × {} m = {} m³",
        area.value(),
        depth.value(),
        vol.value()
    );
    assert!((vol.value() - 60.0).abs() < 1e-12);

    // ── 3. Mass ───────────────────────────────────────────────────────────────
    println!("\n3. Mass:");
    let person = Kilograms::new(70.0);
    println!("   Person: {}", person.value());

    let sun = SolarMasses::new(1.0);
    let sun_kg: Kilograms = sun.to::<Kilogram>();
    println!("   1 M☉ = {:.4e} kg", sun_kg.value());
    assert!(sun_kg.value() > 1.9e30 && sun_kg.value() < 2.0e30);

    // ── 4. Power ──────────────────────────────────────────────────────────────
    println!("\n4. Power:");
    let bulb = Watts::new(100.0);
    println!("   Light bulb: {} W", bulb.value());

    let sol_lum = SolarLuminosities::new(1.0);
    let sol_watts: Watts = sol_lum.to();
    println!("   1 L☉ = {:.4e} W", sol_watts.value());
    assert!(sol_watts.value() > 3.8e26 && sol_watts.value() < 3.9e26);

    // ── 5. Angular Frequency ─────────────────────────────────────────────────
    println!("\n5. Angular frequency (Angular / Time):");
    // Earth rotation: 360° per day
    let earth_rot: Frequency<Degree, Day> = Frequency::new(360.0);
    let earth_rot_rads: Frequency<Radian, Day> = earth_rot.to();
    let earth_rot_degs: Frequency<Degree, Second> = earth_rot.to();
    println!(
        "   Earth rotation: {} °/day = {:.6} rad/day = {:.6e} °/s",
        earth_rot.value(),
        earth_rot_rads.value(),
        earth_rot_degs.value()
    );

    // Moon mean angular velocity: 360° per 27.321661 days (sidereal)
    let moon_angular: Frequency<Degree, Day> = Frequency::new(360.0 / 27.321_661);
    let moon_rads: Frequency<Radian, Day> = moon_angular.to();
    println!(
        "   Moon angular velocity: {:.4} °/day = {:.4} rad/day",
        moon_angular.value(),
        moon_rads.value()
    );

    // ── 6. Unitless ratio via Simplify ───────────────────────────────────────
    println!("\n6. Unitless ratio (Simplify):");
    let au_dist = AstronomicalUnits::new(1.0);
    let au_in_km: Kilometers = au_dist.to::<Kilometer>();
    let km_ratio = Kilometers::new(1_000.0) / Kilometers::new(500.0);
    let dimensionless: qtty::Quantity<Unitless> = km_ratio.simplify();
    println!(
        "   1 km / 500 m (as km/km ratio) simplifies → {}",
        dimensionless.value()
    );
    assert!((dimensionless.value() - 2.0).abs() < 1e-12);
    println!("   1 AU = {:.0} km", au_in_km.value());

    // ── 7. Velocity cross-check ───────────────────────────────────────────────
    println!("\n7. Velocity (Length / Time):");
    let earth_vel: Velocity<Kilometer, Second> = Velocity::new(29.783); // km/s
    let travel = Seconds::new(86_400.0); // 1 day
    let daily_dist: Kilometers = (earth_vel * travel).to();
    println!(
        "   Earth at {:.3} km/s × {:.0} s = {:.0} km",
        earth_vel.value(),
        travel.value(),
        daily_dist.value()
    );

    // ── 8. Integer scalars ────────────────────────────────────────────────────
    println!("\n8. Integer-scalar quantities (i64):");
    {
        use qtty::i64::{Degrees as IDeg, Meters as IMeters};
        let a = IMeters::new(1_000);
        let b = IMeters::new(500);
        let sum = a + b;
        println!("   1000 m + 500 m = {} m  (i64)", sum.value());

        // to_lossy: convert between units via f64, then truncate back
        let km: qtty::Quantity<Kilometer, i64> = a.to_lossy();
        println!("   1000 m to_lossy km = {} km  (truncated)", km.value());

        // Angles
        let angle = IDeg::new(360_i64);
        let halved = angle / 2_i64;
        println!("   360° / 2 = {}°  (i64)", halved.value());
    }

    // ── 9. f32 scalars ────────────────────────────────────────────────────────
    println!("\n9. f32-scalar quantities:");
    {
        use qtty::f32::{Degrees as F32Deg, Meters as F32Meters, Seconds as F32Sec};
        let d: F32Meters = F32Meters::new(299_792_458.0_f32);
        let t: F32Sec = F32Sec::new(1.0_f32);
        let v_mps = d.value() / t.value();
        println!(
            "   {:.3e} m / {} s = {:.3e} m/s  (f32)",
            d.value(),
            t.value(),
            v_mps
        );

        let angle: F32Deg = F32Deg::new(180.0_f32);
        let rad: qtty::Quantity<Radian, f32> = angle.to::<Radian>();
        println!(
            "   180° (f32) = {:.6} rad  (π ≈ {:.6})",
            rad.value(),
            core::f32::consts::PI
        );
        assert!((rad.value() - core::f32::consts::PI).abs() < 1e-4);
    }

    // ── 10. qtty_vec! macro ────────────────────────────────────────────────────
    println!("\n10. qtty_vec! macro:");

    // Const array
    const OFFSETS: [Seconds; 5] = qtty::qtty_vec!(Seconds; 0.0, 15.0, 30.0, 45.0, 60.0);
    print!("    Observation cadence: ");
    for (i, s) in OFFSETS.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{:.0}s", s.value());
    }
    println!();

    // Dynamic vector
    let star_distances: Vec<LightYears> = qtty::qtty_vec!(vec LightYears; 4.24, 8.58, 11.43);
    let names = ["Proxima Centauri", "Barnard's Star", "Wolf 359"];
    for (name, dist) in names.iter().zip(star_distances.iter()) {
        let dist_au: AstronomicalUnits = dist.to::<qtty::AstronomicalUnit>();
        println!(
            "    {}: {:.2} ly = {:.0} AU",
            name,
            dist.value(),
            dist_au.value()
        );
    }

    println!("\n=== all_units complete ===");
}
