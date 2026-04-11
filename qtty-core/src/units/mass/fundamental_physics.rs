// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Unified atomic mass unit (u), a.k.a. dalton (Da).
///
/// Stored in grams using the CODATA recommended value for `m_u` in kilograms, converted by `1 kg = 1000 g`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "u", dimension = Mass, ratio = 1.660_539_068_92e-24)]
pub struct AtomicMassUnit;
/// Type alias shorthand for [`AtomicMassUnit`].
pub type Dalton = AtomicMassUnit;
/// Quantity measured in atomic mass units.
pub type AtomicMassUnits = Quantity<AtomicMassUnit>;
/// One atomic mass unit.
pub const U: AtomicMassUnits = AtomicMassUnits::new(1.0);

crate::impl_unit_from_conversions_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    AtomicMassUnit
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    AtomicMassUnit
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! mass_fundamental_physics_units {
    ($cb:path) => {
        $cb!(
            AtomicMassUnit,
        );
    };
}
