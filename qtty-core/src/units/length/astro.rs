// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use core::f64::consts::PI;
use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Astronomical distance units
// ─────────────────────────────────────────────────────────────────────────────

/// Astronomical unit (au). Exact (IAU 2012): metres per au.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "au", dimension = Length, ratio = 149_597_870_700.0)]
pub struct AstronomicalUnit;
/// Type alias shorthand for [`AstronomicalUnit`].
pub type Au = AstronomicalUnit;
/// A quantity measured in astronomical units.
pub type AstronomicalUnits = Quantity<Au>;
/// One astronomical unit.
pub const AU: AstronomicalUnits = AstronomicalUnits::new(1.0);

// Exact speed of light and Julian year, used to derive the light‑year ratio.
const SPEED_OF_LIGHT_M_PER_S: f64 = 299_792_458.0;
const SECONDS_PER_DAY: f64 = 86_400.0;
const DAYS_PER_JULIAN_YEAR: f64 = 36525.0 / 100.0; // 365.25 d
const SECONDS_PER_JULIAN_YEAR: f64 = SECONDS_PER_DAY * DAYS_PER_JULIAN_YEAR;
const METERS_PER_LIGHT_YEAR: f64 = SPEED_OF_LIGHT_M_PER_S * SECONDS_PER_JULIAN_YEAR;

/// Light-year (ly): distance light travels in one Julian year (`365.25 d`) at `c = 299_792_458 m/s`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ly", dimension = Length, ratio = METERS_PER_LIGHT_YEAR)]
pub struct LightYear;
/// Type alias shorthand for [`LightYear`].
pub type Ly = LightYear;
/// A quantity measured in light-years.
pub type LightYears = Quantity<Ly>;
/// One light-year.
pub const LY: LightYears = LightYears::new(1.0);

/// Parsec (pc): `pc = au * 648000 / π` (exact given au).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "pc", dimension = Length, ratio = 149_597_870_700.0 * (648_000.0 / PI))]
pub struct Parsec;
/// Type alias shorthand for [`Parsec`].
pub type Pc = Parsec;
/// A quantity measured in parsecs.
pub type Parsecs = Quantity<Pc>;
/// One parsec.
pub const PC: Parsecs = Parsecs::new(1.0);

/// Kiloparsec (kpc): `1e3 pc`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kpc", dimension = Length, ratio = 1_000.0 * 149_597_870_700.0 * (648_000.0 / PI))]
pub struct Kiloparsec;
/// A quantity measured in kiloparsecs.
pub type Kiloparsecs = Quantity<Kiloparsec>;
/// One kiloparsec.
pub const KPC: Kiloparsecs = Kiloparsecs::new(1.0);

/// Megaparsec (Mpc): `1e6 pc`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Mpc", dimension = Length, ratio = 1_000_000.0 * 149_597_870_700.0 * (648_000.0 / PI))]
pub struct Megaparsec;
/// A quantity measured in megaparsecs.
pub type Megaparsecs = Quantity<Megaparsec>;
/// One megaparsec.
pub const MPC: Megaparsecs = Megaparsecs::new(1.0);

/// Gigaparsec (Gpc): `1e9 pc`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Gpc", dimension = Length, ratio = 1_000_000_000.0 * 149_597_870_700.0 * (648_000.0 / PI))]
pub struct Gigaparsec;
/// A quantity measured in gigaparsecs.
pub type Gigaparsecs = Quantity<Gigaparsec>;
/// One gigaparsec.
pub const GPC: Gigaparsecs = Gigaparsecs::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Nominal radii and distances
// ─────────────────────────────────────────────────────────────────────────────

/// Nominal astronomical and planetary radii and related distances.
///
/// Values in this module are **nominal** (conventionally rounded) and are kept separate from the
/// main length namespace to avoid confusion with strictly defined units.
pub mod nominal {
    use super::*;

    /// Solar radius (R☉). Nominal value: metres per R☉.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rsun", dimension = Length, ratio = 695_700_000.0)]
    pub struct SolarRadius;
    /// A quantity measured in solar radii.
    pub type SolarRadiuses = Quantity<SolarRadius>;
    /// One solar radius.
    pub const RSUN: SolarRadiuses = SolarRadiuses::new(1.0);

    /// Earth mean radius (nominal).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rearth", dimension = Length, ratio = 6_371_000.0)]
    pub struct EarthRadius;
    /// A quantity measured in Earth radii.
    pub type EarthRadii = Quantity<EarthRadius>;
    /// One Earth radius (mean).
    pub const R_EARTH: EarthRadii = EarthRadii::new(1.0);

    /// Earth equatorial radius (WGS84).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rearth_eq", dimension = Length, ratio = 6_378_137.0)]
    pub struct EarthEquatorialRadius;
    /// A quantity measured in Earth equatorial radii.
    pub type EarthEquatorialRadii = Quantity<EarthEquatorialRadius>;
    /// One Earth equatorial radius.
    pub const R_EARTH_EQ: EarthEquatorialRadii = EarthEquatorialRadii::new(1.0);

    /// Earth polar radius.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rearth_p", dimension = Length, ratio = 6_356_752.314_2)]
    pub struct EarthPolarRadius;
    /// A quantity measured in Earth polar radii.
    pub type EarthPolarRadii = Quantity<EarthPolarRadius>;
    /// One Earth polar radius.
    pub const R_EARTH_P: EarthPolarRadii = EarthPolarRadii::new(1.0);

    /// Lunar radius (mean, nominal).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rmoon", dimension = Length, ratio = 1_737_400.0)]
    pub struct LunarRadius;
    /// A quantity measured in lunar radii.
    pub type LunarRadii = Quantity<LunarRadius>;
    /// One lunar radius.
    pub const R_MOON: LunarRadii = LunarRadii::new(1.0);

    /// Jupiter equatorial radius (nominal).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Rjup", dimension = Length, ratio = 71_492_000.0)]
    pub struct JupiterRadius;
    /// A quantity measured in Jupiter radii.
    pub type JupiterRadii = Quantity<JupiterRadius>;
    /// One Jupiter radius.
    pub const R_JUPITER: JupiterRadii = JupiterRadii::new(1.0);

    /// Lunar distance (Earth–Moon mean distance, LD).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "LD", dimension = Length, ratio = 384_400_000.0)]
    pub struct LunarDistance;
    /// A quantity measured in lunar distances.
    pub type LunarDistances = Quantity<LunarDistance>;
    /// One lunar distance.
    pub const LD: LunarDistances = LunarDistances::new(1.0);

    /// Solar diameter (twice the solar radius).
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
    #[unit(symbol = "Dsun", dimension = Length, ratio = 2.0 * 695_700_000.0)]
    pub struct SolarDiameter;
    /// A quantity measured in solar diameters.
    pub type SolarDiameters = Quantity<SolarDiameter>;
    /// One solar diameter.
    pub const D_SUN: SolarDiameters = SolarDiameters::new(1.0);

    // ── nominal ↔ base metric conversions ────────────────────────────────────
    // Wire all nominal units against the full base metric set so they get the
    // same `From` and cross-unit comparison surface as "real" astro units.
    crate::impl_unit_from_conversions_between!(
        Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
        Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
        Terameter, Petameter, Exameter, Zettameter, Yottameter;
        SolarRadius, SolarDiameter, EarthRadius, EarthEquatorialRadius, EarthPolarRadius,
        LunarRadius, JupiterRadius, LunarDistance
    );

    #[cfg(feature = "cross-unit-ops")]
    crate::impl_unit_cross_unit_ops_between!(
        Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
        Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
        Terameter, Petameter, Exameter, Zettameter, Yottameter;
        SolarRadius, SolarDiameter, EarthRadius, EarthEquatorialRadius, EarthPolarRadius,
        LunarRadius, JupiterRadius, LunarDistance
    );
}

// ── astro ────────────────────────────────────────────────────────────────
crate::impl_unit_from_conversions_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    AstronomicalUnit, LightYear, Parsec, Kiloparsec, Megaparsec, Gigaparsec
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    AstronomicalUnit, LightYear, Parsec, Kiloparsec, Megaparsec, Gigaparsec
);

// ── nominal ↔ astro conversions ──────────────────────────────────────────
crate::__impl_from_each_extra_to_bases!(
    {AstronomicalUnit, LightYear, Parsec, Kiloparsec, Megaparsec, Gigaparsec}
    nominal::SolarRadius, nominal::SolarDiameter, nominal::EarthRadius,
    nominal::EarthEquatorialRadius, nominal::EarthPolarRadius,
    nominal::JupiterRadius, nominal::LunarRadius, nominal::LunarDistance
);

#[cfg(feature = "cross-unit-ops")]
crate::__impl_cross_ops_each_extra_to_bases!(
    {AstronomicalUnit, LightYear, Parsec, Kiloparsec, Megaparsec, Gigaparsec}
    nominal::SolarRadius, nominal::SolarDiameter, nominal::EarthRadius,
    nominal::EarthEquatorialRadius, nominal::EarthPolarRadius,
    nominal::JupiterRadius, nominal::LunarRadius, nominal::LunarDistance
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro for nominal units (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of always-available nominal length units (IAU / WGS84 radii and distances).
///
/// Exported (`#[doc(hidden)]`) for use in `qtty-ffi`'s build.rs discriminant generation.
#[macro_export]
#[doc(hidden)]
macro_rules! length_nominal_units {
    ($cb:path) => {
        $cb!(
            SolarRadius,
            SolarDiameter,
            EarthRadius,
            EarthEquatorialRadius,
            EarthPolarRadius,
            LunarRadius,
            JupiterRadius,
            LunarDistance
        );
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! length_astro_units {
    ($cb:path) => {
        $cb!(
            AstronomicalUnit,
            LightYear,
            Parsec,
            Kiloparsec,
            Megaparsec,
            Gigaparsec,
        );
    };
}
