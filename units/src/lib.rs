//! Strongly typed physical units and conversions.
//!
//! `unit` is the user-facing crate in this workspace. It re-exports the full API from `unit-core` plus a curated set
//! of predefined units (time, angles, lengths, …).
//!
//! The core idea is: a value is always a `Quantity<U>`, where `U` is a zero-sized type describing the unit. This keeps
//! units at compile time with no runtime overhead beyond an `f64`.
//!
//! # What this crate solves
//!
//! - Prevents mixing incompatible dimensions (you can’t add metres to seconds).
//! - Makes unit conversion explicit and type-checked (`to::<TargetUnit>()`).
//! - Provides a small set of astronomy-friendly units (AU, light-year, solar mass/luminosity, …).
//!
//! # What this crate does not try to solve
//!
//! - Arbitrary symbolic unit algebra (e.g. `m^2 * s^-1`) or automatic simplification of arbitrary expressions.
//! - Exact arithmetic: quantities are backed by `f64`.
//! - A full SI-prefix system; only the units defined in this crate are available out of the box.
//!
//! # Quick start
//!
//! Convert degrees to radians:
//!
//! ```rust
//! use unit::{Degrees, Radian};
//!
//! let a = Degrees::new(180.0);
//! let r = a.to::<Radian>();
//! assert!((r.value() - core::f64::consts::PI).abs() < 1e-12);
//! ```
//!
//! Compose and use derived units (velocity = length / time):
//!
//! ```rust
//! use unit::{Kilometers, KilometersPerSecond, Seconds};
//!
//! let d = Kilometers::new(1_000.0);
//! let t = Seconds::new(100.0);
//! let v: KilometersPerSecond = d / t;
//! assert!((v.value() - 10.0).abs() < 1e-12);
//! ```
//!
//! # Incorrect usage (type error)
//!
//! ```compile_fail
//! use unit::{Kilometers, Seconds};
//!
//! let d = Kilometers::new(1.0);
//! let t = Seconds::new(1.0);
//! let _ = d + t; // cannot add different unit types
//! ```
//!
//! # Modules
//!
//! Units are grouped by dimension under modules (also re-exported at the crate root for convenience):
//!
//! - `unit::angular` (degrees, radians, arcseconds, wrapping/trigonometry helpers)
//! - `unit::time` (seconds, days, years, …)
//! - `unit::length` (metres, kilometres, AU, light-year, …)
//! - `unit::mass` (grams, kilograms, solar mass)
//! - `unit::power` (watts, solar luminosity)
//! - `unit::velocity` (`Length / Time` aliases)
//! - `unit::frequency` (`Angular / Time` aliases)
//!
//! # Feature flags
//!
//! - `std` (default): enables `std` support in `unit-core`.
//! - `serde`: enables `serde` support for `Quantity<U>`; serialization is the raw `f64` value only.
//!
//! Disable default features for `no_std`:
//!
//! ```toml
//! [dependencies]
//! unit = { version = "0.1.0", default-features = false }
//! ```
//!
//! # Panics and errors
//!
//! This crate does not define an error type and does not return `Result` from its core operations. Conversions and
//! arithmetic are pure `f64` computations; they do not panic on their own, but they follow IEEE-754 behavior (NaN and
//! infinities propagate according to the underlying operation).
//!
//! # SemVer and stability
//!
//! This workspace is currently `0.x`. Expect breaking changes between minor versions until `1.0`.
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

pub use unit_core::*;

/// Derive macro used by `unit-core` to define unit marker types.
///
/// This macro expands in terms of `crate::Unit` and `crate::Quantity`, so it is intended for use inside `unit-core`
/// (or crates exposing the same crate-root API). Most users should not need this.
pub use unit_derive::Unit;

pub use unit_core::units::angular;
pub use unit_core::units::frequency;
pub use unit_core::units::length;
pub use unit_core::units::mass;
pub use unit_core::units::power;
pub use unit_core::units::time;
pub use unit_core::units::unitless;
pub use unit_core::units::velocity;

pub use unit_core::units::angular::*;
pub use unit_core::units::frequency::*;
pub use unit_core::units::length::*;
pub use unit_core::units::mass::*;
pub use unit_core::units::power::*;
pub use unit_core::units::time::*;
pub use unit_core::units::velocity::*;
