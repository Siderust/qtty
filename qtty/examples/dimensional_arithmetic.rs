//! Dimensional arithmetic examples.
//!
//! Demonstrates how the `*` and `/` operators between quantities automatically
//! produce the correct dimensional result at compile time.
//!
//! Run with:
//! ```sh
//! cargo run -p qtty --example dimensional_arithmetic
//! ```

use qtty::area::SquareMeters;
use qtty::length::{Kilometer, Kilometers, Meter, Meters};
use qtty::time::{Second, Seconds};
use qtty::velocity::Velocity;
use qtty::volume::CubicMeters;
use qtty::{Per, Quantity, Simplify, Unitless};

fn main() {
    // ─────────────────────────────────────────────────────────────────────
    // 1.  Length × Length → Area
    // ─────────────────────────────────────────────────────────────────────
    let width = Meters::new(4.0);
    let height = Meters::new(5.0);

    // The product type is Quantity<Prod<Meter, Meter>>; convert to a named area unit:
    let area: SquareMeters = (width * height).to();
    println!("4 m × 5 m = {} m²", area.value());
    assert!((area.value() - 20.0).abs() < 1e-12);

    // ─────────────────────────────────────────────────────────────────────
    // 2.  Area × Length → Volume
    // ─────────────────────────────────────────────────────────────────────
    let depth = Meters::new(3.0);
    let vol: CubicMeters = (area * depth).to();
    println!("20 m² × 3 m = {} m³", vol.value());
    assert!((vol.value() - 60.0).abs() < 1e-12);

    // ─────────────────────────────────────────────────────────────────────
    // 3.  Length / Time → Velocity
    // ─────────────────────────────────────────────────────────────────────
    let distance = Kilometers::new(100.0);
    let time = Seconds::new(3600.0);
    let speed: Velocity<Kilometer, Second> = distance / time;
    println!("100 km / 3600 s ≈ {:.6} km/s", speed.value());
    assert!((speed.value() - 100.0 / 3600.0).abs() < 1e-12);

    // ─────────────────────────────────────────────────────────────────────
    // 4.  Velocity × Time → recovers Length  (via .to())
    // ─────────────────────────────────────────────────────────────────────
    let recovered: Kilometers = (speed * time).to();
    println!(
        "{:.6} km/s × 3600 s = {} km",
        speed.value(),
        recovered.value()
    );
    assert!((recovered.value() - 100.0).abs() < 1e-9);

    // ─────────────────────────────────────────────────────────────────────
    // 5.  Length / Length → dimensionless
    // ─────────────────────────────────────────────────────────────────────
    let a = Meters::new(10.0);
    let b = Meters::new(4.0);
    let ratio: Quantity<Per<Meter, Meter>> = a / b;
    // Simplify to a plain f64 via Unitless:
    let simplified: Quantity<Unitless> = ratio.simplify();
    println!("10 m / 4 m = {} (dimensionless)", simplified.value());
    assert!((simplified.value() - 2.5).abs() < 1e-12);

    // ─────────────────────────────────────────────────────────────────────
    // 6.  Mixed units: km / m keeps the composite type
    // ─────────────────────────────────────────────────────────────────────
    let km = Kilometers::new(1.0);
    let m = Meters::new(500.0);
    let mixed = km / m; // Quantity<Per<Kilometer, Meter>>
                        // The value is 1.0/500.0 in "km per m" units:
    println!("1 km / 500 m = {} km/m", mixed.value());
    // But in dimensionless terms (since km/m = 1000) the real ratio is 2.0:
    let pure: Quantity<Unitless> = mixed.to();
    println!("  → as dimensionless = {}", pure.value());
    assert!((pure.value() - 2.0).abs() < 1e-12);

    println!("\nAll dimensional arithmetic checks passed!");
}
