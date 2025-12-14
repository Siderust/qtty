//! Example demonstrating the serde_with_unit helper module.
//!
//! This shows how to use #[serde(with = "qtty_core::serde_with_unit")] to preserve
//! unit information in serialized data on a per-field basis.
//!
//! Run with: cargo run --example serde_with_unit --features serde

#[cfg(feature = "serde")]
fn main() {
    use qtty::{Kilometers, Meters, Seconds};
    use serde::{Deserialize, Serialize};
    use serde_json;

    println!("=== Using serde_with_unit Helper ===\n");

    // =========================================================================
    // Example 1: Basic Usage - Mixed Serialization
    // =========================================================================
    println!("1. Basic Usage - Per-Field Control:\n");

    #[derive(Serialize, Deserialize, Debug)]
    struct SensorData {
        // This field will serialize WITH unit information
        #[serde(with = "qtty_core::serde_with_unit")]
        max_range: Meters,

        // This field uses default (compact) serialization
        current_distance: Meters,

        // Another field with unit info
        #[serde(with = "qtty_core::serde_with_unit")]
        timestamp: Seconds,
    }

    let data = SensorData {
        max_range: Meters::new(100.0),
        current_distance: Meters::new(42.5),
        timestamp: Seconds::new(1702562400.0),
    };

    let json = serde_json::to_string_pretty(&data).unwrap();
    println!("Serialized:\n{}\n", json);

    // Deserialize back
    let restored: SensorData = serde_json::from_str(&json).unwrap();
    println!("Deserialized successfully!");
    println!("  max_range: {}", restored.max_range);
    println!("  current_distance: {}", restored.current_distance);
    println!("  timestamp: {}\n", restored.timestamp);

    // =========================================================================
    // Example 2: External API Response
    // =========================================================================
    println!("2. Self-Documenting API Response:\n");

    #[derive(Serialize, Deserialize, Debug)]
    struct WeatherReport {
        station_id: String,

        #[serde(with = "qtty_core::serde_with_unit")]
        temperature_celsius: Meters, // Using Meters as example

        #[serde(with = "qtty_core::serde_with_unit")]
        wind_speed: Meters,

        #[serde(with = "qtty_core::serde_with_unit")]
        visibility: Kilometers,
    }

    let report = WeatherReport {
        station_id: "KJFK".to_string(),
        temperature_celsius: Meters::new(22.0),
        wind_speed: Meters::new(5.5),
        visibility: Kilometers::new(10.0),
    };

    let json = serde_json::to_string_pretty(&report).unwrap();
    println!("Weather API Response:\n{}\n", json);

    // =========================================================================
    // Example 3: Configuration File
    // =========================================================================
    println!("3. Configuration File Format:\n");

    #[derive(Serialize, Deserialize, Debug)]
    struct RobotConfig {
        name: String,

        // Document the units in config files
        #[serde(with = "qtty_core::serde_with_unit")]
        max_speed: Meters,

        #[serde(with = "qtty_core::serde_with_unit")]
        safe_distance: Meters,

        #[serde(with = "qtty_core::serde_with_unit")]
        timeout: Seconds,

        // Internal value doesn't need unit documentation
        calibration_offset: Meters,
    }

    let config = RobotConfig {
        name: "R2D2".to_string(),
        max_speed: Meters::new(2.5),
        safe_distance: Meters::new(0.5),
        timeout: Seconds::new(30.0),
        calibration_offset: Meters::new(0.001),
    };

    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("Robot Configuration:\n{}\n", json);

    // =========================================================================
    // Example 4: Unit Validation on Deserialization
    // =========================================================================
    println!("4. Unit Validation:\n");

    // Valid JSON with unit info
    #[derive(Serialize, Deserialize, Debug)]
    struct SingleValue {
        #[serde(with = "qtty_core::serde_with_unit")]
        distance: Meters,
    }

    let valid_json = r#"{"distance": {"value": 100.0, "unit": "m"}}"#;
    let data: SingleValue =
        serde_json::from_str(valid_json).expect("Should deserialize with matching unit");
    println!("✓ Valid JSON with correct unit: {}", data.distance);

    // Missing unit field (backwards compatible)
    let no_unit_json = r#"{"distance": {"value": 50.0}}"#;
    let data2: SingleValue =
        serde_json::from_str(no_unit_json).expect("Should work without unit field");
    println!("✓ Valid JSON without unit field: {}", data2.distance);

    // Wrong unit (will fail validation)
    let invalid_json = r#"{"distance": {"value": 100.0, "unit": "kg"}}"#;
    match serde_json::from_str::<SingleValue>(invalid_json) {
        Ok(_) => println!("✗ Unexpected success with wrong unit"),
        Err(e) => println!("✓ Correctly rejected wrong unit: {}", e),
    }
    println!();

    // =========================================================================
    // Example 5: Collections with Mixed Serialization
    // =========================================================================
    println!("5. Collections:\n");

    #[derive(Serialize, Deserialize, Debug)]
    struct Measurement {
        id: u32,

        #[serde(with = "qtty_core::serde_with_unit")]
        value: Meters,
    }

    let measurements = vec![
        Measurement {
            id: 1,
            value: Meters::new(10.0),
        },
        Measurement {
            id: 2,
            value: Meters::new(20.0),
        },
        Measurement {
            id: 3,
            value: Meters::new(30.0),
        },
    ];

    let json = serde_json::to_string_pretty(&measurements).unwrap();
    println!("Measurement Array:\n{}\n", json);

    // =========================================================================
    // Example 6: Nested Structures
    // =========================================================================
    println!("6. Nested Structures:\n");

    #[derive(Serialize, Deserialize, Debug)]
    struct Location {
        name: String,
        coordinates: Coordinates,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Coordinates {
        #[serde(with = "qtty_core::serde_with_unit")]
        x: Meters,

        #[serde(with = "qtty_core::serde_with_unit")]
        y: Meters,

        #[serde(with = "qtty_core::serde_with_unit")]
        z: Meters,
    }

    let location = Location {
        name: "Observatory".to_string(),
        coordinates: Coordinates {
            x: Meters::new(100.0),
            y: Meters::new(200.0),
            z: Meters::new(50.0),
        },
    };

    let json = serde_json::to_string_pretty(&location).unwrap();
    println!("Nested structure:\n{}\n", json);

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary ===\n");
    println!("✓ Use #[serde(with = \"qtty_core::serde_with_unit\")] for:");
    println!("  • External/public APIs");
    println!("  • Configuration files");
    println!("  • Self-documenting data");
    println!("  • When consumers might not know the units");
    println!();
    println!("✓ Use default serialization (no attribute) for:");
    println!("  • Internal APIs");
    println!("  • Performance-critical code");
    println!("  • Large datasets");
    println!("  • When types provide sufficient documentation");
    println!();
    println!("✓ Mix both in the same struct as needed!");
}

#[cfg(not(feature = "serde"))]
fn main() {
    println!("This example requires the 'serde' feature.");
    println!("Run with: cargo run --example serde_with_unit --features serde");
}
