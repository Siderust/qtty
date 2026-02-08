//! Strongly typed physical quantities and conversions.
//!
//! `qtty` is the user-facing crate in this workspace. It re-exports the full API from `qtty-core` plus a curated set
//! of predefined units (time, angles, lengths, …).
//!
//! The core idea is: a value is always a `Quantity<U, S>`, where `U` is a zero-sized type describing the unit and
//! `S` is the scalar type (defaults to `f64`). This keeps units at compile time with no runtime overhead beyond
//! the scalar's size.
//!
//! # What this crate solves
//!
//! - Prevents mixing incompatible dimensions (you can't add metres to seconds).
//! - Makes unit conversion explicit and type-checked (`to::<TargetUnit>()`).
//! - Provides a small set of astronomy-friendly units (AU, light-year, solar mass/luminosity, …).
//! - Supports multiple scalar types (`f64`, `f32`, and optionally `Decimal`, `Rational64`, etc.).
//!
//! # What this crate does not try to solve
//!
//! - Arbitrary symbolic unit algebra (e.g. `m^2 * s^-1`) or automatic simplification of arbitrary expressions.
//! - A full SI-prefix system; only the units defined in this crate are available out of the box.
//!
//! # Quick start
//!
//! Convert degrees to radians:
//!
//! ```rust
//! use qtty::{Degrees, Radian};
//!
//! let a = Degrees::new(180.0);
//! let r = a.to::<Radian>();
//! assert!((r.value() - core::f64::consts::PI).abs() < 1e-12);
//! ```
//!
//! Compose and use derived units (velocity = length / time):
//!
//! ```rust
//! use qtty::{Kilometer, Kilometers, Second, Seconds};
//! use qtty::velocity::Velocity;
//!
//! let d = Kilometers::new(1_000.0);
//! let t = Seconds::new(100.0);
//! let v: Velocity<Kilometer, Second> = d / t;
//! assert!((v.value() - 10.0).abs() < 1e-12);
//! ```
//!
//! Using `f32` for memory efficiency:
//!
//! ```rust
//! use qtty::f32::{Degrees, Meters};
//!
//! let angle: Degrees = Degrees::new(45.0_f32);
//! let distance: Meters = Meters::new(100.0_f32);
//! ```
//!
//! # Incorrect usage (type error)
//!
//! ```compile_fail
//! use qtty::{Kilometers, Seconds};
//!
//! let d = Kilometers::new(1.0);
//! let t = Seconds::new(1.0);
//! let _ = d + t; // cannot add different unit types
//! ```
//!
//! # Scalar Types
//!
//! The default scalar type is `f64`. You can use different scalar types:
//!
//! - `f64` (default) - double precision floating point
//! - `f32` - single precision floating point (use `qtty::f32::*`)
//! - `i8`, `i16`, `i32`, `i64`, `i128` - signed integers (use `qtty::i32::*`, `qtty::i64::*`, etc.)
//! - `Decimal` - exact decimal (feature `scalar-decimal`)
//! - `Rational64` - exact rational (feature `scalar-rational`)
//!
//! Integer quantities provide compile-time unit safety for discrete values.
//! They support basic arithmetic and lossy unit conversion via
//! [`to_lossy()`](crate::Quantity::to_lossy), but not the full [`to()`](crate::Quantity::to)
//! method (which requires floating-point semantics).
//!
//! # Modules
//!
//! Units are grouped by dimension under modules (also re-exported at the crate root for convenience):
//!
//! - `qtty::angular` (degrees, radians, arcseconds, wrapping/trigonometry helpers)
//! - `qtty::time` (seconds, days, years, …)
//! - `qtty::length` (metres, kilometres, AU, light-year, …)
//! - `qtty::mass` (grams, kilograms, solar mass)
//! - `qtty::power` (watts, solar luminosity)
//! - `qtty::velocity` (`Length / Time` aliases)
//! - `qtty::frequency` (`Angular / Time` aliases)
//! - `qtty::f32` (all units with `f32` scalar)
//! - `qtty::f64` (all units with `f64` scalar - same as root)
//! - `qtty::i32` (all units with `i32` scalar)
//! - `qtty::i64` (all units with `i64` scalar)
//!
//! # Feature flags
//!
//! - `std` (default): enables `std` support in `qtty-core`.
//! - `serde`: enables `serde` support for `Quantity<U>`; serialization is the raw `f64` value only.
//! - `scalar-decimal`: enables `rust_decimal::Decimal` as a scalar type.
//! - `scalar-rational`: enables `num_rational::Rational64` as a scalar type.
//!
//! Disable default features for `no_std`:
//!
//! ```toml
//! [dependencies]
//! qtty = { version = "0.2.0", default-features = false }
//! ```
//!
//! # Panics and errors
//!
//! This crate does not define an error type and does not return `Result` from its core operations. Conversions and
//! arithmetic are pure computations; they do not panic on their own, but they follow IEEE-754 behavior for floats
//! (NaN and infinities propagate according to the underlying operation).
//!
//! # SemVer and stability
//!
//! This workspace is currently `0.x`. Expect breaking changes between minor versions until `1.0`.
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

pub use qtty_core::*;

/// Derive macro used by `qtty-core` to define unit marker types.
///
/// This macro expands in terms of `crate::Unit` and `crate::Quantity`, so it is intended for use inside `qtty-core`
/// (or crates exposing the same crate-root API). Most users should not need this.
pub use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Scalar-specific modules
// ─────────────────────────────────────────────────────────────────────────────

pub mod f32;
pub mod f64;
pub mod i32;
pub mod i64;

// ─────────────────────────────────────────────────────────────────────────────
// Unit modules (grouped by dimension)
// ─────────────────────────────────────────────────────────────────────────────

pub use qtty_core::units::angular;
pub use qtty_core::units::frequency;
pub use qtty_core::units::length;
pub use qtty_core::units::mass;
pub use qtty_core::units::power;
pub use qtty_core::units::time;
pub use qtty_core::units::unitless;
pub use qtty_core::units::velocity;

// ─────────────────────────────────────────────────────────────────────────────
// Convenience re-exports (default f64 types)
// ─────────────────────────────────────────────────────────────────────────────

pub use qtty_core::units::angular::*;
pub use qtty_core::units::frequency::*;
pub use qtty_core::units::length::*;
pub use qtty_core::units::mass::*;
pub use qtty_core::units::power::*;
pub use qtty_core::units::time::*;
pub use qtty_core::units::velocity::*;
