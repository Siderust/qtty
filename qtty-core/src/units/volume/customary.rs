// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Imperial / US customary volume units
// ─────────────────────────────────────────────────────────────────────────────

/// Cubic inch (`1.6387064e-5 m³`, exact: `0.0254³ m³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "in³", dimension = Volume, ratio = 1.638_706_4e-5)]
pub struct CubicInch;
/// A quantity measured in cubic inches.
pub type CubicInches = Quantity<CubicInch>;

/// Cubic foot (`0.028316846592 m³`, exact: `0.3048³ m³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ft³", dimension = Volume, ratio = 0.028_316_846_592)]
pub struct CubicFoot;
/// A quantity measured in cubic feet.
pub type CubicFeet = Quantity<CubicFoot>;

/// US liquid gallon (`0.003785411784 m³`, exact: `231 in³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "gal", dimension = Volume, ratio = 0.003_785_411_784)]
pub struct UsGallon;
/// A quantity measured in US gallons.
pub type UsGallons = Quantity<UsGallon>;

/// US fluid ounce (`2.95735295625e-5 m³`, exact: `gal / 128`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "fl oz", dimension = Volume, ratio = 2.957_352_956_25e-5)]
pub struct UsFluidOunce;
/// A quantity measured in US fluid ounces.
pub type UsFluidOunces = Quantity<UsFluidOunce>;

crate::impl_unit_from_conversions_between!(
    CubicMeter, CubicKilometer, CubicCentimeter, CubicMillimeter, Liter, Milliliter, Microliter, Centiliter, Deciliter;
    CubicInch, CubicFoot, UsGallon, UsFluidOunce
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    CubicMeter, CubicKilometer, CubicCentimeter, CubicMillimeter, Liter, Milliliter, Microliter, Centiliter, Deciliter;
    CubicInch, CubicFoot, UsGallon, UsFluidOunce
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! volume_customary_units {
    ($cb:path) => {
        $cb!(CubicInch, CubicFoot, UsGallon, UsFluidOunce,);
    };
}
