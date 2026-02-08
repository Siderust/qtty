//! Re-exports of quantity types specialized to `i64` scalar.
//!
//! This module provides type aliases for all unit types using `i64` as the
//! underlying scalar type. Integer quantities provide compile-time unit safety
//! for discrete values (timestamps, counters, high-precision integer measurements, etc.).
//!
//! Integer quantities support basic arithmetic but **not** unit conversion via
//! [`to()`](crate::Quantity::to). Use [`to_lossy()`](crate::Quantity::to_lossy)
//! for lossy (truncating) conversion between units.
//!
//! # Example
//!
//! ```rust
//! use qtty::i64::{Meters, Nanoseconds};
//!
//! let distance: Meters = Meters::new(1_000_000);
//! let time: Nanoseconds = Nanoseconds::new(500_000_000);
//! ```

use crate::Quantity;

// ─────────────────────────────────────────────────────────────────────────────
// Angular units (i64)
// ─────────────────────────────────────────────────────────────────────────────

/// Degrees with `i64` scalar.
pub type Degrees = Quantity<crate::angular::Degree, i64>;
/// Radians with `i64` scalar.
pub type Radians = Quantity<crate::angular::Radian, i64>;
/// Arcminutes with `i64` scalar.
pub type Arcminutes = Quantity<crate::angular::Arcminute, i64>;
/// Arcseconds with `i64` scalar.
pub type Arcseconds = Quantity<crate::angular::Arcsecond, i64>;
/// Milliradians with `i64` scalar.
pub type Milliradians = Quantity<crate::angular::Milliradian, i64>;
/// Milliarcseconds with `i64` scalar.
pub type MilliArcseconds = Quantity<crate::angular::MilliArcsecond, i64>;
/// Microarcseconds with `i64` scalar.
pub type MicroArcseconds = Quantity<crate::angular::MicroArcsecond, i64>;
/// Gradians with `i64` scalar.
pub type Gradians = Quantity<crate::angular::Gradian, i64>;
/// Turns with `i64` scalar.
pub type Turns = Quantity<crate::angular::Turn, i64>;
/// Hour angles with `i64` scalar.
pub type HourAngles = Quantity<crate::angular::HourAngle, i64>;

// ─────────────────────────────────────────────────────────────────────────────
// Length units (i64)
// ─────────────────────────────────────────────────────────────────────────────

/// Meters with `i64` scalar.
pub type Meters = Quantity<crate::length::Meter, i64>;
/// Kilometers with `i64` scalar.
pub type Kilometers = Quantity<crate::length::Kilometer, i64>;
/// Centimeters with `i64` scalar.
pub type Centimeters = Quantity<crate::length::Centimeter, i64>;
/// Millimeters with `i64` scalar.
pub type Millimeters = Quantity<crate::length::Millimeter, i64>;
/// Micrometers with `i64` scalar.
pub type Micrometers = Quantity<crate::length::Micrometer, i64>;
/// Nanometers with `i64` scalar.
pub type Nanometers = Quantity<crate::length::Nanometer, i64>;
/// Astronomical units with `i64` scalar.
pub type AstronomicalUnits = Quantity<crate::length::AstronomicalUnit, i64>;
/// Light years with `i64` scalar.
pub type LightYears = Quantity<crate::length::LightYear, i64>;
/// Parsecs with `i64` scalar.
pub type Parsecs = Quantity<crate::length::Parsec, i64>;

// ─────────────────────────────────────────────────────────────────────────────
// Time units (i64)
// ─────────────────────────────────────────────────────────────────────────────

/// Seconds with `i64` scalar.
pub type Seconds = Quantity<crate::time::Second, i64>;
/// Milliseconds with `i64` scalar.
pub type Milliseconds = Quantity<crate::time::Millisecond, i64>;
/// Microseconds with `i64` scalar.
pub type Microseconds = Quantity<crate::time::Microsecond, i64>;
/// Nanoseconds with `i64` scalar.
pub type Nanoseconds = Quantity<crate::time::Nanosecond, i64>;
/// Minutes with `i64` scalar.
pub type Minutes = Quantity<crate::time::Minute, i64>;
/// Hours with `i64` scalar.
pub type Hours = Quantity<crate::time::Hour, i64>;
/// Days with `i64` scalar.
pub type Days = Quantity<crate::time::Day, i64>;
/// Years with `i64` scalar.
pub type Years = Quantity<crate::time::Year, i64>;

// ─────────────────────────────────────────────────────────────────────────────
// Mass units (i64)
// ─────────────────────────────────────────────────────────────────────────────

/// Grams with `i64` scalar.
pub type Grams = Quantity<crate::mass::Gram, i64>;
/// Kilograms with `i64` scalar.
pub type Kilograms = Quantity<crate::mass::Kilogram, i64>;
/// Solar masses with `i64` scalar.
pub type SolarMasses = Quantity<crate::mass::SolarMass, i64>;

// ─────────────────────────────────────────────────────────────────────────────
// Power units (i64)
// ─────────────────────────────────────────────────────────────────────────────

/// Watts with `i64` scalar.
pub type Watts = Quantity<crate::power::Watt, i64>;
/// Kilowatts with `i64` scalar.
pub type Kilowatts = Quantity<crate::power::Kilowatt, i64>;
/// Solar luminosities with `i64` scalar.
pub type SolarLuminosities = Quantity<crate::power::SolarLuminosity, i64>;
