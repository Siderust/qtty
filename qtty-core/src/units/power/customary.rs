// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Metric horsepower (`PS`), defined as exactly `735.49875 W`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "PS", dimension = Power, ratio = 73_549_875.0 / 100_000.0)]
pub struct HorsepowerMetric;
/// A quantity measured in metric horsepower.
pub type HorsepowerMetrics = Quantity<HorsepowerMetric>;
/// One metric horsepower.
pub const PS: HorsepowerMetrics = HorsepowerMetrics::new(1.0);

/// Electric horsepower (`hp_e`), defined as exactly `746 W`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "hp_e", dimension = Power, ratio = 746.0)]
pub struct HorsepowerElectric;
/// A quantity measured in electric horsepower.
pub type HorsepowerElectrics = Quantity<HorsepowerElectric>;
/// One electric horsepower.
pub const HP_E: HorsepowerElectrics = HorsepowerElectrics::new(1.0);

crate::impl_unit_from_conversions_between!(
    Watt, Yoctowatt, Zeptowatt, Attowatt, Femtowatt, Picowatt, Nanowatt, Microwatt,
    Milliwatt, Deciwatt, Decawatt, Hectowatt, Kilowatt, Megawatt, Gigawatt, Terawatt,
    Petawatt, Exawatt, Zettawatt, Yottawatt;
    HorsepowerMetric, HorsepowerElectric
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Watt, Yoctowatt, Zeptowatt, Attowatt, Femtowatt, Picowatt, Nanowatt, Microwatt,
    Milliwatt, Deciwatt, Decawatt, Hectowatt, Kilowatt, Megawatt, Gigawatt, Terawatt,
    Petawatt, Exawatt, Zettawatt, Yottawatt;
    HorsepowerMetric, HorsepowerElectric
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! power_customary_units {
    ($cb:path) => {
        $cb!(
            HorsepowerMetric,
            HorsepowerElectric,
        );
    };
}
