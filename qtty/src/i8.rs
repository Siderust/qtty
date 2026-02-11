//! Re-exports of quantity types specialized to `i8` scalar.
//!
//! This module provides type aliases for all unit types using `i8` as the
//! underlying scalar type. Integer quantities provide compile-time unit safety
//! for discrete values (counters, pixel coordinates, ADC readings, etc.).
//!
//! Integer quantities support basic arithmetic but **not** unit conversion via
//! [`to()`](crate::Quantity::to). Use [`to_lossy()`](crate::Quantity::to_lossy)
//! for lossy (truncating) conversion between units.
//!
//! # Example
//!
//! ```rust
//! use qtty::i8::{Meters, Seconds};
//!
//! let distance: Meters = Meters::new(120);
//! let time: Seconds = Seconds::new(10);
//! ```

use crate::Quantity;

// ─────────────────────────────────────────────────────────────────────────────
// Angular units (i8)
// ─────────────────────────────────────────────────────────────────────────────

/// Degrees with `i8` scalar.
pub type Degrees = Quantity<crate::angular::Degree, i8>;
/// Radians with `i8` scalar.
pub type Radians = Quantity<crate::angular::Radian, i8>;
/// Arcminutes with `i8` scalar.
pub type Arcminutes = Quantity<crate::angular::Arcminute, i8>;
/// Arcseconds with `i8` scalar.
pub type Arcseconds = Quantity<crate::angular::Arcsecond, i8>;
/// Milliradians with `i8` scalar.
pub type Milliradians = Quantity<crate::angular::Milliradian, i8>;
/// Milliarcseconds with `i8` scalar.
pub type MilliArcseconds = Quantity<crate::angular::MilliArcsecond, i8>;
/// Microarcseconds with `i8` scalar.
pub type MicroArcseconds = Quantity<crate::angular::MicroArcsecond, i8>;
/// Gradians with `i8` scalar.
pub type Gradians = Quantity<crate::angular::Gradian, i8>;
/// Turns with `i8` scalar.
pub type Turns = Quantity<crate::angular::Turn, i8>;
/// Hour angles with `i8` scalar.
pub type HourAngles = Quantity<crate::angular::HourAngle, i8>;

// ─────────────────────────────────────────────────────────────────────────────
// Length units (i8)
// ─────────────────────────────────────────────────────────────────────────────

/// Meters with `i8` scalar.
pub type Meters = Quantity<crate::length::Meter, i8>;
/// Kilometers with `i8` scalar.
pub type Kilometers = Quantity<crate::length::Kilometer, i8>;
/// Centimeters with `i8` scalar.
pub type Centimeters = Quantity<crate::length::Centimeter, i8>;
/// Millimeters with `i8` scalar.
pub type Millimeters = Quantity<crate::length::Millimeter, i8>;
/// Micrometers with `i8` scalar.
pub type Micrometers = Quantity<crate::length::Micrometer, i8>;
/// Nanometers with `i8` scalar.
pub type Nanometers = Quantity<crate::length::Nanometer, i8>;
/// Astronomical units with `i8` scalar.
pub type AstronomicalUnits = Quantity<crate::length::AstronomicalUnit, i8>;
/// Light years with `i8` scalar.
pub type LightYears = Quantity<crate::length::LightYear, i8>;
/// Parsecs with `i8` scalar.
pub type Parsecs = Quantity<crate::length::Parsec, i8>;

// ─────────────────────────────────────────────────────────────────────────────
// Time units (i8)
// ─────────────────────────────────────────────────────────────────────────────

/// Seconds with `i8` scalar.
pub type Seconds = Quantity<crate::time::Second, i8>;
/// Milliseconds with `i8` scalar.
pub type Milliseconds = Quantity<crate::time::Millisecond, i8>;
/// Microseconds with `i8` scalar.
pub type Microseconds = Quantity<crate::time::Microsecond, i8>;
/// Nanoseconds with `i8` scalar.
pub type Nanoseconds = Quantity<crate::time::Nanosecond, i8>;
/// Minutes with `i8` scalar.
pub type Minutes = Quantity<crate::time::Minute, i8>;
/// Hours with `i8` scalar.
pub type Hours = Quantity<crate::time::Hour, i8>;
/// Days with `i8` scalar.
pub type Days = Quantity<crate::time::Day, i8>;
/// Years with `i8` scalar.
pub type Years = Quantity<crate::time::Year, i8>;

// ─────────────────────────────────────────────────────────────────────────────
// Mass units (i8)
// ─────────────────────────────────────────────────────────────────────────────

/// Grams with `i8` scalar.
pub type Grams = Quantity<crate::mass::Gram, i8>;
/// Kilograms with `i8` scalar.
pub type Kilograms = Quantity<crate::mass::Kilogram, i8>;
/// Solar masses with `i8` scalar.
pub type SolarMasses = Quantity<crate::mass::SolarMass, i8>;

// ─────────────────────────────────────────────────────────────────────────────
// Power units (i8)
// ─────────────────────────────────────────────────────────────────────────────

/// Watts with `i8` scalar.
pub type Watts = Quantity<crate::power::Watt, i8>;
/// Kilowatts with `i8` scalar.
pub type Kilowatts = Quantity<crate::power::Kilowatt, i8>;
/// Solar luminosities with `i8` scalar.
pub type SolarLuminosities = Quantity<crate::power::SolarLuminosity, i8>;
