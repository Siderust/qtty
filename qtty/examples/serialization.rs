// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Examples of serializing and deserializing quantities.
//!
//! This example demonstrates how to use serde to serialize and deserialize
//! physical quantities to/from JSON and other formats.
//!
//! To run this example with serde support:
//! ```bash
//! cargo run --example serialization --features serde
//! ```

#[cfg(feature = "serde")]
fn main() {
    use qtty::velocity::Velocity;
    use qtty::{Kilogram, Kilometer, Meter, Second, Watt};
    use serde::{Deserialize, Serialize};
    use serde_json;

    println!("=== Quantity Serialization Examples ===\n");

    // Example 1: Basic serialization to JSON
    println!("1. Basic JSON Serialization:");
    let distance = Meter::new(42.5);
    let json = serde_json::to_string(&distance).unwrap();
    println!("   Distance: {} → JSON: {}", distance, json);

    let time = Second::new(std::f64::consts::PI);
    let json = serde_json::to_string(&time).unwrap();
    println!("   Time: {} → JSON: {}", time, json);

    let mass = Kilogram::new(100.0);
    let json = serde_json::to_string(&mass).unwrap();
    println!("   Mass: {} → JSON: {}", mass, json);
    println!();

    // Example 2: Deserialization from JSON
    println!("2. JSON Deserialization:");
    let json_distance = "42.5";
    let distance = serde_json::from_str::<Meter>(json_distance).unwrap();
    println!("   JSON: {} → {}", json_distance, distance);

    let json_time = "3.14";
    let time = serde_json::from_str::<Second>(json_time).unwrap();
    println!("   JSON: {} → {}", json_time, time);
    println!();

    // Example 3: Round-trip serialization
    println!("3. Round-trip Serialization:");
    let original_distance = Meter::new(299792458.0);
    let original_time = Second::new(1.0);
    let original: Velocity<qtty::unit::Meter, qtty::unit::Second> =
        original_distance / original_time;
    let json = serde_json::to_string(&original).unwrap();
    let restored =
        serde_json::from_str::<Velocity<qtty::unit::Meter, qtty::unit::Second>>(&json).unwrap();
    println!("   Original: {}", original);
    println!("   JSON: {}", json);
    println!("   Restored: {}", restored);
    println!(
        "   Equal: {}",
        (original.value() - restored.value()).abs() < 1e-6
    );
    println!();

    // Example 4: Serializing structs containing quantities
    #[derive(Serialize, Deserialize, Debug)]
    struct Measurement {
        distance: Meter,
        time: Second,
        mass: Kilogram,
    }

    println!("4. Serializing Structs with Quantities:");
    let measurement = Measurement {
        distance: Meter::new(100.0),
        time: Second::new(9.58),
        mass: Kilogram::new(75.0),
    };

    let json = serde_json::to_string_pretty(&measurement).unwrap();
    println!("   Struct to JSON:\n{}", json);

    let restored: Measurement = serde_json::from_str(&json).unwrap();
    println!("   Restored: {:?}", restored);
    println!();

    // Example 5: Serializing collections of quantities
    println!("5. Serializing Collections:");
    let distances = vec![Meter::new(10.0), Meter::new(20.0), Meter::new(30.0)];
    let json = serde_json::to_string(&distances).unwrap();
    println!("   Vec of distances → JSON: {}", json);

    let restored: Vec<Meter> = serde_json::from_str(&json).unwrap();
    println!("   Restored: {:?}", restored);
    println!();

    // Example 6: Using pretty printing
    println!("6. Pretty-printed JSON:");
    #[derive(Serialize, Deserialize)]
    struct Experiment {
        name: String,
        height: Meter,
        duration: Second,
        velocity: Velocity<qtty::unit::Meter, qtty::unit::Second>,
    }

    let experiment = Experiment {
        name: "Free Fall Test".to_string(),
        height: Meter::new(100.0),
        duration: Second::new(4.52),
        velocity: Meter::new(44.3) / Second::new(1.0),
    };

    let json = serde_json::to_string_pretty(&experiment).unwrap();
    println!("{}", json);
    println!();

    // Example 7: Handling conversion before serialization
    println!("7. Converting Units Before Serialization:");
    let distance_km = Kilometer::new(5.0);
    // Convert to base unit (meter) before serializing
    let distance_m = distance_km.to::<qtty::unit::Meter>();
    let json = serde_json::to_string(&distance_m).unwrap();
    println!("   {} → {} → JSON: {}", distance_km, distance_m, json);
    println!();

    // Example 8: Derived quantities
    println!("8. Serializing Derived Quantities:");
    let power = Watt::new(1500.0);
    let json = serde_json::to_string(&power).unwrap();
    println!("   Power: {} → JSON: {}", power, json);

    println!("=== Important Notes ===");
    println!("• Quantities serialize as bare f64 values (the numeric value only)");
    println!("• Unit information is NOT preserved in serialization");
    println!("• You must specify the correct unit type when deserializing");
    println!("• Always use the base SI unit for consistency when serializing");
}

#[cfg(not(feature = "serde"))]
fn main() {
    println!("This example requires the 'serde' feature.");
    println!("Run with: cargo run --example serialization --features serde");
}
