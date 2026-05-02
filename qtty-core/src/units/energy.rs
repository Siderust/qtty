// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Energy units.
//!
//! The canonical scaling unit for this dimension is the **joule** (`Joule::RATIO == 1.0`).
//! All other energy units are expressed as exact ratios to joules.
//!
//! This module provides:
//!
//! - **SI joule** and commonly used SI prefixes.
//! - **Erg** (feature: `fundamental-physics`) — CGS unit (1 erg = 10⁻⁷ J).
//! - **Electronvolt** (feature: `fundamental-physics`) — 1 eV ≈ 1.602 176 634 × 10⁻¹⁹ J (exact, 2019 SI).
//! - **Calorie / kilocalorie** (feature: `customary`) — thermochemical calorie.
//!
//! ```rust
//! use qtty_core::energy::{Kilojoules, Joule};
//!
//! let kj = Kilojoules::new(1.0);
//! let j = kj.to::<Joule>();
//! assert_eq!(j.value(), 1000.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the energy dimension from the dimension module.
pub use crate::dimension::Energy;

/// Marker trait for any [`Unit`] whose dimension is [`Energy`].
pub trait EnergyUnit: Unit<Dim = Energy> {}
impl<T: Unit<Dim = Energy>> EnergyUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// SI joule
// ─────────────────────────────────────────────────────────────────────────────

/// Joule — SI coherent derived unit of energy (kg·m²/s²).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "J", dimension = Energy, ratio = 1.0)]
pub struct Joule;
/// A quantity measured in joules.
pub type Joules = Quantity<Joule>;
/// One joule.
pub const JOULE: Joules = Joules::new(1.0);

macro_rules! si_joule {
    ($name:ident, $sym:literal, $ratio:expr, $qty:ident, $one:ident) => {
        #[doc = concat!("SI-prefixed joule unit (", stringify!($ratio), " J).")]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
        #[unit(symbol = $sym, dimension = Energy, ratio = $ratio)]
        pub struct $name;
        #[doc = concat!("A quantity measured in ", stringify!($name), "s.")]
        pub type $qty = Quantity<$name>;
        #[doc = concat!("One ", stringify!($name), ".")]
        pub const $one: $qty = $qty::new(1.0);
    };
}

si_joule!(Nanojoule, "nJ", 1e-9, Nanojoules, NANOJOULE);
si_joule!(Picojoule, "pJ", 1e-12, Picojoules, PICOJOULE);
si_joule!(Microjoule, "µJ", 1e-6, Microjoules, MICROJOULE);
si_joule!(Millijoule, "mJ", 1e-3, Millijoules, MILLIJOULE);
si_joule!(Kilojoule, "kJ", 1e3, Kilojoules, KILOJOULE);
si_joule!(Megajoule, "MJ", 1e6, Megajoules, MEGAJOULE);
si_joule!(Gigajoule, "GJ", 1e9, Gigajoules, GIGAJOULE);
si_joule!(Terajoule, "TJ", 1e12, Terajoules, TERAJOULE);

/// Watt-hour — 1 Wh = 3 600 J (exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Wh", dimension = Energy, ratio = 3_600.0)]
pub struct WattHour;
/// A quantity measured in watt-hours.
pub type WattHours = Quantity<WattHour>;
/// One watt-hour.
pub const WATT_HOUR: WattHours = WattHours::new(1.0);

/// Kilowatt-hour — 1 kWh = 3 600 000 J (exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kWh", dimension = Energy, ratio = 3_600_000.0)]
pub struct KilowattHour;
/// A quantity measured in kilowatt-hours.
pub type KilowattHours = Quantity<KilowattHour>;
/// One kilowatt-hour.
pub const KILOWATT_HOUR: KilowattHours = KilowattHours::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Feature-gated units
// ─────────────────────────────────────────────────────────────────────────────

/// Erg — CGS unit of energy (1 erg = 10⁻⁷ J).
#[cfg(feature = "fundamental-physics")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "erg", dimension = Energy, ratio = 1e-7)]
pub struct Erg;
/// A quantity measured in ergs.
#[cfg(feature = "fundamental-physics")]
pub type Ergs = Quantity<Erg>;

/// Electronvolt — 1 eV = 1.602 176 634 × 10⁻¹⁹ J (exact, 2019 SI redefinition).
#[cfg(feature = "fundamental-physics")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "eV", dimension = Energy, ratio = 1.602_176_634e-19)]
pub struct Electronvolt;
/// A quantity measured in electronvolts.
#[cfg(feature = "fundamental-physics")]
pub type Electronvolts = Quantity<Electronvolt>;

/// Kilo-electronvolt (1 keV = 10³ eV).
#[cfg(feature = "fundamental-physics")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "keV", dimension = Energy, ratio = 1.602_176_634e-16)]
pub struct Kiloelectronvolt;
/// A quantity measured in kilo-electronvolts.
#[cfg(feature = "fundamental-physics")]
pub type Kiloelectronvolts = Quantity<Kiloelectronvolt>;

/// Mega-electronvolt (1 MeV = 10⁶ eV).
#[cfg(feature = "fundamental-physics")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "MeV", dimension = Energy, ratio = 1.602_176_634e-13)]
pub struct Megaelectronvolt;
/// A quantity measured in mega-electronvolts.
#[cfg(feature = "fundamental-physics")]
pub type Megaelectronvolts = Quantity<Megaelectronvolt>;

/// Thermochemical calorie (1 cal_th = 4.184 J, exact).
#[cfg(feature = "customary")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cal", dimension = Energy, ratio = 4.184)]
pub struct Calorie;
/// A quantity measured in (thermochemical) calories.
#[cfg(feature = "customary")]
pub type Calories = Quantity<Calorie>;

/// Kilocalorie (1 kcal = 4184 J, exact).
#[cfg(feature = "customary")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kcal", dimension = Energy, ratio = 4184.0)]
pub struct Kilocalorie;
/// A quantity measured in kilocalories.
#[cfg(feature = "customary")]
pub type Kilocalories = Quantity<Kilocalorie>;

/// British Thermal Unit — 1 BTU ≈ 1 055.05585262 J (ISO 31-4).
#[cfg(feature = "customary")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "BTU", dimension = Energy, ratio = 1_055.05585262)]
pub struct BritishThermalUnit;
/// A quantity measured in British thermal units.
#[cfg(feature = "customary")]
pub type BritishThermalUnits = Quantity<BritishThermalUnit>;

/// Therm — 1 therm = 100 000 BTU = 105 505 585.262 J.
#[cfg(feature = "customary")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "therm", dimension = Energy, ratio = 105_505_585.262)]
pub struct Therm;
/// A quantity measured in therms.
#[cfg(feature = "customary")]
pub type Therms = Quantity<Therm>;

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macro
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of always-available (metric SI) energy units.
#[macro_export]
#[doc(hidden)]
macro_rules! energy_units {
    ($cb:path) => {
        $cb!(
            Joule,
            Picojoule,
            Nanojoule,
            Microjoule,
            Millijoule,
            Kilojoule,
            Megajoule,
            Gigajoule,
            Terajoule,
            WattHour,
            KilowattHour
        );
    };
}

// Generate bidirectional From impls between base metric SI energy units.
energy_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
energy_units!(crate::impl_unit_cross_unit_ops);

// ── Cross-feature: customary × fundamental-physics ───────────────────────────
#[cfg(all(feature = "customary", feature = "fundamental-physics"))]
crate::__impl_from_each_extra_to_bases!(
    {Calorie, Kilocalorie, BritishThermalUnit, Therm}
    Erg, Electronvolt, Kiloelectronvolt, Megaelectronvolt
);
#[cfg(all(
    feature = "customary",
    feature = "fundamental-physics",
    feature = "cross-unit-ops"
))]
crate::__impl_cross_ops_each_extra_to_bases!(
    {Calorie, Kilocalorie, BritishThermalUnit, Therm}
    Erg, Electronvolt, Kiloelectronvolt, Megaelectronvolt
);

// Compile-time check: every base energy unit is registered as BuiltinUnit.
#[cfg(test)]
energy_units!(crate::assert_units_are_builtin);

/// Canonical list of `fundamental-physics`-gated energy units (Erg, eV family).
#[cfg(feature = "fundamental-physics")]
#[macro_export]
#[doc(hidden)]
macro_rules! energy_fundamental_physics_units {
    ($cb:path) => {
        $cb!(Erg, Electronvolt, Kiloelectronvolt, Megaelectronvolt);
    };
}

/// Canonical list of `customary`-gated energy units (calorie, kilocalorie, BTU, therm).
#[cfg(feature = "customary")]
#[macro_export]
#[doc(hidden)]
macro_rules! energy_customary_units {
    ($cb:path) => {
        $cb!(Calorie, Kilocalorie, BritishThermalUnit, Therm);
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn kilojoule_to_joule() {
        let kj = Kilojoules::new(1.0);
        let j: Joules = kj.to();
        assert_abs_diff_eq!(j.value(), 1_000.0, epsilon = 1e-12);
    }

    #[test]
    fn joule_to_millijoule() {
        let j = Joules::new(1.0);
        let mj: Millijoules = j.to();
        assert_abs_diff_eq!(mj.value(), 1_000.0, epsilon = 1e-12);
    }

    #[test]
    fn megajoule_to_kilojoule() {
        let mj = Megajoules::new(1.0);
        let kj: Kilojoules = mj.to();
        assert_abs_diff_eq!(kj.value(), 1_000.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "fundamental-physics")]
    fn joule_to_erg() {
        let j = Joules::new(1.0);
        let e: Ergs = j.to();
        assert_abs_diff_eq!(e.value(), 1e7, epsilon = 1e-5);
    }

    #[test]
    #[cfg(feature = "fundamental-physics")]
    fn ev_to_joule() {
        let ev = Electronvolts::new(1.0);
        let j: Joules = ev.to();
        assert_abs_diff_eq!(j.value(), 1.602_176_634e-19, epsilon = 1e-30);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn calorie_to_joule() {
        let cal = Calories::new(1.0);
        let j: Joules = cal.to();
        assert_abs_diff_eq!(j.value(), 4.184, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn kilocalorie_to_joule() {
        let kcal = Kilocalories::new(1.0);
        let j: Joules = kcal.to();
        assert_abs_diff_eq!(j.value(), 4184.0, epsilon = 1e-9);
    }

    #[test]
    fn watt_hour_to_joule() {
        let wh = WattHours::new(1.0);
        let j: Joules = wh.to();
        assert_abs_diff_eq!(j.value(), 3_600.0, epsilon = 1e-10);
    }

    #[test]
    fn kilowatt_hour_to_joule() {
        let kwh = KilowattHours::new(1.0);
        let j: Joules = kwh.to();
        assert_abs_diff_eq!(j.value(), 3_600_000.0, epsilon = 1e-6);
    }

    #[test]
    fn nanojoule_to_picojoule() {
        let nj = Nanojoules::new(1.0);
        let pj: Picojoules = nj.to();
        assert_abs_diff_eq!(pj.value(), 1_000.0, epsilon = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn btu_to_joule() {
        let btu = BritishThermalUnits::new(1.0);
        let j: Joules = btu.to();
        assert_abs_diff_eq!(j.value(), 1_055.05585262, epsilon = 1e-6);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn therm_to_btu() {
        let therm = Therms::new(1.0);
        let btu: BritishThermalUnits = therm.to();
        assert_abs_diff_eq!(btu.value(), 100_000.0, epsilon = 1e-3);
    }
}
