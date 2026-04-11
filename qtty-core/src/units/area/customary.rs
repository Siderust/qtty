// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Square inch (`6.4516e-4 m²`, exact: `0.0254² m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "in²", dimension = Area, ratio = 6.4516e-4)]
pub struct SquareInch;
/// A quantity measured in square inches.
pub type SquareInches = Quantity<SquareInch>;

/// Square foot (`0.09290304 m²`, exact: `0.3048² m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ft²", dimension = Area, ratio = 0.09290304)]
pub struct SquareFoot;
/// A quantity measured in square feet.
pub type SquareFeet = Quantity<SquareFoot>;

/// Square yard (`0.83612736 m²`, exact: `0.9144² m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "yd²", dimension = Area, ratio = 0.83612736)]
pub struct SquareYard;
/// A quantity measured in square yards.
pub type SquareYards = Quantity<SquareYard>;

/// Square mile (`2_589_988.110336 m²`, exact: `1609.344² m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mi²", dimension = Area, ratio = 2_589_988.110_336)]
pub struct SquareMile;
/// A quantity measured in square miles.
pub type SquareMiles = Quantity<SquareMile>;

crate::impl_unit_from_conversions_between!(
    SquareMeter, SquareKilometer, SquareCentimeter, SquareMillimeter;
    SquareInch, SquareFoot, SquareYard, SquareMile
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    SquareMeter, SquareKilometer, SquareCentimeter, SquareMillimeter;
    SquareInch, SquareFoot, SquareYard, SquareMile
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! area_customary_units {
    ($cb:path) => {
        $cb!(
            SquareInch,
            SquareFoot,
            SquareYard,
            SquareMile,
        );
    };
}
