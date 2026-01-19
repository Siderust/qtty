//! Example showing how to use `qtty_core::Quantity` types with Diesel ORM.
//!
//! This example demonstrates:
//! - Using `Quantity<U>` types (e.g., `Degrees`, `Meters`) directly in Diesel models
//! - Automatic serialization/deserialization to/from SQL DOUBLE PRECISION columns
//! - Type-safe queries with strongly-typed physical quantities
//!
//! # Running this example
//!
//! ```bash
//! cargo run -p qtty-core --example diesel_integration --features diesel
//! ```
//!
//! Note: This is a conceptual example showing the API usage. For a real database integration,
//! you would need to set up an actual database connection and schema.

#[cfg(feature = "diesel")]
fn main() {
    use qtty_core::angular::Degrees;
    use qtty_core::length::Meters;
    use qtty_core::Quantity;

    println!("=== Diesel ORM Integration Example ===\n");

    // Example 1: Conceptual Diesel model using Quantity types
    println!("1. Diesel Model with Quantity Types:\n");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Observatory {
        id: i32,
        name: String,
        latitude: Degrees,  // Instead of f64!
        longitude: Degrees, // Type-safe angles
        elevation: Meters,  // Type-safe length
    }

    let obs = Observatory {
        id: 1,
        name: "Mauna Kea".to_string(),
        latitude: Degrees::new(19.8207),
        longitude: Degrees::new(-155.4681),
        elevation: Meters::new(4207.0),
    };

    println!("Observatory: {}", obs.name);
    println!(
        "  Location: {:.4}° N, {:.4}° W",
        obs.latitude.value(),
        obs.longitude.value().abs()
    );
    println!("  Elevation: {:.0} m", obs.elevation.value());

    // Example 2: Type conversion
    println!("\n2. Type Conversions:");
    use qtty_core::angular::Radians;
    use qtty_core::length::Kilometers;

    let lat_rad: Radians = obs.latitude.to();
    let elev_km: Kilometers = obs.elevation.to();

    println!("  Latitude: {:.6} rad", lat_rad.value());
    println!("  Elevation: {:.3} km", elev_km.value());

    // Example 3: Conceptual schema definition
    println!("\n3. Conceptual Diesel Schema:");
    println!(
        r#"
    // In your Diesel schema.rs:
    table! {{
        observatories (id) {{
            id -> Int4,
            name -> Text,
            latitude -> Float8,   // Maps to qtty::Degrees
            longitude -> Float8,  // Maps to qtty::Degrees
            elevation -> Float8,  // Maps to qtty::Meters
        }}
    }}

    // In your models.rs:
    use qtty::{{Degrees, Meters}};
    
    #[derive(Queryable, Selectable)]
    #[diesel(table_name = observatories)]
    pub struct Observatory {{
        pub id: i32,
        pub name: String,
        pub latitude: Degrees,   // Direct use of Quantity types!
        pub longitude: Degrees,
        pub elevation: Meters,
    }}

    #[derive(Insertable)]
    #[diesel(table_name = observatories)]
    pub struct NewObservatory {{
        pub name: String,
        pub latitude: Degrees,
        pub longitude: Degrees,
        pub elevation: Meters,
    }}
    "#
    );

    // Example 4: Optional fields
    println!("\n4. Optional Quantity Fields:");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Telescope {
        id: i32,
        name: String,
        min_altitude: Option<Degrees>, // Optional constraint
        max_altitude: Option<Degrees>,
    }

    let telescope = Telescope {
        id: 1,
        name: "Keck I".to_string(),
        min_altitude: Some(Degrees::new(15.0)),
        max_altitude: Some(Degrees::new(85.0)),
    };

    println!("Telescope: {}", telescope.name);
    if let Some(min_alt) = telescope.min_altitude {
        println!("  Min altitude: {}°", min_alt.value());
    }
    if let Some(max_alt) = telescope.max_altitude {
        println!("  Max altitude: {}°", max_alt.value());
    }

    // Example 5: Benefits summary
    println!("\n5. Benefits of Using Quantity Types:");
    println!("  ✓ Compile-time unit checking");
    println!("  ✓ No manual unit conversions in database layer");
    println!("  ✓ Self-documenting code (field type shows units)");
    println!("  ✓ Automatic serialization/deserialization");
    println!("  ✓ Zero runtime overhead (newtype pattern)");

    println!("\n=== Example Complete ===");
}

#[cfg(not(feature = "diesel"))]
fn main() {
    println!("This example requires the 'diesel' feature to be enabled.");
    println!("Run with: cargo run -p qtty-core --example diesel_integration --features diesel");
}
