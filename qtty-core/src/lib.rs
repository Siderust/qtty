// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Core type system for strongly typed physical quantities.
//!
//! `qtty-core` provides a minimal, zero-cost units model:
//!
//! - A *unit* is a zero-sized marker type implementing [`Unit`].
//! - A value tagged with a unit is a [`Quantity<U, S>`], where `S` is the scalar
//!   type (defaults to `f64`). Supported scalars include `f64`, `f32`, signed
//!   integers (`i8`–`i128`), and optionally `Rational64`.
//! - Conversion is an explicit, type-checked scaling via [`Quantity::to`] (for
//!   [`Real`] scalars) or [`Quantity::to_lossy`] (for [`Exact`] scalars).
//! - Derived units like velocity are expressed as [`Per<N, D>`] (e.g. `Meter/Second`).
//!
//! Most users should depend on `qtty` (the facade crate) unless they need direct access to these primitives.
//!
//! # What this crate solves
//!
//! - Compile-time separation of dimensions (length vs time vs angle, …).
//! - Zero runtime overhead for unit tags (phantom types only).
//! - Full dimensional arithmetic: `m * m → Prod<Meter, Meter>`,
//!   `m / s → Per<Meter, Second>`, `m / m → Quantity<Unitless>`.
//!   Named derived units (e.g. `SquareMeter`) are obtained via `.to()`.
//! - Automatic compile-time verification that multiplied/divided quantities
//!   produce the correct dimension.
//!
//! # What this crate does not try to solve
//!
//! - General-purpose symbolic simplification of arbitrary unit expressions.
//!   Products and quotients are structural (`Prod<A, B>`, `Per<A, B>`); use `.to()` to convert to named units.
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
//! qtty-core = { version = "0.7.0", default-features = false }
//! ```
//!
//! When `std` is disabled, floating-point math that isn't available in `core` is provided via `libm`.
//!
//! # Feature flags
//!
//! - `std` (default): enables `std` support.
//! - `cross-unit-ops` (default): enables direct cross-unit comparison operators (`==`, `<`, etc.) for built-in unit catalogs.
//! - `serde`: enables `serde` support for `Quantity<U, S>`; serialization is the raw scalar value.
//! - `pyo3`: enables PyO3 bindings for Python interop via `#[pyclass]` and `#[pymethods]`.
//!
//! # Panics and errors
//!
//! This crate does not define an error type and does not return `Result` from its core operations. For floating-point
//! scalars (`f64`, `f32`), arithmetic follows IEEE-754 behavior (NaN and infinities propagate). For integer
//! scalars, `abs()` uses saturating semantics at the minimum value (e.g. `i32::MIN.abs()` returns `i32::MAX`
//! instead of panicking). Standard integer overflow rules still apply to addition, subtraction, and multiplication.
//!
//! # SemVer and stability
//!
//! This crate is currently `0.x`. Expect breaking changes between minor versions until `1.0`.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![recursion_limit = "512"]

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
#[cfg(feature = "tiberius")]
mod feature_tiberius;
mod macros;
mod quantity;
pub mod scalar;
mod unit;
/// Stable unit arithmetic layer: [`UnitDiv`] and [`UnitMul`] traits.
pub mod unit_arithmetic;

// ─────────────────────────────────────────────────────────────────────────────
// Public re-exports of core types
// ─────────────────────────────────────────────────────────────────────────────

pub use dimension::{
    // Derived dimensions
    Acceleration,
    // Additional base dimensions (less commonly used)
    AmountOfSubstance,
    // Base dimensions
    Angular,
    AngularRate,
    Area,
    // New electrical/magnetic derived dimensions
    Capacitance,
    Charge,
    Current,
    // New density dimension
    Density,
    Dimension,
    Dimensionless,
    Energy,
    Force,
    // New frequency dimension
    Frequency,
    // New luminous-flux / illuminance dimensions
    Illuminance,
    // New magnetic dimensions
    Inductance,
    InverseSolidAngle,
    Length,
    LuminousFlux,
    LuminousIntensity,
    MagneticFlux,
    MagneticFluxDensity,
    Mass,
    PhotonRadiance,
    Power,
    Pressure,
    Radiance,
    // New resistance / voltage dimensions
    Resistance,
    SolidAngle,
    SpectralPhotonRadiance,
    SpectralRadiance,
    Temperature,
    Time,
    Velocity,
    Voltage,
    Volume,
};

// Implementation machinery — public for advanced downstream use but not part
// of the recommended API surface.
#[doc(hidden)]
pub use dimension::{Dim, DimDiv, DimMul};
pub use quantity::{
    Quantity, Quantity32, Quantity64, QuantityI128, QuantityI16, QuantityI32, QuantityI64,
    QuantityI8,
};
pub use scalar::{Exact, IntegerScalar, Real, Scalar, Transcendental};
pub use unit::{Per, Prod, Unit};
pub use unit_arithmetic::{QuantityDivOutput, SameDivOutput, UnitDiv, UnitMul, UnitSqrt};

#[cfg(feature = "scalar-rational")]
pub use quantity::QuantityRational;

#[cfg(all(feature = "serde", feature = "std"))]
pub use feature_serde::serde_with_unit;

#[cfg(feature = "serde")]
pub use feature_serde::serde_scalar;

// ─────────────────────────────────────────────────────────────────────────────
// Predefined unit modules (grouped by dimension)
// ─────────────────────────────────────────────────────────────────────────────

/// Predefined unit modules (grouped by dimension).
///
/// These are defined in `qtty-core` so they can implement formatting and helper traits without running into Rust's
/// orphan rules.
pub mod units;

pub use units::acceleration;
pub use units::angular;
pub use units::angular_rate;
pub use units::area;
pub use units::energy;
pub use units::force;
pub use units::length;
pub use units::mass;
#[cfg(feature = "photometry")]
pub use units::photometry;
pub use units::power;
pub use units::pressure;
#[cfg(feature = "radiometry")]
pub use units::radiometry;
pub use units::solid_angle;
pub use units::temperature;
pub use units::time;
pub use units::velocity;
pub use units::volume;

#[cfg(feature = "chemistry")]
pub use units::amount;
#[cfg(feature = "density")]
pub use units::density;
#[cfg(feature = "electrical")]
pub use units::electrical;
#[cfg(feature = "frequency")]
pub use units::frequency;
