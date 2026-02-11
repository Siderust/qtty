//! Re-exports of quantity types specialized to `i16` scalar.
//!
//! This module provides type aliases for all unit types using `i16` as the
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
//! use qtty::i16::{Meters, Seconds};
//!
//! let distance: Meters = Meters::new(1500);
//! let time: Seconds = Seconds::new(10);
//! ```

use crate::Quantity;

// ─────────────────────────────────────────────────────────────────────────────
// Angular units (i16)
// ─────────────────────────────────────────────────────────────────────────────

/// Degrees with `i16` scalar.
pub type Degrees = Quantity<crate::angular::Degree, i16>;
/// Radians with `i16` scalar.
pub type Radians = Quantity<crate::angular::Radian, i16>;
/// Arcminutes with `i16` scalar.
pub type Arcminutes = Quantity<crate::angular::Arcminute, i16>;
/// Arcseconds with `i16` scalar.
pub type Arcseconds = Quantity<crate::angular::Arcsecond, i16>;
/// Milliradians with `i16` scalar.
pub type Milliradians = Quantity<crate::angular::Milliradian, i16>;
/// Milliarcseconds with `i16` scalar.
pub type MilliArcseconds = Quantity<crate::angular::MilliArcsecond, i16>;
/// Microarcseconds with `i16` scalar.
pub type MicroArcseconds = Quantity<crate::angular::MicroArcsecond, i16>;
/// Gradians with `i16` scalar.
pub type Gradians = Quantity<crate::angular::Gradian, i16>;
/// Turns with `i16` scalar.
pub type Turns = Quantity<crate::angular::Turn, i16>;
/// Hour angles with `i16` scalar.
pub type HourAngles = Quantity<crate::angular::HourAngle, i16>;

// ─────────────────────────────────────────────────────────────────────────────
// Length units (i16)
// ─────────────────────────────────────────────────────────────────────────────

/// Meters with `i16` scalar.
pub type Meters = Quantity<crate::length::Meter, i16>;
/// Kilometers with `i16` scalar.
pub type Kilometers = Quantity<crate::length::Kilometer, i16>;
/// Centimeters with `i16` scalar.
pub type Centimeters = Quantity<crate::length::Centimeter, i16>;
/// Millimeters with `i16` scalar.
pub type Millimeters = Quantity<crate::length::Millimeter, i16>;
/// Micrometers with `i16` scalar.
pub type Micrometers = Quantity<crate::length::Micrometer, i16>;
/// Nanometers with `i16` scalar.
pub type Nanometers = Quantity<crate::length::Nanometer, i16>;
/// Astronomical units with `i16` scalar.
pub type AstronomicalUnits = Quantity<crate::length::AstronomicalUnit, i16>;
/// Light years with `i16` scalar.
pub type LightYears = Quantity<crate::length::LightYear, i16>;
/// Parsecs with `i16` scalar.
pub type Parsecs = Quantity<crate::length::Parsec, i16>;

// ─────────────────────────────────────────────────────────────────────────────
// Time units (i16)
// ─────────────────────────────────────────────────────────────────────────────

/// Seconds with `i16` scalar.
pub type Seconds = Quantity<crate::time::Second, i16>;
/// Milliseconds with `i16` scalar.
pub type Milliseconds = Quantity<crate::time::Millisecond, i16>;
/// Microseconds with `i16` scalar.
pub type Microseconds = Quantity<crate::time::Microsecond, i16>;
/// Nanoseconds with `i16` scalar.
pub type Nanoseconds = Quantity<crate::time::Nanosecond, i16>;
/// Minutes with `i16` scalar.
pub type Minutes = Quantity<crate::time::Minute, i16>;
/// Hours with `i16` scalar.
pub type Hours = Quantity<crate::time::Hour, i16>;
/// Days with `i16` scalar.
pub type Days = Quantity<crate::time::Day, i16>;
/// Years with `i16` scalar.
pub type Years = Quantity<crate::time::Year, i16>;

// ─────────────────────────────────────────────────────────────────────────────
// Mass units (i16)
// ─────────────────────────────────────────────────────────────────────────────

/// Grams with `i16` scalar.
pub type Grams = Quantity<crate::mass::Gram, i16>;
/// Kilograms with `i16` scalar.
pub type Kilograms = Quantity<crate::mass::Kilogram, i16>;
/// Solar masses with `i16` scalar.
pub type SolarMasses = Quantity<crate::mass::SolarMass, i16>;

// ─────────────────────────────────────────────────────────────────────────────
// Power units (i16)
// ─────────────────────────────────────────────────────────────────────────────

/// Watts with `i16` scalar.
pub type Watts = Quantity<crate::power::Watt, i16>;
/// Kilowatts with `i16` scalar.
pub type Kilowatts = Quantity<crate::power::Kilowatt, i16>;
/// Solar luminosities with `i16` scalar.
pub type SolarLuminosities = Quantity<crate::power::SolarLuminosity, i16>;
