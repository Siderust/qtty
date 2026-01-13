//! Core type system for strongly typed physical quantities.
//!
//! `qtty-core` provides a minimal, zero-cost units model:
//!
//! - A *unit* is a zero-sized marker type implementing [`Unit`].
//! - A value tagged with a unit is a [`Quantity<U>`], backed by an `f64`.
//! - Conversion is an explicit, type-checked scaling via [`Quantity::to`].
//! - Derived units like velocity are expressed as [`Per<N, D>`] (e.g. `Meter/Second`).
//!
//! Most users should depend on `qtty` (the facade crate) unless they need direct access to these primitives.
//!
//! # What this crate solves
//!
//! - Compile-time separation of dimensions (length vs time vs angle, …).
//! - Zero runtime overhead for unit tags (phantom types only).
//! - A small vocabulary to express derived units via type aliases (`Per`, `DivDim`).
//!
//! # What this crate does not try to solve
//!
//! - Exact arithmetic (`Quantity` is `f64`).
//! - General-purpose symbolic simplification of arbitrary unit expressions.
//! - Automatic tracking of exponent dimensions (`m^2`, `s^-1`, …); only the expression forms represented by the
//!   provided types are modeled.
//!
//! # Quick start
//!
//! Convert between predefined units:
//!
//! ```rust
//! use qtty_core::length::{Kilometers, Meter};
//!
//! let km = Kilometers::new(1.25);
//! let m = km.to::<Meter>();
//! assert!((m.value() - 1250.0).abs() < 1e-12);
//! ```
//!
//! Compose derived units using `/`:
//!
//! ```rust
//! use qtty_core::length::{Meter, Meters};
//! use qtty_core::time::{Second, Seconds};
//! use qtty_core::velocity::Velocity;
//!
//! let d = Meters::new(100.0);
//! let t = Seconds::new(20.0);
//! let v: Velocity<Meter, Second> = d / t;
//! assert!((v.value() - 5.0).abs() < 1e-12);
//! ```
//!
//! # `no_std`
//!
//! Disable default features to build `qtty-core` without `std`:
//!
//! ```toml
//! [dependencies]
//! qtty-core = { version = "0.1.0", default-features = false }
//! ```
//!
//! When `std` is disabled, floating-point math that isn't available in `core` is provided via `libm`.
//!
//! # Feature flags
//!
//! - `std` (default): enables `std` support.
//! - `serde`: enables `serde` support for `Quantity<U>`; serialization is the raw `f64` value only.
//! - `pyo3`: enables PyO3 bindings for Python interop via `#[pyclass]` and `#[pymethods]`.
//!
//! # Panics and errors
//!
//! This crate does not define an error type and does not return `Result` from its core operations. Conversions and
//! arithmetic are pure `f64` computations; they do not panic on their own, but they follow IEEE-754 behavior (NaN and
//! infinities propagate according to the underlying operation).
//!
//! # SemVer and stability
//!
//! This crate is currently `0.x`. Expect breaking changes between minor versions until `1.0`.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(not(feature = "std"))]
extern crate libm;

// ─────────────────────────────────────────────────────────────────────────────
// Core modules
// ─────────────────────────────────────────────────────────────────────────────

mod dimension;
#[cfg(feature = "diesel")]
mod feature_diesel;
#[cfg(feature = "pyo3")]
mod feature_pyo3;
#[cfg(feature = "serde")]
mod feature_serde;
mod macros;
mod quantity;
mod unit;

// ─────────────────────────────────────────────────────────────────────────────
// Public re-exports of core types
// ─────────────────────────────────────────────────────────────────────────────

pub use dimension::{Dimension, Dimensionless, DivDim};
pub use quantity::Quantity;
pub use unit::{Per, Simplify, Unit, Unitless};

#[cfg(feature = "serde")]
pub use feature_serde::serde_with_unit;

// ─────────────────────────────────────────────────────────────────────────────
// Predefined unit modules (grouped by dimension)
// ─────────────────────────────────────────────────────────────────────────────

/// Predefined unit modules (grouped by dimension).
pub mod units;

pub use units::angular;
pub use units::frequency;
pub use units::length;
pub use units::mass;
pub use units::power;
pub use units::time;
pub use units::unitless;
pub use units::velocity;
