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
//! - Supports multiple scalar types (`f64`, `f32`, and optionally `Rational64`, etc.).
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
//! use qtty::{Degree, Radian};
//!
//! let a = Degree::new(180.0);
//! let r = a.to::<qtty::unit::Radian>();
//! assert!((r.value() - core::f64::consts::PI).abs() < 1e-12);
//! ```
//!
//! Compose and use derived units (velocity = length / time):
//!
//! ```rust
//! use qtty::{Kilometer, Second};
//! use qtty::velocity::Velocity;
//!
//! let d = Kilometer::new(1_000.0);
//! let t = Second::new(100.0);
//! let v: Velocity<qtty::unit::Kilometer, qtty::unit::Second> = d / t;
//! assert!((v.value() - 10.0).abs() < 1e-12);
//! ```
//!
//! Using `f32` for memory efficiency:
//!
//! ```rust
//! use qtty::f32::{Degree, Meter};
//!
//! let angle: Degree = Degree::new(45.0_f32);
//! let distance: Meter = Meter::new(100.0_f32);
//! ```
//!
//! # Incorrect usage (type error)
//!
//! ```compile_fail
//! use qtty::{Kilometer, Second};
//!
//! let d = Kilometer::new(1.0);
//! let t = Second::new(1.0);
//! let _ = d + t; // cannot add different unit types
//! ```
//!
//! # Scalar Types
//!
//! The default scalar type is `f64`. You can use different scalar types:
//!
//! - `f64` (default) - double precision floating point
//! - `f32` - single precision floating point (use `qtty::f32::*`)
//! - `i8`, `i16`, `i32`, `i64`, `i128` - signed integers
//!   (use `qtty::i8::*`, `qtty::i16::*`, `qtty::i32::*`, `qtty::i64::*`, `qtty::i128::*`)
//! - `Rational64` - exact rational (feature `scalar-rational`)
//!
//! Integer quantities provide compile-time unit safety for discrete values.
//! They support basic arithmetic and lossy unit conversion via
//! [`to_lossy()`](crate::Quantity::to_lossy), but not the full [`to()`](crate::Quantity::to)
//! method (which requires floating-point semantics).
//!
//! # Modules
//!
//! Quantity aliases are exported at the crate root. Unit markers live under
//! `qtty::unit` for generic APIs such as `Quantity<U, S>` and `Velocity<N, D>`:
//!
//! - `qtty::velocity` (`Length / Time` aliases)
//! - `qtty::frequency` (`Angular / Time` aliases)
//! - `qtty::unit` (type-level unit markers)
//! - `qtty::f32` (all units with `f32` scalar)
//! - `qtty::f64` (all units with `f64` scalar - same as root)
//! - `qtty::i8` (all units with `i8` scalar)
//! - `qtty::i16` (all units with `i16` scalar)
//! - `qtty::i32` (all units with `i32` scalar)
//! - `qtty::i64` (all units with `i64` scalar)
//! - `qtty::i128` (all units with `i128` scalar)
//!
//! # Feature flags
//!
//! - `std` (default): enables `std` support in `qtty-core`.
//! - `cross-unit-ops` (default): enables direct cross-unit comparison operators (`==`, `<`, etc.) for built-in units.
//! - `alloc`: enables heap-backed helpers (like `qtty_vec!(vec ...)`) in `no_std` builds.
//! - `serde`: enables `serde` support for `Quantity<U>`; serialization is the raw `f64` value only.
//! - `scalar-rational`: enables `num_rational::Rational64` as a scalar type.
//!
//! Disable default features for `no_std`:
//!
//! ```toml
//! [dependencies]
//! qtty = { version = "0.5.0", default-features = false }
//! ```
//!
//! If you need `qtty_vec!(vec ...)` in `no_std`, enable `alloc`:
//!
//! ```toml
//! [dependencies]
//! qtty = { version = "0.5.0", default-features = false, features = ["alloc"] }
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

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

pub use qtty_core::{
    Acceleration, AmountOfSubstance, Angular, Area, Current, Dimension, Dimensionless, Energy,
    Exact, Force, FrequencyDim, IntegerScalar, Length, LuminousIntensity, Mass, Per, Power, Prod,
    Quantity, Quantity32, Quantity64, QuantityI128, QuantityI16, QuantityI32, QuantityI64,
    QuantityI8, Real, Scalar, Simplify, Temperature, Time, Transcendental, Unit, VelocityDim,
    Volume,
};

#[doc(hidden)]
pub use qtty_core::{Dim, DimDiv, DimMul};

#[cfg(feature = "scalar-rational")]
pub use qtty_core::QuantityRational;

#[cfg(all(feature = "serde", feature = "std"))]
pub use qtty_core::serde_with_unit;

#[cfg(feature = "serde")]
pub use qtty_core::serde_scalar;

/// Derive macro used by `qtty-core` to define unit marker types.
///
/// This macro expands in terms of `crate::Unit` and `crate::Quantity`, so it is intended for use inside `qtty-core`
/// (or crates exposing the same crate-root API). Most users should not need this.
pub use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Internal macro for scalar-specific modules
// ─────────────────────────────────────────────────────────────────────────────

#[macro_use]
mod scalar_aliases;

// ─────────────────────────────────────────────────────────────────────────────
// Scalar-specific modules
// ─────────────────────────────────────────────────────────────────────────────

pub mod f32;
pub mod f64;
pub mod i128;
pub mod i16;
pub mod i32;
pub mod i64;
pub mod i8;

// ─────────────────────────────────────────────────────────────────────────────
// Type-level unit markers
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level unit markers used by [`Quantity`].
///
/// The crate root exposes singular quantity aliases such as [`Meter`] and
/// [`Second`]. Use this module when generic code needs the unit marker itself,
/// for example `Quantity<unit::Meter, S>` or `to::<unit::Kilometer>()`.
pub mod unit {
    pub use qtty_core::{Per, Prod, Unit, Unitless};

    pub use qtty_core::units::angular::{
        AngularUnit, Arcminute, Arcsecond, Degree, Gradian, HourAngle, MicroArcsecond,
        MilliArcsecond, Milliradian, Radian, Turn,
    };
    pub use qtty_core::units::area::{
        Acre, Are, AreaUnit, Hectare, SquareCentimeter, SquareFoot, SquareInch, SquareKilometer,
        SquareMeter, SquareMile, SquareMillimeter, SquareYard,
    };
    pub use qtty_core::units::length::nominal::{
        EarthEquatorialRadius, EarthPolarRadius, EarthRadius, JupiterRadius, LunarDistance,
        LunarRadius, SolarDiameter, SolarRadius,
    };
    pub use qtty_core::units::length::{
        AstronomicalUnit, Attometer, BohrRadius, Centimeter, Chain, ClassicalElectronRadius,
        Decameter, Decimeter, EarthEquatorialCircumference, EarthMeridionalCircumference,
        ElectronReducedComptonWavelength, Exameter, Fathom, Femtometer, Foot, Gigameter,
        Gigaparsec, Hectometer, Inch, Kilometer, Kiloparsec, LengthUnit, LightYear, Link,
        Megameter, Megaparsec, Meter, Micrometer, Mile, Millimeter, Nanometer, NauticalMile,
        Parsec, Petameter, Picometer, PlanckLength, Rod, Terameter, Yard, Yoctometer, Yottameter,
        Zeptometer, Zettameter,
    };
    pub use qtty_core::units::mass::{
        AtomicMassUnit, Attogram, Carat, Centigram, Decagram, Decigram, Exagram, Femtogram,
        Gigagram, Grain, Gram, Hectogram, Kilogram, LongTon, MassUnit, Megagram, Microgram,
        Milligram, Nanogram, Ounce, Petagram, Picogram, Pound, ShortTon, SolarMass, Stone,
        Teragram, Tonne, Yoctogram, Yottagram, Zeptogram, Zettagram,
    };
    pub use qtty_core::units::power::{
        Attowatt, Decawatt, Deciwatt, ErgPerSecond, Exawatt, Femtowatt, Gigawatt, Hectowatt,
        HorsepowerElectric, HorsepowerMetric, Kilowatt, Megawatt, Microwatt, Milliwatt, Nanowatt,
        Petawatt, Picowatt, PowerUnit, SolarLuminosity, Terawatt, Watt, Yoctowatt, Yottawatt,
        Zeptowatt, Zettawatt,
    };
    pub use qtty_core::units::time::{
        Attosecond, Centisecond, Century, Day, Decade, Decasecond, Decisecond, Femtosecond,
        Fortnight, Gigasecond, Hectosecond, Hour, JulianCentury, JulianYear, Kilosecond,
        Megasecond, Microsecond, Millennium, Millisecond, Minute, Nanosecond, Picosecond, Second,
        SiderealDay, SiderealYear, SynodicMonth, Terasecond, TimeUnit, Week, Year,
    };
    pub use qtty_core::units::volume::{
        Centiliter, CubicCentimeter, CubicFoot, CubicInch, CubicKilometer, CubicMeter,
        CubicMillimeter, Deciliter, Liter, Microliter, Milliliter, UsFluidOunce, UsGallon,
        VolumeUnit,
    };
}

/// Velocity quantities represented as one unit divided by another.
pub mod velocity {
    pub use qtty_core::units::velocity::{Velocity, VelocityUnit};
}

/// Angular-frequency quantities represented as one unit divided by another.
pub mod frequency {
    pub use qtty_core::units::frequency::{Frequency, FrequencyUnit};
}

// ─────────────────────────────────────────────────────────────────────────────
// Convenience quantity aliases (default f64 scalar)
// ─────────────────────────────────────────────────────────────────────────────

scalar_aliases::define_scalar_aliases!(f64);

/// Dimensionless quantity alias.
pub type Unitless<S = f64> = Quantity<unit::Unitless, S>;

pub use qtty_core::units::angular::{DEG, RAD};
pub use qtty_core::units::length::{AU, KM, LY, M};
pub use qtty_core::units::time::{DAY, SEC};

pub use frequency::Frequency;
pub use velocity::Velocity;

#[doc(hidden)]
pub mod __private {
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;
}

/// Build typed quantities from scalar literals without repeating `Unit::new(...)`.
///
/// # Forms
///
/// - Array (const-friendly):
///   `qtty::qtty_vec!(Second; 1.0, 2.0, 3.0)`
/// - Vector:
///   `qtty::qtty_vec!(vec Second; 1.0, 2.0, 3.0)` (requires `std` or `alloc`)
///
/// # Examples
///
/// ```
/// use qtty::Second;
///
/// const OFFSETS: [Second; 3] = qtty::qtty_vec!(Second; 56.86, 63.83, 70.0);
/// assert_eq!(OFFSETS[1].value(), 63.83);
///
/// let samples: Vec<Second> = qtty::qtty_vec!(vec Second; 1.0, 2.0, 3.0);
/// assert_eq!(samples.len(), 3);
/// ```
#[cfg(any(feature = "std", feature = "alloc"))]
#[macro_export]
macro_rules! qtty_vec {
    (vec $unit:ty; $($value:expr),* $(,)?) => {
        <$crate::__private::Vec<$unit>>::from([$(<$unit>::new($value)),*])
    };
    ($unit:ty; $($value:expr),* $(,)?) => {
        [$(<$unit>::new($value)),*]
    };
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[macro_export]
macro_rules! qtty_vec {
    (vec $unit:ty; $($value:expr),* $(,)?) => {
        compile_error!(
            "`qtty::qtty_vec!(vec ...)` requires the `std` or `alloc` feature. \
Use `qtty::qtty_vec!(Unit; ...)` in pure `no_std` builds."
        )
    };
    ($unit:ty; $($value:expr),* $(,)?) => {
        [$(<$unit>::new($value)),*]
    };
}
