// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Solar luminosity (IAU nominal constant; watts per L☉).
///
/// This is a *nominal reference* value intended for consistent conversion.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "L☉", dimension = Power, ratio = 3.828e26)]
pub struct SolarLuminosity;
/// A quantity measured in solar luminosities.
pub type SolarLuminosities = Quantity<SolarLuminosity>;
/// One solar luminosity.
pub const L_SUN: SolarLuminosities = SolarLuminosities::new(1.0);

crate::impl_unit_from_conversions_between!(
    Watt, Yoctowatt, Zeptowatt, Attowatt, Femtowatt, Picowatt, Nanowatt, Microwatt,
    Milliwatt, Deciwatt, Decawatt, Hectowatt, Kilowatt, Megawatt, Gigawatt, Terawatt,
    Petawatt, Exawatt, Zettawatt, Yottawatt;
    SolarLuminosity
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Watt, Yoctowatt, Zeptowatt, Attowatt, Femtowatt, Picowatt, Nanowatt, Microwatt,
    Milliwatt, Deciwatt, Decawatt, Hectowatt, Kilowatt, Megawatt, Gigawatt, Terawatt,
    Petawatt, Exawatt, Zettawatt, Yottawatt;
    SolarLuminosity
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! power_astro_units {
    ($cb:path) => {
        $cb!(
            SolarLuminosity,
        );
    };
}
