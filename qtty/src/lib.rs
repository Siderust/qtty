// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

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
//! - `qtty::angular_rate` (`Angular / Time` aliases)
//! - `qtty::area`, `qtty::volume`, `qtty::force`, `qtty::energy`
//! - `qtty::accel` (`Length / Time²` aliases)
//! - `qtty::pressure` (pressure units: Pa, hPa, kPa, bar)
//! - `qtty::temperature` (thermodynamic temperature units: K)
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
//! - `serde`: enables `serde` support for `Quantity<U, S>`; serialization is the raw scalar value.
//! - `scalar-rational`: enables `num_rational::Rational64` as a scalar type.
//!
//! # Custom Units
//!
//! `qtty` re-exports the derive macro plus the arithmetic/conversion macros
//! from `qtty-core`, so downstream crates can define units without depending on
//! `qtty-core` directly:
//!
//! ```ignore
//! #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, qtty::Unit)]
//! #[unit(crate = qtty, symbol = "smoot", dimension = qtty::Length, ratio = 1.7018)]
//! pub struct Smoot;
//!
//! qtty::impl_unit_arithmetic_pairs_between!(qtty::unit::Meter, qtty::unit::Kilometer; Smoot);
//! ```
//!
//! ## Limitations of downstream units
//!
//! Custom units participate in arithmetic (`*`, `/`) and `From`-based
//! conversion via the macro registrations shown above.  However, due to Rust's
//! orphan rules, the derive macro cannot generate `Display`/`LowerExp`/`UpperExp`
//! impls for `Quantity<CustomUnit, S>` from a downstream crate.  Downstream
//! units must implement formatting manually if needed.
//!
//! Disable default features for `no_std`:
//!
//! ```toml
//! [dependencies]
//! qtty = { version = "0.6.1", default-features = false }
//! ```
//!
//! If you need `qtty_vec!(vec ...)` in `no_std`, enable `alloc`:
//!
//! ```toml
//! [dependencies]
//! qtty = { version = "0.6.1", default-features = false, features = ["alloc"] }
//! ```
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
//! This workspace is currently `0.x`. Expect breaking changes between minor versions until `1.0`.
//!
//! The following items are **explicitly excluded from the semver guarantee**
//! even though they appear in the compiled crate:
//!
//! - [`Dim`], [`DimDiv`], [`DimMul`] — typenum-driven dimension markers used
//!   internally by `impl_unit_*` macros.  Their shape will change if the
//!   underlying numeric representation is ever replaced.
//! - [`UnitDiv`], [`UnitMul`] — arithmetic-layer traits whose associated-type
//!   signatures depend on the dimension representation above.
//!
//! All five are marked `#[doc(hidden)]`.  Do not depend on their concrete
//! types in downstream code.
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

pub use qtty_core::{
    impl_unit_arithmetic_pairs, impl_unit_arithmetic_pairs_between, impl_unit_cross_unit_ops,
    impl_unit_cross_unit_ops_between, impl_unit_division_pairs, impl_unit_division_pairs_between,
    impl_unit_from_conversions, impl_unit_from_conversions_between, impl_unit_multiplication_pairs,
    impl_unit_multiplication_pairs_between,
};
pub use qtty_core::{
    Acceleration, AmountOfSubstance, Angular, AngularRate, Area, Capacitance, Charge, Current,
    Density, Dimension, Dimensionless, Energy, Exact, Force, Frequency, Illuminance, Inductance,
    IntegerScalar, Length, LuminousFlux, LuminousIntensity, MagneticFlux, MagneticFluxDensity,
    Mass, Per, Power, Pressure, Prod, Quantity, Quantity32, Quantity64, QuantityI128, QuantityI16,
    QuantityI32, QuantityI64, QuantityI8, Real, Resistance, Scalar, Temperature, Time,
    Transcendental, Unit, Velocity, Voltage, Volume,
};

// `UnitDiv`, `UnitMul`, and the dimension-level traits are needed by the
// `impl_unit_*` macros when they expand in downstream crates.  They are
// also useful for writing generic code, but their associated types expose
// typenum-based dimension markers.
//
// # Stability note
//
// `Dim`, `DimDiv`, `DimMul`, `UnitDiv`, and `UnitMul` are **excluded from
// the semver stability guarantee** of this crate even though they are
// technically re-exported.  They are marked `#[doc(hidden)]` because they
// are implementation details of the macro-generated arithmetic layer.
// Specifically:
//
// * `Dim` and its `DimDiv`/`DimMul` associated types are driven by
//   `typenum` integers.  Replacing `typenum` would be a breaking change to
//   these types, and that replacement is not considered a breaking change
//   for `qtty`'s public API.
// * The `impl_unit_*` macros re-export these at their expansion sites so
//   downstream users are never expected to name them directly.
//
// If you find yourself writing `use qtty::{Dim, DimDiv, DimMul}` in
// application code, consider opening an issue — that is a sign of a missing
// ergonomic abstraction in the public API.
#[doc(hidden)]
pub use qtty_core::{Dim, DimDiv, DimMul, UnitDiv, UnitMul, UnitSqrt};

#[cfg(feature = "scalar-rational")]
pub use qtty_core::QuantityRational;

#[cfg(all(feature = "serde", feature = "std"))]
pub use qtty_core::serde_with_unit;

#[cfg(feature = "serde")]
pub use qtty_core::serde_scalar;

/// Derive macro used to define unit marker types.
///
/// Inside `qtty-core`, `#[derive(Unit)]` works with the default
/// `#[unit(symbol = ..., dimension = ..., ratio = ...)]` form. Downstream crates
/// should target the public facade explicitly:
///
/// ```ignore
/// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, qtty::Unit)]
/// #[unit(crate = qtty, symbol = "smoot", dimension = qtty::Length, ratio = 1.7018)]
/// pub struct Smoot;
/// ```
///
/// Pair this with `qtty::impl_unit_arithmetic_pairs!` or
/// `qtty::impl_unit_arithmetic_pairs_between!` when you want custom units to
/// participate in `*` and `/` result-type inference.
pub use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Internal macros for inventory-driven generation
// ─────────────────────────────────────────────────────────────────────────────

/// Invoke a callback macro with every qtty-core dimension inventory.
///
/// The callback is invoked once per always-available dimension family (11
/// total). This is usable for
/// additive generation (aliases, assertions) where the callback doesn't need
/// to know which dimension/module the units come from.
#[doc(hidden)]
#[macro_export]
macro_rules! __qtty_invoke_all_inventories {
    ($cb:path) => {
        qtty_core::angular_units!($cb);
        qtty_core::length_units!($cb);
        qtty_core::time_units!($cb);
        qtty_core::mass_units!($cb);
        qtty_core::power_units!($cb);
        qtty_core::area_units!($cb);
        qtty_core::solid_angle_units!($cb);
        qtty_core::volume_units!($cb);
        qtty_core::acceleration_units!($cb);
        qtty_core::force_units!($cb);
        qtty_core::energy_units!($cb);
        qtty_core::pressure_units!($cb);
        qtty_core::temperature_units!($cb);
    };
}

/// Invoke a callback macro with feature-gated qtty-core dimension inventories.
///
/// This keeps the crate-root quantity aliases aligned with the unit markers
/// re-exported from [`crate::unit`] whenever optional unit families are enabled.
#[doc(hidden)]
#[macro_export]
macro_rules! __qtty_invoke_optional_inventories {
    ($cb:path) => {
        #[cfg(feature = "astro")]
        qtty_core::angular_astro_units!($cb);
        #[cfg(feature = "astro")]
        qtty_core::length_astro_units!($cb);
        #[cfg(feature = "astro")]
        qtty_core::length_nominal_units!($cb);
        #[cfg(feature = "astro")]
        qtty_core::mass_astro_units!($cb);
        #[cfg(feature = "astro")]
        qtty_core::power_astro_units!($cb);
        #[cfg(feature = "astro")]
        qtty_core::solid_angle_astro_units!($cb);
        #[cfg(feature = "astro")]
        qtty_core::time_astro_units!($cb);

        #[cfg(feature = "navigation")]
        qtty_core::angular_navigation_units!($cb);
        #[cfg(feature = "navigation")]
        qtty_core::length_navigation_units!($cb);

        #[cfg(feature = "land-area")]
        qtty_core::area_land_area_units!($cb);

        #[cfg(feature = "customary")]
        qtty_core::area_customary_units!($cb);
        #[cfg(feature = "customary")]
        qtty_core::energy_customary_units!($cb);
        #[cfg(feature = "customary")]
        qtty_core::length_customary_units!($cb);
        #[cfg(feature = "customary")]
        qtty_core::mass_customary_units!($cb);
        #[cfg(feature = "customary")]
        qtty_core::power_customary_units!($cb);
        #[cfg(feature = "customary")]
        qtty_core::pressure_customary_units!($cb);
        #[cfg(feature = "customary")]
        qtty_core::volume_customary_units!($cb);

        #[cfg(feature = "fundamental-physics")]
        qtty_core::energy_fundamental_physics_units!($cb);
        #[cfg(feature = "fundamental-physics")]
        qtty_core::length_fundamental_physics_units!($cb);
        #[cfg(feature = "fundamental-physics")]
        qtty_core::mass_fundamental_physics_units!($cb);
        #[cfg(feature = "fundamental-physics")]
        qtty_core::power_fundamental_physics_units!($cb);

        #[cfg(feature = "julian-time")]
        qtty_core::time_julian_time_units!($cb);

        #[cfg(feature = "radiometry")]
        qtty_core::radiance_units!($cb);
        #[cfg(feature = "radiometry")]
        qtty_core::spectral_radiance_units!($cb);
        #[cfg(feature = "radiometry")]
        qtty_core::photon_radiance_units!($cb);
        #[cfg(feature = "radiometry")]
        qtty_core::spectral_photon_radiance_units!($cb);
        #[cfg(feature = "radiometry")]
        qtty_core::inverse_solid_angle_units!($cb);

        #[cfg(feature = "photometry")]
        qtty_core::candela_units!($cb);
        #[cfg(feature = "photometry")]
        qtty_core::lumen_units!($cb);
        #[cfg(feature = "photometry")]
        qtty_core::lux_units!($cb);

        #[cfg(feature = "frequency")]
        qtty_core::frequency_units!($cb);

        #[cfg(feature = "chemistry")]
        qtty_core::amount_units!($cb);

        #[cfg(feature = "electrical")]
        qtty_core::ampere_units!($cb);
        #[cfg(feature = "electrical")]
        qtty_core::coulomb_units!($cb);
        #[cfg(feature = "electrical")]
        qtty_core::volt_units!($cb);
        #[cfg(feature = "electrical")]
        qtty_core::ohm_units!($cb);
        #[cfg(feature = "electrical")]
        qtty_core::farad_units!($cb);
        #[cfg(feature = "electrical")]
        qtty_core::henry_units!($cb);
        #[cfg(feature = "electrical")]
        qtty_core::weber_units!($cb);
        #[cfg(feature = "electrical")]
        qtty_core::tesla_units!($cb);

        #[cfg(feature = "density")]
        qtty_core::density_units!($cb);
        #[cfg(all(feature = "density", feature = "customary"))]
        qtty_core::density_customary_units!($cb);
    };
}

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
// Dimension modules (re-exported from qtty-core)
// ─────────────────────────────────────────────────────────────────────────────

pub use qtty_core::units::acceleration;
pub use qtty_core::units::angular;
pub use qtty_core::units::area;
pub use qtty_core::units::energy;
pub use qtty_core::units::force;
pub use qtty_core::units::length;
pub use qtty_core::units::mass;
#[cfg(feature = "photometry")]
pub use qtty_core::units::photometry;
pub use qtty_core::units::power;
pub use qtty_core::units::pressure;
#[cfg(feature = "radiometry")]
pub use qtty_core::units::radiometry;
pub use qtty_core::units::solid_angle;
pub use qtty_core::units::temperature;
pub use qtty_core::units::time;
pub use qtty_core::units::volume;

#[cfg(feature = "chemistry")]
pub use qtty_core::units::amount;
#[cfg(feature = "density")]
pub use qtty_core::units::density;
#[cfg(feature = "electrical")]
pub use qtty_core::units::electrical;
#[cfg(feature = "frequency")]
pub use qtty_core::units::frequency;

// ─────────────────────────────────────────────────────────────────────────────
// Type-level unit markers
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level unit markers used by [`Quantity`].
///
/// The crate root exposes singular quantity aliases such as [`Meter`] and
/// [`Second`]. Use this module when generic code needs the unit marker itself,
/// for example `Quantity<unit::Meter, S>` or `to::<unit::Kilometer>()`.
pub mod unit {
    pub use qtty_core::{Per, Prod, Unit, UnitDiv, UnitMul, UnitSqrt};

    #[cfg(feature = "navigation")]
    pub use qtty_core::units::angular::Gradian;
    pub use qtty_core::units::angular::{AngularUnit, Degree, Milliradian, Radian, Turn};
    #[cfg(feature = "astro")]
    pub use qtty_core::units::angular::{
        Arcminute, Arcsecond, HourAngle, MicroArcsecond, MilliArcsecond,
    };

    #[cfg(feature = "land-area")]
    pub use qtty_core::units::area::{Acre, Are, Hectare};
    pub use qtty_core::units::area::{
        AreaUnit, SquareCentimeter, SquareKilometer, SquareMeter, SquareMillimeter,
    };
    #[cfg(feature = "customary")]
    pub use qtty_core::units::area::{SquareFoot, SquareInch, SquareMile, SquareYard};

    #[cfg(feature = "astro")]
    pub use qtty_core::units::length::nominal::{
        EarthEquatorialRadius, EarthPolarRadius, EarthRadius, JupiterRadius, LunarDistance,
        LunarRadius, SolarDiameter, SolarRadius,
    };
    #[cfg(feature = "astro")]
    pub use qtty_core::units::length::{
        AstronomicalUnit, Gigaparsec, Kiloparsec, LightYear, Megaparsec, Parsec,
    };
    pub use qtty_core::units::length::{
        Attometer, Centimeter, Decameter, Decimeter, Exameter, Femtometer, Gigameter, Hectometer,
        Kilometer, LengthUnit, Megameter, Meter, Micrometer, Millimeter, Nanometer, Petameter,
        Picometer, Terameter, Yoctometer, Yottameter, Zeptometer, Zettameter,
    };
    #[cfg(feature = "fundamental-physics")]
    pub use qtty_core::units::length::{
        BohrRadius, ClassicalElectronRadius, ElectronReducedComptonWavelength, PlanckLength,
    };
    #[cfg(feature = "navigation")]
    pub use qtty_core::units::length::{
        Chain, EarthEquatorialCircumference, EarthMeridionalCircumference, Fathom, Link,
        NauticalMile, Rod,
    };
    #[cfg(feature = "customary")]
    pub use qtty_core::units::length::{Foot, Inch, Mile, Yard};

    #[cfg(feature = "fundamental-physics")]
    pub use qtty_core::units::mass::AtomicMassUnit;
    #[cfg(feature = "astro")]
    pub use qtty_core::units::mass::SolarMass;
    pub use qtty_core::units::mass::{
        Attogram, Centigram, Decagram, Decigram, Exagram, Femtogram, Gigagram, Gram, Hectogram,
        Kilogram, MassUnit, Megagram, Microgram, Milligram, Nanogram, Petagram, Picogram, Teragram,
        Tonne, Yoctogram, Yottagram, Zeptogram, Zettagram,
    };
    #[cfg(feature = "customary")]
    pub use qtty_core::units::mass::{Carat, Grain, LongTon, Ounce, Pound, ShortTon, Stone};

    #[cfg(feature = "fundamental-physics")]
    pub use qtty_core::units::power::ErgPerSecond;
    #[cfg(feature = "astro")]
    pub use qtty_core::units::power::SolarLuminosity;
    pub use qtty_core::units::power::{
        Attowatt, Decawatt, Deciwatt, Exawatt, Femtowatt, Gigawatt, Hectowatt, Kilowatt, Megawatt,
        Microwatt, Milliwatt, Nanowatt, Petawatt, Picowatt, PowerUnit, Terawatt, Watt, Yoctowatt,
        Yottawatt, Zeptowatt, Zettawatt,
    };
    #[cfg(feature = "customary")]
    pub use qtty_core::units::power::{HorsepowerElectric, HorsepowerMetric};

    #[cfg(feature = "astro")]
    pub use qtty_core::units::solid_angle::{SquareArcminute, SquareArcsecond};
    pub use qtty_core::units::solid_angle::{SquareDegree, SquareMilliradian, Steradian};

    pub use qtty_core::units::acceleration::{
        AccelerationUnit, MeterPerSecondSquared, StandardGravity,
    };

    #[cfg(feature = "fundamental-physics")]
    pub use qtty_core::units::force::Dyne;
    #[cfg(feature = "customary")]
    pub use qtty_core::units::force::PoundForce;
    pub use qtty_core::units::force::{
        ForceUnit, Giganewton, Kilonewton, Meganewton, Micronewton, Millinewton, Newton,
    };

    #[cfg(feature = "customary")]
    pub use qtty_core::units::pressure::{
        Atmosphere, InchOfMercury, MillimeterOfMercury, PoundPerSquareInch, Torr,
    };
    pub use qtty_core::units::pressure::{
        Bar, Gigapascal, Hectopascal, Kilopascal, Megapascal, Millipascal, Pascal, PressureUnit,
    };

    pub use qtty_core::units::temperature::{Kelvin, Rankine, TemperatureUnit};

    #[cfg(feature = "radiometry")]
    pub use qtty_core::units::radiometry::{
        ErgPerSecondSquareCentimeterSteradian, ErgPerSecondSquareCentimeterSteradianAngstrom,
        InverseSolidAngleUnit, PhotonPerSquareCentimeterNanosecondSteradian,
        PhotonPerSquareCentimeterNanosecondSteradianNanometer,
        PhotonPerSquareCentimeterSecondSteradian, PhotonPerSquareCentimeterSecondSteradianAngstrom,
        PhotonPerSquareCentimeterSecondSteradianNanometer, PhotonPerSquareMeterSecondSteradian,
        PhotonPerSquareMeterSecondSteradianMeter, PhotonRadianceUnit, RadianceUnit,
        SpectralPhotonRadianceUnit, SpectralRadianceUnit, WattPerSquareMeterSteradian,
        WattPerSquareMeterSteradianMeter, WattPerSquareMeterSteradianNanometer, S10,
    };

    #[cfg(feature = "customary")]
    pub use qtty_core::units::energy::{BritishThermalUnit, Calorie, Kilocalorie, Therm};
    #[cfg(feature = "fundamental-physics")]
    pub use qtty_core::units::energy::{Electronvolt, Erg, Kiloelectronvolt, Megaelectronvolt};
    pub use qtty_core::units::energy::{
        EnergyUnit, Gigajoule, Joule, Kilojoule, KilowattHour, Megajoule, Microjoule, Millijoule,
        Nanojoule, Picojoule, Terajoule, WattHour,
    };

    pub use qtty_core::units::time::{
        Attosecond, Centisecond, Century, Day, Decade, Decasecond, Decisecond, Femtosecond,
        Fortnight, Gigasecond, Hectosecond, Hour, Kilosecond, Megasecond, Microsecond, Millennium,
        Millisecond, Minute, Nanosecond, Picosecond, Second, Terasecond, TimeUnit, Week, Year,
    };
    #[cfg(feature = "julian-time")]
    pub use qtty_core::units::time::{JulianCentury, JulianYear};
    #[cfg(feature = "astro")]
    pub use qtty_core::units::time::{SiderealDay, SiderealYear, SynodicMonth};

    pub use qtty_core::units::volume::{
        Centiliter, CubicCentimeter, CubicKilometer, CubicMeter, CubicMillimeter, Deciliter, Liter,
        Microliter, Milliliter, VolumeUnit,
    };
    #[cfg(feature = "customary")]
    pub use qtty_core::units::volume::{CubicFoot, CubicInch, UsFluidOunce, UsGallon};

    #[cfg(feature = "photometry")]
    pub use qtty_core::units::photometry::{
        Candela, Kilolumen, Kilolux, Lumen, Lux, Millilumen, Millilux,
    };

    #[cfg(feature = "frequency")]
    pub use qtty_core::units::frequency::{
        FrequencyUnit, Gigahertz, Hertz, Kilohertz, Megahertz, Millihertz, Terahertz,
    };

    #[cfg(feature = "chemistry")]
    pub use qtty_core::units::amount::{
        AmountUnit, Kilomole, Micromole, Millimole, Mole, Nanomole,
    };

    #[cfg(feature = "electrical")]
    pub use qtty_core::units::electrical::{
        Ampere, Coulomb, Farad, Henry, Kiloampere, Kilocoulomb, Kilohm, Kilovolt, Megaohm,
        Megavolt, Microampere, Microcoulomb, Microfarad, Microhenry, Microtesla, Microvolt,
        Milliampere, Millicoulomb, Millifarad, Millihenry, Milliohm, Millitesla, Millivolt,
        Milliweber, Nanofarad, Ohm, Picofarad, Tesla, Volt, Weber,
    };

    #[cfg(all(feature = "density", feature = "customary"))]
    pub use qtty_core::units::density::PoundPerCubicFoot;
    #[cfg(feature = "density")]
    pub use qtty_core::units::density::{
        DensityUnit, GramPerCubicCentimeter, GramPerMilliliter, KilogramPerCubicMeter,
    };
}

/// Plural alias for [`unit`]. `qtty::units::Meter` resolves to the same
/// type-level marker as `qtty::unit::Meter`, mirroring the user-facing
/// expectation that singular and plural module names both work.
pub use unit as units;

/// Velocity quantities represented as one unit divided by another.
pub mod velocity {
    pub use qtty_core::units::velocity::{Velocity, VelocityUnit};
}

/// Angular-rate quantities represented as one unit divided by another (`Angular / Time`).
///
/// This module models angular displacement per unit time (e.g. rad/s, deg/day).
/// It does **not** model SI Hertz-style inverse-time frequency (`T⁻¹`); see
/// [`qtty_core::angular_rate`] for the distinction.
pub mod angular_rate {
    pub use qtty_core::units::angular_rate::{AngularRate, AngularRateUnit};
}

/// Acceleration quantities represented as `Length / Time²`.
pub mod accel {
    pub use qtty_core::units::acceleration::{Accel, AccelerationUnit};
}

// ─────────────────────────────────────────────────────────────────────────────
// Convenience quantity aliases (default f64 scalar)
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! _root_alias {
    ($($unit:ident),+ $(,)?) => {
        $(
            #[doc = concat!(stringify!($unit), ".")]
            pub type $unit<S = f64> = Quantity<unit::$unit, S>;
        )+
    };
}
__qtty_invoke_all_inventories!(_root_alias);
__qtty_invoke_optional_inventories!(_root_alias);

pub use qtty_core::units::angular::{DEG, RAD};
#[cfg(feature = "astro")]
pub use qtty_core::units::length::{AU, LY};
pub use qtty_core::units::length::{KM, M};
pub use qtty_core::units::time::{DAY, SEC};

pub use accel::Accel;

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
