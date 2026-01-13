//! Example showing how to use `qtty_core::Quantity` types with `serde` and (optionally) Tiberius DB.
//!
//! This demonstrates:
//! - Default serde behavior: `Quantity<U>` serializes as a raw `f64` by default
//! - `qtty_core::serde_with_unit`: serialize with unit information using `#[serde(with = "...")]`
//! - `tiberius` feature: bind/extract `Quantity<U>` directly (conceptual example)
//!
//! Run with:
//! - `cargo run -p qtty-core --example quantity_db_serde --features serde`
//! - `cargo run -p qtty-core --example quantity_db_serde --features serde,tiberius`

#[cfg(feature = "serde")]
use qtty_core::angular::Degrees;
#[cfg(feature = "serde")]
use qtty_core::time::Seconds;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// Example 1: Serde with default f64 serialization
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SchedulingConstraints {
    /// Minimum altitude in degrees (serializes as raw f64 by default)
    pub min_altitude: Degrees,

    /// Maximum altitude in degrees (serializes as raw f64 by default)
    pub max_altitude: Degrees,

    /// Minimum azimuth in degrees (serializes as raw f64 by default)
    pub min_azimuth: Degrees,

    /// Maximum azimuth in degrees (serializes as raw f64 by default)
    pub max_azimuth: Degrees,

    /// Minimum observation time in seconds (serializes as raw f64 by default)
    pub min_observation_time: Seconds,
}

#[cfg(feature = "serde")]
fn example_serde() {
    println!("=== Default Serde Example (compact f64) ===\n");

    // Create constraints with typed quantities
    let constraints = SchedulingConstraints {
        min_altitude: Degrees::new(30.0),
        max_altitude: Degrees::new(90.0),
        min_azimuth: Degrees::new(0.0),
        max_azimuth: Degrees::new(360.0),
        min_observation_time: Seconds::new(1200.0),
    };

    // Serialize to JSON (as raw f64 values - compact!)
    let json = serde_json::to_string_pretty(&constraints).unwrap();
    println!("Serialized JSON:\n{}\n", json);

    // Deserialize back
    let parsed: SchedulingConstraints = serde_json::from_str(&json).unwrap();
    println!("Deserialized successfully!");
    assert_eq!(
        constraints.min_altitude.value(),
        parsed.min_altitude.value()
    );
    println!("✓ Round-trip successful\n");
}

// ─────────────────────────────────────────────────────────────────────────────
// Example 1b: Serde with unit information (for self-documenting APIs)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ConfigWithUnits {
    /// Maximum speed with unit information preserved
    #[serde(with = "qtty_core::serde_with_unit")]
    pub max_altitude: Degrees,
}

#[cfg(feature = "serde")]
fn example_serde_with_unit() {
    println!("=== Serde with Unit Information (self-documenting) ===\n");

    let config = ConfigWithUnits {
        max_altitude: Degrees::new(90.0),
    };

    // Serialize to JSON with unit information
    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("Serialized JSON:\n{}\n", json);
    println!("✓ Unit symbol preserved in output for clarity!\n");

    // Deserialize back
    let parsed: ConfigWithUnits = serde_json::from_str(&json).unwrap();
    assert_eq!(config.max_altitude.value(), parsed.max_altitude.value());
    println!("✓ Round-trip successful\n");
}

// ─────────────────────────────────────────────────────────────────────────────
// Example 2: Direct DB usage (conceptual - requires actual DB connection)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(all(feature = "serde", feature = "tiberius"))]
fn example_tiberius_concept() {
    println!("=== Tiberius DB Example (conceptual) ===\n");

    // In real code, you would:
    // 1. Bind Quantity types directly to queries:
    println!("// Binding Quantity to query:");
    println!("let min_alt = Degrees::new(30.0);");
    println!("query.bind(min_alt);  // Works directly!\n");

    // 2. Extract Quantity types from result rows:
    println!("// Extracting Quantity from DB row:");
    println!("let altitude: Degrees = row.try_get(\"altitude\")?.unwrap();");
    println!("// No manual f64 → Degrees conversion needed!\n");
}

// ─────────────────────────────────────────────────────────────────────────────
// Example 3: Migration comparison
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
fn example_migration_comparison() {
    println!("=== Migration Comparison ===\n");

    // Old approach: store as f64, convert on access
    struct OldConstraints {
        min_alt: f64,
        max_alt: f64,
    }

    impl OldConstraints {
        fn min_alt(&self) -> Degrees {
            Degrees::new(self.min_alt)
        }

        fn max_alt(&self) -> Degrees {
            Degrees::new(self.max_alt)
        }
    }

    let old = OldConstraints {
        min_alt: 30.0,
        max_alt: 90.0,
    };

    println!("Old approach:");
    println!("  - Store: f64");
    println!("  - Access: .min_alt() -> Degrees");
    println!(
        "  - Values: {}..{} degrees\n",
        old.min_alt().value(),
        old.max_alt().value()
    );

    // New approach: store as Quantity directly
    #[derive(Serialize, Deserialize)]
    struct NewConstraints {
        min_alt: Degrees,
        max_alt: Degrees,
    }

    let new = NewConstraints {
        min_alt: Degrees::new(30.0),
        max_alt: Degrees::new(90.0),
    };

    println!("New approach:");
    println!("  - Store: Degrees (typed!)");
    println!("  - Access: .min_alt (direct)");
    println!(
        "  - Values: {}..{} degrees",
        new.min_alt.value(),
        new.max_alt.value()
    );
    println!("  ✓ No conversion methods needed!");
    println!("  ✓ Type safety at compile time!\n");
}

// ─────────────────────────────────────────────────────────────────────────────
// Main
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
fn run() {
    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║  qtty-core: Serde and DB Integration Examples         ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    example_serde();
    example_serde_with_unit();

    #[cfg(feature = "tiberius")]
    example_tiberius_concept();

    example_migration_comparison();

    println!("═══════════════════════════════════════════════════════");
    println!("All examples completed successfully!");
    println!("═══════════════════════════════════════════════════════\n");
}

fn main() {
    #[cfg(feature = "serde")]
    {
        run();
    }

    #[cfg(not(feature = "serde"))]
    {
        eprintln!("This example requires `--features serde`.");
    }
}
