//! Re-exports of quantity types specialized to `i128` scalar.
//!
//! This module provides type aliases for all unit types using `i128` as the
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
//! use qtty::i128::{Meters, Seconds};
//!
//! let distance: Meters = Meters::new(1500);
//! let time: Seconds = Seconds::new(10);
//! ```

use crate::Quantity;

// ─────────────────────────────────────────────────────────────────────────────
// Angular units (i128)
// ─────────────────────────────────────────────────────────────────────────────

/// Degrees with `i128` scalar.
pub type Degrees = Quantity<crate::angular::Degree, i128>;
/// Radians with `i128` scalar.
pub type Radians = Quantity<crate::angular::Radian, i128>;
/// Arcminutes with `i128` scalar.
pub type Arcminutes = Quantity<crate::angular::Arcminute, i128>;
/// Arcseconds with `i128` scalar.
pub type Arcseconds = Quantity<crate::angular::Arcsecond, i128>;
/// Milliradians with `i128` scalar.
pub type Milliradians = Quantity<crate::angular::Milliradian, i128>;
/// Milliarcseconds with `i128` scalar.
pub type MilliArcseconds = Quantity<crate::angular::MilliArcsecond, i128>;
/// Microarcseconds with `i128` scalar.
pub type MicroArcseconds = Quantity<crate::angular::MicroArcsecond, i128>;
/// Gradians with `i128` scalar.
pub type Gradians = Quantity<crate::angular::Gradian, i128>;
/// Turns with `i128` scalar.
pub type Turns = Quantity<crate::angular::Turn, i128>;
/// Hour angles with `i128` scalar.
pub type HourAngles = Quantity<crate::angular::HourAngle, i128>;

// ─────────────────────────────────────────────────────────────────────────────
// Length units (i128)
// ─────────────────────────────────────────────────────────────────────────────

/// Meters with `i128` scalar.
pub type Meters = Quantity<crate::length::Meter, i128>;
/// Kilometers with `i128` scalar.
pub type Kilometers = Quantity<crate::length::Kilometer, i128>;
/// Centimeters with `i128` scalar.
pub type Centimeters = Quantity<crate::length::Centimeter, i128>;
/// Millimeters with `i128` scalar.
pub type Millimeters = Quantity<crate::length::Millimeter, i128>;
/// Micrometers with `i128` scalar.
pub type Micrometers = Quantity<crate::length::Micrometer, i128>;
/// Nanometers with `i128` scalar.
pub type Nanometers = Quantity<crate::length::Nanometer, i128>;
/// Astronomical units with `i128` scalar.
pub type AstronomicalUnits = Quantity<crate::length::AstronomicalUnit, i128>;
/// Light years with `i128` scalar.
pub type LightYears = Quantity<crate::length::LightYear, i128>;
/// Parsecs with `i128` scalar.
pub type Parsecs = Quantity<crate::length::Parsec, i128>;

// ─────────────────────────────────────────────────────────────────────────────
// Time units (i128)
// ─────────────────────────────────────────────────────────────────────────────

/// Seconds with `i128` scalar.
pub type Seconds = Quantity<crate::time::Second, i128>;
/// Milliseconds with `i128` scalar.
pub type Milliseconds = Quantity<crate::time::Millisecond, i128>;
/// Microseconds with `i128` scalar.
pub type Microseconds = Quantity<crate::time::Microsecond, i128>;
/// Nanoseconds with `i128` scalar.
pub type Nanoseconds = Quantity<crate::time::Nanosecond, i128>;
/// Minutes with `i128` scalar.
pub type Minutes = Quantity<crate::time::Minute, i128>;
/// Hours with `i128` scalar.
pub type Hours = Quantity<crate::time::Hour, i128>;
/// Days with `i128` scalar.
pub type Days = Quantity<crate::time::Day, i128>;
/// Years with `i128` scalar.
pub type Years = Quantity<crate::time::Year, i128>;

// ─────────────────────────────────────────────────────────────────────────────
// Mass units (i128)
// ─────────────────────────────────────────────────────────────────────────────

/// Grams with `i128` scalar.
pub type Grams = Quantity<crate::mass::Gram, i128>;
/// Kilograms with `i128` scalar.
pub type Kilograms = Quantity<crate::mass::Kilogram, i128>;
/// Solar masses with `i128` scalar.
pub type SolarMasses = Quantity<crate::mass::SolarMass, i128>;

// ─────────────────────────────────────────────────────────────────────────────
// Power units (i128)
// ─────────────────────────────────────────────────────────────────────────────

/// Watts with `i128` scalar.
pub type Watts = Quantity<crate::power::Watt, i128>;
/// Kilowatts with `i128` scalar.
pub type Kilowatts = Quantity<crate::power::Kilowatt, i128>;
/// Solar luminosities with `i128` scalar.
pub type SolarLuminosities = Quantity<crate::power::SolarLuminosity, i128>;
