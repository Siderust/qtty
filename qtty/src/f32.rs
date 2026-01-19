//! Re-exports of quantity types specialized to `f32` scalar.
//!
//! This module provides type aliases for all unit types using `f32` as the
//! underlying scalar type. Use this when memory efficiency is more important
//! than precision.
//!
//! # Example
//!
//! ```rust
//! use qtty::f32::{Degrees, Meters, Seconds};
//!
//! let angle: Degrees = Degrees::new(90.0_f32);
//! let distance: Meters = Meters::new(100.0_f32);
//! let time: Seconds = Seconds::new(10.0_f32);
//! ```

use crate::Quantity;

// ─────────────────────────────────────────────────────────────────────────────
// Angular units (f32)
// ─────────────────────────────────────────────────────────────────────────────

/// Degrees with `f32` scalar.
pub type Degrees = Quantity<crate::angular::Degree, f32>;
/// Radians with `f32` scalar.
pub type Radians = Quantity<crate::angular::Radian, f32>;
/// Arcminutes with `f32` scalar.
pub type Arcminutes = Quantity<crate::angular::Arcminute, f32>;
/// Arcseconds with `f32` scalar.
pub type Arcseconds = Quantity<crate::angular::Arcsecond, f32>;
/// Milliradians with `f32` scalar.
pub type Milliradians = Quantity<crate::angular::Milliradian, f32>;
/// Milliarcseconds with `f32` scalar.
pub type MilliArcseconds = Quantity<crate::angular::MilliArcsecond, f32>;
/// Microarcseconds with `f32` scalar.
pub type MicroArcseconds = Quantity<crate::angular::MicroArcsecond, f32>;
/// Gradians with `f32` scalar.
pub type Gradians = Quantity<crate::angular::Gradian, f32>;
/// Turns with `f32` scalar.
pub type Turns = Quantity<crate::angular::Turn, f32>;
/// Hour angles with `f32` scalar.
pub type HourAngles = Quantity<crate::angular::HourAngle, f32>;

// ─────────────────────────────────────────────────────────────────────────────
// Length units (f32)
// ─────────────────────────────────────────────────────────────────────────────

/// Meters with `f32` scalar.
pub type Meters = Quantity<crate::length::Meter, f32>;
/// Kilometers with `f32` scalar.
pub type Kilometers = Quantity<crate::length::Kilometer, f32>;
/// Centimeters with `f32` scalar.
pub type Centimeters = Quantity<crate::length::Centimeter, f32>;
/// Millimeters with `f32` scalar.
pub type Millimeters = Quantity<crate::length::Millimeter, f32>;
/// Micrometers with `f32` scalar.
pub type Micrometers = Quantity<crate::length::Micrometer, f32>;
/// Nanometers with `f32` scalar.
pub type Nanometers = Quantity<crate::length::Nanometer, f32>;
/// Astronomical units with `f32` scalar.
pub type AstronomicalUnits = Quantity<crate::length::AstronomicalUnit, f32>;
/// Light years with `f32` scalar.
pub type LightYears = Quantity<crate::length::LightYear, f32>;
/// Parsecs with `f32` scalar.
pub type Parsecs = Quantity<crate::length::Parsec, f32>;

// ─────────────────────────────────────────────────────────────────────────────
// Time units (f32)
// ─────────────────────────────────────────────────────────────────────────────

/// Seconds with `f32` scalar.
pub type Seconds = Quantity<crate::time::Second, f32>;
/// Milliseconds with `f32` scalar.
pub type Milliseconds = Quantity<crate::time::Millisecond, f32>;
/// Microseconds with `f32` scalar.
pub type Microseconds = Quantity<crate::time::Microsecond, f32>;
/// Nanoseconds with `f32` scalar.
pub type Nanoseconds = Quantity<crate::time::Nanosecond, f32>;
/// Minutes with `f32` scalar.
pub type Minutes = Quantity<crate::time::Minute, f32>;
/// Hours with `f32` scalar.
pub type Hours = Quantity<crate::time::Hour, f32>;
/// Days with `f32` scalar.
pub type Days = Quantity<crate::time::Day, f32>;
/// Years with `f32` scalar.
pub type Years = Quantity<crate::time::Year, f32>;

// ─────────────────────────────────────────────────────────────────────────────
// Mass units (f32)
// ─────────────────────────────────────────────────────────────────────────────

/// Grams with `f32` scalar.
pub type Grams = Quantity<crate::mass::Gram, f32>;
/// Kilograms with `f32` scalar.
pub type Kilograms = Quantity<crate::mass::Kilogram, f32>;
/// Solar masses with `f32` scalar.
pub type SolarMasses = Quantity<crate::mass::SolarMass, f32>;

// ─────────────────────────────────────────────────────────────────────────────
// Power units (f32)
// ─────────────────────────────────────────────────────────────────────────────

/// Watts with `f32` scalar.
pub type Watts = Quantity<crate::power::Watt, f32>;
/// Kilowatts with `f32` scalar.
pub type Kilowatts = Quantity<crate::power::Kilowatt, f32>;
/// Solar luminosities with `f32` scalar.
pub type SolarLuminosities = Quantity<crate::power::SolarLuminosity, f32>;
