//! Advanced serialization examples showing edge cases and best practices.
//!
//! Run with: cargo run --example serialization_advanced --features serde

#[cfg(feature = "serde")]
fn main() {
    use qtty::{Kilometers, Meters, Seconds};
    use serde::{Deserialize, Serialize};
    use serde_json;

    println!("=== Advanced Serialization Examples ===\n");

    // Example 1: Handling zero and special values
    println!("1. Special Values:");
    let zero = Meters::new(0.0);
    let json = serde_json::to_string(&zero).unwrap();
    println!("   Zero: {} → {}", zero, json);

    let negative = Meters::new(-42.5);
    let json = serde_json::to_string(&negative).unwrap();
    println!("   Negative: {} → {}", negative, json);

    let large = Meters::new(1.23e15);
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
        x: Meters,
        y: Meters,
        z: Meters,
    }

    let location = Location {
        name: "Mount Everest".to_string(),
        coordinates: Coordinates {
            x: Meters::new(86.925278),
            y: Meters::new(27.988056),
            z: Meters::new(8848.86),
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
        required: Meters,
        optional: Option<Meters>,
        #[serde(skip_serializing_if = "Option::is_none")]
        skipped_if_none: Option<Seconds>,
    }

    let with_value = Measurement {
        required: Meters::new(100.0),
        optional: Some(Meters::new(50.0)),
        skipped_if_none: Some(Seconds::new(10.0)),
    };
    println!(
        "   With values: {}",
        serde_json::to_string_pretty(&with_value).unwrap()
    );

    let without_value = Measurement {
        required: Meters::new(100.0),
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

    let distance_km = Kilometers::new(5.0);
    let distance_m = distance_km.to::<qtty::Meter>();

    let json_km = serde_json::to_string(&distance_km).unwrap();
    let json_m = serde_json::to_string(&distance_m).unwrap();

    println!("   5 Km serialized directly: {}", json_km);
    println!("   5 Km converted to meters: {}", json_m);
    println!("   Note: Both serialize the same value, but semantics differ!");
    println!();

    // Example 5: Error handling
    println!("5. Error Handling:");

    // Invalid JSON
    let invalid_json = "not_a_number";
    match serde_json::from_str::<Meters>(invalid_json) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   Expected error: {}", e),
    }

    // Empty string
    let empty = "";
    match serde_json::from_str::<Meters>(empty) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   Expected error: {}", e),
    }
    println!();

    // Example 6: Compact vs Pretty printing
    println!("6. Compact vs Pretty Printing:");
    #[derive(Serialize, Deserialize)]
    struct Data {
        distances: Vec<Meters>,
        times: Vec<Seconds>,
    }

    let data = Data {
        distances: vec![Meters::new(1.0), Meters::new(2.0), Meters::new(3.0)],
        times: vec![Seconds::new(0.1), Seconds::new(0.2), Seconds::new(0.3)],
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
