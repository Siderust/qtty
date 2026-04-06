//! Advanced serialization examples showing edge cases and best practices.
//!
//! Run with: cargo run --example serialization_advanced --features serde

#[cfg(feature = "serde")]
fn main() {
    use qtty::{Kilometer, Meter, Second};
    use serde::{Deserialize, Serialize};
    use serde_json;

    println!("=== Advanced Serialization Examples ===\n");

    // Example 1: Handling zero and special values
    println!("1. Special Values:");
    let zero = Meter::new(0.0);
    let json = serde_json::to_string(&zero).unwrap();
    println!("   Zero: {} → {}", zero, json);

    let negative = Meter::new(-42.5);
    let json = serde_json::to_string(&negative).unwrap();
    println!("   Negative: {} → {}", negative, json);

    let large = Meter::new(1.23e15);
    let json = serde_json::to_string(&large).unwrap();
    println!("   Large number: {} → {}", large, json);
    println!();

    // Example 2: Nested structures
    println!("2. Nested Structures:");
    #[derive(Serialize, Deserialize, Debug)]
    struct Location {
        name: String,
        coordinates: Coordinates,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Coordinates {
        x: Meter,
        y: Meter,
        z: Meter,
    }

    let location = Location {
        name: "Mount Everest".to_string(),
        coordinates: Coordinates {
            x: Meter::new(86.925278),
            y: Meter::new(27.988056),
            z: Meter::new(8848.86),
        },
    };

    let json = serde_json::to_string_pretty(&location).unwrap();
    println!("{}", json);

    let restored: Location = serde_json::from_str(&json).unwrap();
    println!("   Restored name: {}", restored.name);
    println!();

    // Example 3: Option types
    println!("3. Optional Quantities:");
    #[derive(Serialize, Deserialize, Debug)]
    struct Measurement {
        required: Meter,
        optional: Option<Meter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        skipped_if_none: Option<Second>,
    }

    let with_value = Measurement {
        required: Meter::new(100.0),
        optional: Some(Meter::new(50.0)),
        skipped_if_none: Some(Second::new(10.0)),
    };
    println!(
        "   With values: {}",
        serde_json::to_string_pretty(&with_value).unwrap()
    );

    let without_value = Measurement {
        required: Meter::new(100.0),
        optional: None,
        skipped_if_none: None,
    };
    println!(
        "   Without optional: {}",
        serde_json::to_string_pretty(&without_value).unwrap()
    );
    println!();

    // Example 4: Unit conversion awareness
    println!("4. Unit Conversion During Serialization:");
    println!("   ⚠️  WARNING: Always convert to base units before serializing!");

    let distance_km = Kilometer::new(5.0);
    let distance_m = distance_km.to::<qtty::unit::Meter>();

    let json_km = serde_json::to_string(&distance_km).unwrap();
    let json_m = serde_json::to_string(&distance_m).unwrap();

    println!("   5 km serialized directly: {}", json_km);
    println!("   5 km converted to meters: {}", json_m);
    println!("   Note: Both serialize the same value, but semantics differ!");
    println!();

    // Example 5: Error handling
    println!("5. Error Handling:");

    // Invalid JSON
    let invalid_json = "not_a_number";
    match serde_json::from_str::<Meter>(invalid_json) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   Expected error: {}", e),
    }

    // Empty string
    let empty = "";
    match serde_json::from_str::<Meter>(empty) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   Expected error: {}", e),
    }
    println!();

    // Example 6: Compact vs Pretty printing
    println!("6. Compact vs Pretty Printing:");
    #[derive(Serialize, Deserialize)]
    struct Data {
        distances: Vec<Meter>,
        times: Vec<Second>,
    }

    let data = Data {
        distances: vec![Meter::new(1.0), Meter::new(2.0), Meter::new(3.0)],
        times: vec![Second::new(0.1), Second::new(0.2), Second::new(0.3)],
    };

    let compact = serde_json::to_string(&data).unwrap();
    println!("   Compact: {}", compact);

    let pretty = serde_json::to_string_pretty(&data).unwrap();
    println!("   Pretty:\n{}", pretty);
    println!();

    println!("=== Best Practices ===");
    println!("✓ Always convert to base SI units before serializing");
    println!("✓ Document the expected unit in your API documentation");
    println!("✓ Validate deserialized values are in expected range");
    println!("✓ Consider creating custom serializers for complex scenarios");
    println!("✓ Use Option<Quantity<U>> for optional measurements");
}

#[cfg(not(feature = "serde"))]
fn main() {
    println!("This example requires the 'serde' feature.");
    println!("Run with: cargo run --example serialization_advanced --features serde");
}
