//! Re-exports of quantity types specialized to `i32` scalar.
//!
//! This module provides type aliases for all unit types using `i32` as the
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
//! use qtty::i32::{Meters, Seconds};
//!
//! let distance: Meters = Meters::new(1500);
//! let time: Seconds = Seconds::new(10);
//! ```

use crate::Quantity;

// ─────────────────────────────────────────────────────────────────────────────
// Angular units (i32)
// ─────────────────────────────────────────────────────────────────────────────

/// Degrees with `i32` scalar.
pub type Degrees = Quantity<crate::angular::Degree, i32>;
/// Radians with `i32` scalar.
pub type Radians = Quantity<crate::angular::Radian, i32>;
/// Arcminutes with `i32` scalar.
pub type Arcminutes = Quantity<crate::angular::Arcminute, i32>;
/// Arcseconds with `i32` scalar.
pub type Arcseconds = Quantity<crate::angular::Arcsecond, i32>;
/// Milliradians with `i32` scalar.
pub type Milliradians = Quantity<crate::angular::Milliradian, i32>;
/// Milliarcseconds with `i32` scalar.
pub type MilliArcseconds = Quantity<crate::angular::MilliArcsecond, i32>;
/// Microarcseconds with `i32` scalar.
pub type MicroArcseconds = Quantity<crate::angular::MicroArcsecond, i32>;
/// Gradians with `i32` scalar.
pub type Gradians = Quantity<crate::angular::Gradian, i32>;
/// Turns with `i32` scalar.
pub type Turns = Quantity<crate::angular::Turn, i32>;
/// Hour angles with `i32` scalar.
pub type HourAngles = Quantity<crate::angular::HourAngle, i32>;

// ─────────────────────────────────────────────────────────────────────────────
// Length units (i32)
// ─────────────────────────────────────────────────────────────────────────────

/// Meters with `i32` scalar.
pub type Meters = Quantity<crate::length::Meter, i32>;
/// Kilometers with `i32` scalar.
pub type Kilometers = Quantity<crate::length::Kilometer, i32>;
/// Centimeters with `i32` scalar.
pub type Centimeters = Quantity<crate::length::Centimeter, i32>;
/// Millimeters with `i32` scalar.
pub type Millimeters = Quantity<crate::length::Millimeter, i32>;
/// Micrometers with `i32` scalar.
pub type Micrometers = Quantity<crate::length::Micrometer, i32>;
/// Nanometers with `i32` scalar.
pub type Nanometers = Quantity<crate::length::Nanometer, i32>;
/// Astronomical units with `i32` scalar.
pub type AstronomicalUnits = Quantity<crate::length::AstronomicalUnit, i32>;
/// Light years with `i32` scalar.
pub type LightYears = Quantity<crate::length::LightYear, i32>;
/// Parsecs with `i32` scalar.
pub type Parsecs = Quantity<crate::length::Parsec, i32>;

// ─────────────────────────────────────────────────────────────────────────────
// Time units (i32)
// ─────────────────────────────────────────────────────────────────────────────

/// Seconds with `i32` scalar.
pub type Seconds = Quantity<crate::time::Second, i32>;
/// Milliseconds with `i32` scalar.
pub type Milliseconds = Quantity<crate::time::Millisecond, i32>;
/// Microseconds with `i32` scalar.
pub type Microseconds = Quantity<crate::time::Microsecond, i32>;
/// Nanoseconds with `i32` scalar.
pub type Nanoseconds = Quantity<crate::time::Nanosecond, i32>;
/// Minutes with `i32` scalar.
pub type Minutes = Quantity<crate::time::Minute, i32>;
/// Hours with `i32` scalar.
pub type Hours = Quantity<crate::time::Hour, i32>;
/// Days with `i32` scalar.
pub type Days = Quantity<crate::time::Day, i32>;
/// Years with `i32` scalar.
pub type Years = Quantity<crate::time::Year, i32>;

// ─────────────────────────────────────────────────────────────────────────────
// Mass units (i32)
// ─────────────────────────────────────────────────────────────────────────────

/// Grams with `i32` scalar.
pub type Grams = Quantity<crate::mass::Gram, i32>;
/// Kilograms with `i32` scalar.
pub type Kilograms = Quantity<crate::mass::Kilogram, i32>;
/// Solar masses with `i32` scalar.
pub type SolarMasses = Quantity<crate::mass::SolarMass, i32>;

// ─────────────────────────────────────────────────────────────────────────────
// Power units (i32)
// ─────────────────────────────────────────────────────────────────────────────

/// Watts with `i32` scalar.
pub type Watts = Quantity<crate::power::Watt, i32>;
/// Kilowatts with `i32` scalar.
pub type Kilowatts = Quantity<crate::power::Kilowatt, i32>;
/// Solar luminosities with `i32` scalar.
pub type SolarLuminosities = Quantity<crate::power::SolarLuminosity, i32>;
