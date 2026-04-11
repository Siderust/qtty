// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Fundamental physics lengths (CODATA values)
// ─────────────────────────────────────────────────────────────────────────────

/// Bohr radius (`a0`). CODATA 2018 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "a0", dimension = Length, ratio = 5.291_772_109_03e-11)]
pub struct BohrRadius;
/// A quantity measured in Bohr radii.
pub type BohrRadii = Quantity<BohrRadius>;
/// One Bohr radius.
pub const A0: BohrRadii = BohrRadii::new(1.0);

/// Classical electron radius (`re`). CODATA 2018 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "re", dimension = Length, ratio = 2.817_940_326_2e-15)]
pub struct ClassicalElectronRadius;
/// A quantity measured in classical electron radii.
pub type ClassicalElectronRadii = Quantity<ClassicalElectronRadius>;
/// One classical electron radius.
pub const RE: ClassicalElectronRadii = ClassicalElectronRadii::new(1.0);

/// Planck length (`lp`). CODATA 2018 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lp", dimension = Length, ratio = 1.616_255e-35)]
pub struct PlanckLength;
/// A quantity measured in Planck lengths.
pub type PlanckLengths = Quantity<PlanckLength>;
/// One Planck length.
pub const LP: PlanckLengths = PlanckLengths::new(1.0);

/// Reduced Compton wavelength of the electron (`lambda_bar_e`). CODATA 2018 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lambda_bar_e", dimension = Length, ratio = 3.861_592_679_6e-13)]
pub struct ElectronReducedComptonWavelength;
/// A quantity measured in reduced Compton wavelengths of the electron.
pub type ElectronReducedComptonWavelengths = Quantity<ElectronReducedComptonWavelength>;
/// One reduced Compton wavelength of the electron.
pub const LAMBDA_BAR_E: ElectronReducedComptonWavelengths =
    ElectronReducedComptonWavelengths::new(1.0);

// ── fundamental-physics ──────────────────────────────────────────────────
crate::impl_unit_from_conversions_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    BohrRadius, ClassicalElectronRadius, PlanckLength, ElectronReducedComptonWavelength
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    BohrRadius, ClassicalElectronRadius, PlanckLength, ElectronReducedComptonWavelength
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! length_fundamental_physics_units {
    ($cb:path) => {
        $cb!(
            PlanckLength,
            BohrRadius,
            ClassicalElectronRadius,
            ElectronReducedComptonWavelength,
        );
    };
}
