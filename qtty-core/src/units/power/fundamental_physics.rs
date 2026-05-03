// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Erg per second (`erg/s`).
///
/// Exact: `1 erg = 1e-7 J`, so `1 erg/s = 1e-7 W`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "erg/s", dimension = Power, ratio = 1e-7)]
pub struct ErgPerSecond;
/// One erg/s.
pub const ERG_PER_S: Quantity<ErgPerSecond> = Quantity::new(1.0);

crate::impl_unit_from_conversions_between!(
    Watt, Yoctowatt, Zeptowatt, Attowatt, Femtowatt, Picowatt, Nanowatt, Microwatt,
    Milliwatt, Deciwatt, Decawatt, Hectowatt, Kilowatt, Megawatt, Gigawatt, Terawatt,
    Petawatt, Exawatt, Zettawatt, Yottawatt;
    ErgPerSecond
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Watt, Yoctowatt, Zeptowatt, Attowatt, Femtowatt, Picowatt, Nanowatt, Microwatt,
    Milliwatt, Deciwatt, Decawatt, Hectowatt, Kilowatt, Megawatt, Gigawatt, Terawatt,
    Petawatt, Exawatt, Zettawatt, Yottawatt;
    ErgPerSecond
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! power_fundamental_physics_units {
    ($cb:path) => {
        $cb!(ErgPerSecond,);
    };
}
