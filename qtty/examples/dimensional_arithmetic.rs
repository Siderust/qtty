// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Dimensional arithmetic examples.
//!
//! Demonstrates how the `*` and `/` operators between quantities automatically
//! produce the correct dimensional result at compile time.
//!
//! Run with:
//! ```sh
//! cargo run -p qtty --example dimensional_arithmetic
//! ```

use qtty::velocity::Velocity;
use qtty::{CubicMeter, Kilometer, Meter, Second, SquareMeter};

fn main() {
    // ─────────────────────────────────────────────────────────────────────
    // 1.  Length × Length → Area
    // ─────────────────────────────────────────────────────────────────────
    let width = Meter::new(4.0);
    let height = Meter::new(5.0);

    // The product type is Quantity<Prod<unit::Meter, unit::Meter>>; convert to a named area unit:
    let area: SquareMeter = (width * height).to();
    println!("4 m × 5 m = {} m²", area.value());
    assert!((area.value() - 20.0).abs() < 1e-12);

    // ─────────────────────────────────────────────────────────────────────
    // 2.  Area × Length → Volume
    // ─────────────────────────────────────────────────────────────────────
    let depth = Meter::new(3.0);
    let vol: CubicMeter = (area * depth).to();
    println!("20 m² × 3 m = {} m³", vol.value());
    assert!((vol.value() - 60.0).abs() < 1e-12);

    // ─────────────────────────────────────────────────────────────────────
    // 3.  Length / Time → Velocity
    // ─────────────────────────────────────────────────────────────────────
    let distance = Kilometer::new(100.0);
    let time = Second::new(3600.0);
    let speed: Velocity<qtty::unit::Kilometer, qtty::unit::Second> = distance / time;
    println!("100 km / 3600 s ≈ {:.6} km/s", speed.value());
    assert!((speed.value() - 100.0 / 3600.0).abs() < 1e-12);

    // ─────────────────────────────────────────────────────────────────────
    // 4.  Velocity × Time → recovers Length  (via recovery impl)
    // ─────────────────────────────────────────────────────────────────────
    let recovered: Kilometer = speed * time;
    println!(
        "{:.6} km/s × 3600 s = {} km",
        speed.value(),
        recovered.value()
    );
    assert!((recovered.value() - 100.0).abs() < 1e-9);

    // ─────────────────────────────────────────────────────────────────────
    // 5.  Length / Length → raw scalar
    // ─────────────────────────────────────────────────────────────────────
    let a = Meter::new(10.0);
    let b = Meter::new(4.0);
    let ratio: f64 = a / b;
    println!("10 m / 4 m = {} (dimensionless)", ratio);
    assert!((ratio - 2.5).abs() < 1e-12);

    // ─────────────────────────────────────────────────────────────────────
    // 6.  Mixed units: km / m keeps the composite type; normalise to scalar
    // ─────────────────────────────────────────────────────────────────────
    let km = Kilometer::new(1.0);
    let m = Meter::new(500.0);
    let mixed = km / m; // Quantity<Per<qtty::unit::Kilometer, qtty::unit::Meter>>
                        // The value is 1.0/500.0 in "km per m" units:
    println!("1 km / 500 m = {} km/m", mixed.value());
    // Convert km to meters, then divide same-unit to get the raw scalar:
    let km_as_m: Meter = km.to();
    let pure: f64 = km_as_m / m;
    println!("  → as dimensionless = {}", pure);
    assert!((pure - 2.0).abs() < 1e-12);

    println!("\nAll dimensional arithmetic checks passed!");
}
