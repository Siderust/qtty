// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Nominal solar mass (IAU 2015 Resolution B3; grams per M☉).
///
/// This is a **conversion constant** (nominal), not a “best estimate” of the Sun’s true mass.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "M☉", dimension = Mass, ratio = 1.988_416e33)]
pub struct SolarMass;
/// A quantity measured in solar masses.
pub type SolarMasses = Quantity<SolarMass>;
/// One nominal solar mass.
pub const MSUN: SolarMasses = SolarMasses::new(1.0);

crate::impl_unit_from_conversions_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    SolarMass
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    SolarMass
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! mass_astro_units {
    ($cb:path) => {
        $cb!(
            SolarMass,
        );
    };
}
