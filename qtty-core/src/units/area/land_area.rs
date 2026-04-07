// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Land measurement
// ─────────────────────────────────────────────────────────────────────────────

/// Hectare (`10 000 m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ha", dimension = Area, ratio = 1e4)]
pub struct Hectare;
/// A quantity measured in hectares.
pub type Hectares = Quantity<Hectare>;

/// Are (`100 m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "a", dimension = Area, ratio = 100.0)]
pub struct Are;
/// A quantity measured in ares.
pub type Ares = Quantity<Are>;

/// Acre (exactly `4046.8564224 m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ac", dimension = Area, ratio = 4_046.856_422_4)]
pub struct Acre;
/// A quantity measured in acres.
pub type Acres = Quantity<Acre>;

crate::impl_unit_from_conversions_between!(
    SquareMeter, SquareKilometer, SquareCentimeter, SquareMillimeter;
    Hectare, Are, Acre
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    SquareMeter, SquareKilometer, SquareCentimeter, SquareMillimeter;
    Hectare, Are, Acre
);
