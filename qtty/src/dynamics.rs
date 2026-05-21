// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Typed unit aliases and physical constants for spacecraft astrodynamics.
//!
//! This module provides the unit aliases and semantic coefficient newtypes that the
//! `siderust::astro::dynamics` redesign requires.  They live in `qtty` rather than
//! `siderust` because they are pure quantity/unit definitions with no observational
//! or ephemeris semantics.
//!
//! ## Unit aliases
//!
//! | Alias | Expansion | Typical use |
//! |-------|-----------|-------------|
//! | [`KmPerSecond`] | `Per<Kilometer, Second>` | Orbital velocity |
//! | [`KmPerSecondSquared`] | `Per<Per<Kilometer, Second>, Second>` | Acceleration in km/s² |
//! | [`InverseSecond`] | `Per<Ratio, Second>` | Decay/rate, inverse-time quantities |
//! | [`AreaToMassUnit`] | `Per<SquareMeter, Kilogram>` | Ballistic coefficient area/mass |
//! | [`GravitationalParameter`] | `Quantity<Per<CubicKilometer, Prod<Second, Second>>>` | μ = G·M |
//!
//! ## Gravitational parameter constants
//!
//! | Constant | Value (km³/s²) | Reference |
//! |----------|----------------|-----------|
//! | [`GM_EARTH`] | 398 600.441 8 | EGM2008 / WGS-84 |
//! | [`GM_SUN`] | 1.327 124 400 18 × 10¹¹ | IAU 2012 |
//! | [`GM_MOON`] | 4.902 800 066 × 10³ | DE430 |
//! | [`GM_MERCURY`] | 2.203 187 832 8 × 10⁴ | DE430 |
//! | [`GM_VENUS`] | 3.248 585 920 0 × 10⁵ | DE430 |
//! | [`GM_MARS`] | 4.282 837 581 6 × 10⁴ | DE430 |
//! | [`GM_JUPITER`] | 1.267 127 648 0 × 10⁸ | DE430 |
//! | [`GM_SATURN`] | 3.794 058 520 0 × 10⁷ | DE430 |
//! | [`GM_URANUS`] | 5.794 548 600 0 × 10⁶ | DE430 |
//! | [`GM_NEPTUNE`] | 6.836 527 100 5 × 10⁶ | DE430 |
//! | [`GM_PLUTO`] | 9.770 × 10² | DE430 |
//!
//! ## Semantic coefficient newtypes
//!
//! [`DragCoefficient`], [`SrpCoefficient`], [`J2Coefficient`], and
//! [`StokesCoefficient`] are thin wrappers around a bare `f64` that prevent
//! accidental mixing of aerodynamic, solar-radiation-pressure, gravitational
//! zonal, and tesseral/sectorial harmonic coefficients in force-model APIs.

use qtty_core::units::area::{SquareKilometer, SquareMeter};
use qtty_core::units::dimensionless::Ratio;
use qtty_core::units::length::Kilometer;
use qtty_core::units::mass::Kilogram;
use qtty_core::units::time::Second;
use qtty_core::units::volume::CubicKilometer;
use qtty_core::{Per, Prod, Quantity};

// ─────────────────────────────────────────────────────────────────────────────
// Unit type aliases
// ─────────────────────────────────────────────────────────────────────────────

/// Unit marker for kilometre per second (km/s).
///
/// Canonical orbital-velocity unit used throughout astrodynamics.
pub type KmPerSecond = Per<Kilometer, Second>;

/// Quantity alias: a velocity in km/s.
pub type KmPerSeconds = Quantity<KmPerSecond>;

/// Unit marker for kilometre per second squared (km/s²).
///
/// Astrodynamics acceleration unit: `(km/s) / s`.
pub type KmPerSecondSquared = Per<Per<Kilometer, Second>, Second>;

/// Quantity alias: an acceleration in km/s².
pub type KmPerSecondsSquared = Quantity<KmPerSecondSquared>;

/// Unit marker for km²/s — specific angular momentum.
pub type KmSquaredPerSecond = Per<SquareKilometer, Second>;

/// Quantity alias: specific angular momentum in km²/s.
pub type SpecificAngularMomentum = Quantity<KmSquaredPerSecond>;

/// Unit marker for km²/s² — specific orbital energy.
pub type KmSquaredPerSecondSquared = Per<SquareKilometer, Prod<Second, Second>>;

/// Quantity alias: specific orbital energy in km²/s².
pub type SpecificOrbitalEnergy = Quantity<KmSquaredPerSecondSquared>;

/// Unit marker for inverse-second (1/s).
///
/// Represents a rate or decay constant.  The numerator uses [`Ratio`] as the
/// dimensionless "one" unit, giving the dimension `Dimensionless / Time`.
pub type InverseSecond = Per<Ratio, Second>;

/// Quantity alias: a rate or decay constant in 1/s.
pub type InverseSeconds = Quantity<InverseSecond>;

/// Unit marker for area-to-mass ratio (m²/kg).
///
/// Used as the effective cross-sectional area per unit mass in drag and SRP
/// force models.
pub type AreaToMassUnit = Per<SquareMeter, Kilogram>;

/// Quantity alias: an area-to-mass ratio in m²/kg.
pub type AreaToMass = Quantity<AreaToMassUnit>;

// ─────────────────────────────────────────────────────────────────────────────
// Gravitational parameter
// ─────────────────────────────────────────────────────────────────────────────

/// Unit marker for the standard gravitational parameter μ = G·M (km³/s²).
///
/// EGM2008 uses this convention: `μ_Earth ≈ 398 600.441 8 km³/s²`.
/// The unit is `CubicKilometer / (Second * Second)`.
pub type GravitationalParameterUnit = Per<CubicKilometer, Prod<Second, Second>>;

/// Typed standard gravitational parameter: km³/s².
///
/// Used for G·M of Earth, Sun, Moon, and any other body expressed in the
/// conventional astrodynamics unit.
pub type GravitationalParameter = Quantity<GravitationalParameterUnit>;

/// Earth standard gravitational parameter (EGM2008 / WGS-84).
///
/// μ_⊕ = 398 600.441 8 km³/s²
pub const GM_EARTH: GravitationalParameter = GravitationalParameter::new(398_600.441_8);

/// Sun standard gravitational parameter (IAU 2012 system).
///
/// μ_☉ = 1.327 124 400 18 × 10¹¹ km³/s²
pub const GM_SUN: GravitationalParameter = GravitationalParameter::new(1.327_124_400_18e11);

/// Moon standard gravitational parameter (DE430 value).
///
/// μ_☾ = 4.902 800 066 × 10³ km³/s²
pub const GM_MOON: GravitationalParameter = GravitationalParameter::new(4.902_800_066e3);

/// Mercury standard gravitational parameter (DE430 value).
///
/// μ = 2.203 187 832 8 × 10⁴ km³/s²
pub const GM_MERCURY: GravitationalParameter = GravitationalParameter::new(2.203_187_832_8e4);

/// Venus standard gravitational parameter (DE430 value).
///
/// μ = 3.248 585 920 0 × 10⁵ km³/s²
pub const GM_VENUS: GravitationalParameter = GravitationalParameter::new(3.248_585_920_0e5);

/// Mars system standard gravitational parameter (DE430 value, Mars + Phobos + Deimos).
///
/// μ = 4.282 837 581 6 × 10⁴ km³/s²
pub const GM_MARS: GravitationalParameter = GravitationalParameter::new(4.282_837_581_6e4);

/// Jupiter system standard gravitational parameter (DE430 value, Jupiter + all moons).
///
/// μ = 1.267 127 648 0 × 10⁸ km³/s²
pub const GM_JUPITER: GravitationalParameter = GravitationalParameter::new(1.267_127_648_0e8);

/// Saturn system standard gravitational parameter (DE430 value, Saturn + all moons).
///
/// μ = 3.794 058 520 0 × 10⁷ km³/s²
pub const GM_SATURN: GravitationalParameter = GravitationalParameter::new(3.794_058_520_0e7);

/// Uranus system standard gravitational parameter (DE430 value, Uranus + all moons).
///
/// μ = 5.794 548 600 0 × 10⁶ km³/s²
pub const GM_URANUS: GravitationalParameter = GravitationalParameter::new(5.794_548_600_0e6);

/// Neptune system standard gravitational parameter (DE430 value, Neptune + all moons).
///
/// μ = 6.836 527 100 5 × 10⁶ km³/s²
pub const GM_NEPTUNE: GravitationalParameter = GravitationalParameter::new(6.836_527_100_5e6);

/// Pluto system standard gravitational parameter (DE430 value, Pluto + Charon).
///
/// μ = 9.770 × 10² km³/s²
pub const GM_PLUTO: GravitationalParameter = GravitationalParameter::new(9.770e2);

/// Speed of light in vacuum (km/s).
///
/// c = 299 792.458 km/s (exact by SI definition).
pub const SPEED_OF_LIGHT_KM_S: KmPerSeconds = KmPerSeconds::new(299_792.458);

// ─────────────────────────────────────────────────────────────────────────────
// Semantic dimensionless coefficient newtypes
// ─────────────────────────────────────────────────────────────────────────────

/// Aerodynamic drag coefficient C_D (dimensionless).
///
/// Physically represents the ratio of drag force to the product of dynamic
/// pressure and reference area. Typical values: 2.0–2.4 for flat plates in
/// free-molecular flow.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DragCoefficient(f64);

impl DragCoefficient {
    /// Creates a new drag coefficient from a raw dimensionless value.
    #[inline]
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the raw dimensionless value.
    #[inline]
    pub fn value(self) -> f64 {
        self.0
    }
}

/// Solar radiation pressure coefficient C_R (dimensionless).
///
/// Scales the ideal-reflector SRP force.  A perfectly absorbing surface has
/// C_R = 1.0; a perfect specular mirror has C_R = 2.0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SrpCoefficient(f64);

impl SrpCoefficient {
    /// Creates a new SRP coefficient from a raw dimensionless value.
    #[inline]
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the raw dimensionless value.
    #[inline]
    pub fn value(self) -> f64 {
        self.0
    }
}

/// Gravitational zonal harmonic J₂ coefficient (dimensionless).
///
/// The dominant oblateness term in the geopotential expansion.
/// For Earth: J₂ ≈ 1.082 635 9 × 10⁻³ (EGM2008).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct J2Coefficient(f64);

impl J2Coefficient {
    /// Creates a new J₂ coefficient from a raw dimensionless value.
    #[inline]
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the raw dimensionless value.
    #[inline]
    pub fn value(self) -> f64 {
        self.0
    }
}

/// Stokes (spherical-harmonic) geopotential coefficient C_nm or S_nm (dimensionless).
///
/// Stores a single fully-normalised Stokes coefficient as used in the EGM2008
/// and similar gravity-field models.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StokesCoefficient(f64);

impl StokesCoefficient {
    /// Creates a new Stokes coefficient from a raw dimensionless value.
    #[inline]
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the raw dimensionless value.
    #[inline]
    pub fn value(self) -> f64 {
        self.0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use qtty_core::units::length::Kilometers;
    use qtty_core::units::time::Seconds;

    // ── Gravitational parameter constants ─────────────────────────────────────

    #[test]
    fn gm_earth_value() {
        assert!((GM_EARTH.value() - 398_600.441_8).abs() < 1e-3);
    }

    #[test]
    fn gm_sun_order_of_magnitude() {
        assert!(GM_SUN.value() > 1e11 && GM_SUN.value() < 1.4e11);
    }

    #[test]
    fn gm_moon_order_of_magnitude() {
        assert!(GM_MOON.value() > 4_000.0 && GM_MOON.value() < 6_000.0);
    }

    // ── Coefficient newtypes ──────────────────────────────────────────────────

    #[test]
    fn drag_coefficient_roundtrip() {
        let cd = DragCoefficient::new(2.2);
        assert!((cd.value() - 2.2).abs() < 1e-15);
    }

    #[test]
    fn srp_coefficient_roundtrip() {
        let cr = SrpCoefficient::new(1.5);
        assert!((cr.value() - 1.5).abs() < 1e-15);
    }

    #[test]
    fn j2_coefficient_roundtrip() {
        let j2 = J2Coefficient::new(1.082_635_9e-3);
        assert!((j2.value() - 1.082_635_9e-3).abs() < 1e-20);
    }

    #[test]
    fn stokes_coefficient_roundtrip() {
        let c = StokesCoefficient::new(-0.484_165_4e-3);
        assert!((c.value() - (-0.484_165_4e-3)).abs() < 1e-20);
    }

    // ── Dimensional alias correctness ─────────────────────────────────────────

    /// KmPerSecondSquared * Second * Second should produce Kilometers.
    ///
    /// `(km/s)/s * s = km/s`, then `km/s * s = km`.
    #[test]
    fn km_per_second_squared_dimensional() {
        // 2 km/s² * 3 s = 6 km/s
        let a: Quantity<KmPerSecondSquared> = Quantity::new(2.0);
        let t = Seconds::new(3.0);
        // (km/s)/s * s → km/s  (Per<N,D> * D → N)
        let v: KmPerSeconds = a * t;
        assert!((v.value() - 6.0).abs() < 1e-12);
        // km/s * s → km
        let d: Kilometers = v * Seconds::new(1.0);
        assert!((d.value() - 6.0).abs() < 1e-12);
    }
}
