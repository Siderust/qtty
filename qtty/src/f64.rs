//! Re-exports of quantity types specialized to `f64` scalar (default).
//!
//! This module provides type aliases for all unit types using `f64` as the
//! underlying scalar type. These are identical to the types exported at the
//! crate root, but are provided for symmetry with the [`crate::f32`] module.
//!
//! # Example
//!
//! ```rust
//! use qtty::f64::{Degrees, Meters, Seconds};
//!
//! let angle: Degrees = Degrees::new(90.0);
//! let distance: Meters = Meters::new(100.0);
//! let time: Seconds = Seconds::new(10.0);
//! ```

// Re-export the default (f64) types from qtty-core
pub use crate::angular::{
    Arcminutes, Arcseconds, Degrees, Gradians, HourAngles, MicroArcseconds, MilliArcseconds,
    Milliradians, Radians, Turns,
};
pub use crate::length::{
    AstronomicalUnits, Centimeters, Kilometers, LightYears, Meters, Micrometers, Millimeters,
    Nanometers, Parsecs,
};
pub use crate::mass::{Grams, Kilograms, SolarMasses};
pub use crate::power::{Kilowatts, SolarLuminosities, Watts};
pub use crate::time::{
    Days, Hours, Microseconds, Milliseconds, Minutes, Nanoseconds, Seconds, Years,
};
