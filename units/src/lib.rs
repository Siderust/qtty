//! # Units Module
//!
//! This module provides a comprehensive set of strongly-typed units and utilities
//! for astronomical and scientific calculations. It is designed to ensure correctness,
//! clarity, and ease of use when working with various units of measurement.
//!
//! ## Features
//! - **Time Units**: Includes representations for Days, Years, Julian Years, and Centuries.
//! - **Angular Units**: Provides types for Degrees, Radians, DMS (Degrees, Minutes, Seconds), HMS (HourAngles, Minutes, Seconds), and Arcseconds.
//! - **Length Units**: Includes types for meters and astronomical units (AstronomicalUnits).
//! - **Velocity Units**: Provides types for meters per second and kilometers per second.
//! - **Mass Units**: Includes types for kilograms and solar masses.
//! - **Power Units**: Includes types for watts and solar luminosity.
//! - **Arithmetic Operations**: Supports arithmetic operations between compatible units, ensuring type safety.
//!
//! ## Example Usage
//! ```rust
//! use siderust_units::*;
//!
//! // Angular Units
//! let degrees = Degrees::new(180.0);
//! let radians = degrees.to::<Radian>();
//! let dms = Degrees::from_dms(12, 34, 56.0);
//!
//! // Mass Units
//! let mass_kg = Kilograms::new(5.0);
//! let mass_solar = SolarMasses::new(2.0);
//!
//! // Conversions
//! let dms_to_decimal = dms.value();
//!
//! assert_eq!(radians.value(), std::f64::consts::PI);
//! ```
//!
//! ## Modules
//! - [`time`]: Time-related units and utilities.
//! - [`angular`]: Angular measurement units and utilities.
//! - [`length`]: Length units and utilities.
//! - [`velocity`]: Velocity-related units and utilities.
//! - [`mass`]: Mass-related units and utilities.
//! - [`power`]: Power-related units and utilities.

// Re-export all units-core types and modules
pub use units_core::*;

// Re-export the derive macro
pub use units_derive::Unit;

// Re-export unit modules - they're defined in units-core
pub use units_core::units::angular;
pub use units_core::units::frequency;
pub use units_core::units::length;
pub use units_core::units::mass;
pub use units_core::units::power;
pub use units_core::units::time;
pub use units_core::units::unitless;
pub use units_core::units::velocity;

// Re-export all types from unit modules for convenience
pub use units_core::units::angular::*;
pub use units_core::units::frequency::*;
pub use units_core::units::length::*;
pub use units_core::units::mass::*;
pub use units_core::units::power::*;
pub use units_core::units::time::*;
pub use units_core::units::velocity::*;
