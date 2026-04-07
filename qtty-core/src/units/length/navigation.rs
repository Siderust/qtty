// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Nautical mile (`1852 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "nmi", dimension = Length, ratio = 1_852.0)]
pub struct NauticalMile;
/// A quantity measured in nautical miles.
pub type NauticalMiles = Quantity<NauticalMile>;
/// One nautical mile.
pub const NMI: NauticalMiles = NauticalMiles::new(1.0);

/// Chain (`66 ft` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ch", dimension = Length, ratio = 66.0 * (3048.0 / 10_000.0))]
pub struct Chain;
/// A quantity measured in chains.
pub type Chains = Quantity<Chain>;
/// One chain.
pub const CHAIN: Chains = Chains::new(1.0);

/// Rod / pole / perch (`16.5 ft` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "rd", dimension = Length, ratio = 16.5 * (3048.0 / 10_000.0))]
pub struct Rod;
/// A quantity measured in rods/poles/perches.
pub type Rods = Quantity<Rod>;
/// One rod.
pub const ROD: Rods = Rods::new(1.0);

/// Link (`1/100 of a chain`, i.e. `0.66 ft`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lk", dimension = Length, ratio = (66.0 / 100.0) * (3048.0 / 10_000.0))]
pub struct Link;
/// A quantity measured in links.
pub type Links = Quantity<Link>;
/// One link.
pub const LINK: Links = Links::new(1.0);

/// Fathom (`6 ft` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ftm", dimension = Length, ratio = 6.0 * (3048.0 / 10_000.0))]
pub struct Fathom;
/// A quantity measured in fathoms.
pub type Fathoms = Quantity<Fathom>;
/// One fathom.
pub const FTM: Fathoms = Fathoms::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Geodesy and navigation
// ─────────────────────────────────────────────────────────────────────────────

/// Earth meridional circumference (approximate mean value).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Cmer", dimension = Length, ratio = 40_007_863.0)]
pub struct EarthMeridionalCircumference;
/// A quantity measured in Earth meridional circumferences.
pub type EarthMeridionalCircumferences = Quantity<EarthMeridionalCircumference>;
/// One Earth meridional circumference.
pub const C_MERIDIONAL: EarthMeridionalCircumferences = EarthMeridionalCircumferences::new(1.0);

/// Earth equatorial circumference.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Ceq", dimension = Length, ratio = 40_075_017.0)]
pub struct EarthEquatorialCircumference;
/// A quantity measured in Earth equatorial circumferences.
pub type EarthEquatorialCircumferences = Quantity<EarthEquatorialCircumference>;
/// One Earth equatorial circumference.
pub const C_EQUATORIAL: EarthEquatorialCircumferences = EarthEquatorialCircumferences::new(1.0);

// ── navigation ───────────────────────────────────────────────────────────
crate::impl_unit_from_conversions_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    NauticalMile, Chain, Rod, Link, Fathom, EarthMeridionalCircumference, EarthEquatorialCircumference
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    NauticalMile, Chain, Rod, Link, Fathom, EarthMeridionalCircumference, EarthEquatorialCircumference
);
