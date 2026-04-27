// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Pressure units.
//!
//! The canonical scaling unit for this dimension is the **pascal** (`Pascal::RATIO == 1.0`).
//! All other pressure units are expressed as exact ratios to pascals.
//!
//! Pressure has dimension M¹ · L⁻¹ · T⁻² (equivalently, force per unit area: N/m²).
//!
//! This module provides:
//!
//! - **SI pascal** and the commonly used SI-prefixed variants.
//! - **Bar** — non-SI unit accepted for use with SI (1 bar = 100 000 Pa exactly).
//!
//! ```rust
//! use qtty_core::pressure::{Pascals, Hectopascal};
//!
//! let p = Pascals::new(101_325.0);
//! let hpa = p.to::<Hectopascal>();
//! assert!((hpa.value() - 1013.25).abs() < 1e-9);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the pressure dimension from the dimension module.
pub use crate::dimension::Pressure;

/// Marker trait for any [`Unit`] whose dimension is [`Pressure`].
pub trait PressureUnit: Unit<Dim = Pressure> {}
impl<T: Unit<Dim = Pressure>> PressureUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// SI pascal
// ─────────────────────────────────────────────────────────────────────────────

/// Pascal — SI coherent derived unit of pressure (kg·m⁻¹·s⁻², equivalently N/m²).
///
/// BIPM SI brochure 9th ed., Table 4: 1 Pa = 1 N m⁻¹ = 1 kg m⁻¹ s⁻².
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Pa", dimension = Pressure, ratio = 1.0)]
pub struct Pascal;
/// Type alias shorthand for [`Pascal`].
pub type Pa = Pascal;
/// A quantity measured in pascals.
pub type Pascals = Quantity<Pa>;
/// One pascal.
pub const PASCAL: Pascals = Pascals::new(1.0);

macro_rules! si_pascal {
    ($name:ident, $sym:literal, $ratio:expr, $alias:ident, $qty:ident, $one:ident) => {
        #[doc = concat!("SI-prefixed pascal unit (", stringify!($ratio), " Pa).")]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
        #[unit(symbol = $sym, dimension = Pressure, ratio = $ratio)]
        pub struct $name;
        #[doc = concat!("Type alias shorthand for [`", stringify!($name), "`].")]
        pub type $alias = $name;
        #[doc = concat!("A quantity measured in ", stringify!($name), "s.")]
        pub type $qty = Quantity<$alias>;
        #[doc = concat!("One ", stringify!($name), ".")]
        pub const $one: $qty = $qty::new(1.0);
    };
}

/// Hectopascal — SI-prefixed pascal unit (100 Pa).
///
/// 1 hPa = 100 Pa exactly. Widely used in meteorology and observatory/site
/// metadata to report atmospheric pressure (standard atmosphere ≈ 1013.25 hPa).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "hPa", dimension = Pressure, ratio = 1e2)]
pub struct Hectopascal;
/// Type alias shorthand for [`Hectopascal`].
pub type HPa = Hectopascal;
/// A quantity measured in hectopascals.
pub type Hectopascals = Quantity<HPa>;
/// One hectopascal.
pub const HECTOPASCAL: Hectopascals = Hectopascals::new(1.0);

si_pascal!(Millipascal, "mPa", 1e-3, MPa, Millipascals, MILLIPASCAL);
si_pascal!(Kilopascal,  "kPa", 1e3,  KPa, Kilopascals,  KILOPASCAL);
si_pascal!(Megapascal,  "MPa", 1e6,  MpaPa, Megapascals,  MEGAPASCAL);
si_pascal!(Gigapascal,  "GPa", 1e9,  GPa, Gigapascals,  GIGAPASCAL);

// ─────────────────────────────────────────────────────────────────────────────
// Non-SI but common: bar
// ─────────────────────────────────────────────────────────────────────────────

/// Bar — non-SI unit of pressure accepted for use with the SI (1 bar = 10⁵ Pa exactly).
///
/// BIPM SI brochure 9th ed., Table 8. Widely used in engineering, oceanography,
/// and observatory metadata. The millibar (mbar) is numerically identical to the
/// hectopascal (hPa).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "bar", dimension = Pressure, ratio = 1e5)]
pub struct Bar;
/// A quantity measured in bars.
pub type Bars = Quantity<Bar>;
/// One bar.
pub const BAR: Bars = Bars::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macro
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of always-available pressure units.
#[macro_export]
#[doc(hidden)]
macro_rules! pressure_units {
    ($cb:path) => {
        $cb!(
            Pascal,
            Millipascal,
            Hectopascal,
            Kilopascal,
            Megapascal,
            Gigapascal,
            Bar
        );
    };
}

// Generate bidirectional From impls between all pressure units.
pressure_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
pressure_units!(crate::impl_unit_cross_unit_ops);

// Compile-time check: every pressure unit is registered as BuiltinUnit.
#[cfg(test)]
pressure_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn pascal_to_hectopascal() {
        let p = Pascals::new(101_325.0);
        let hpa: Hectopascals = p.to();
        assert_abs_diff_eq!(hpa.value(), 1013.25, epsilon = 1e-9);
    }

    #[test]
    fn hectopascal_to_pascal() {
        let hpa = Hectopascals::new(1013.25);
        let p: Pascals = hpa.to();
        assert_abs_diff_eq!(p.value(), 101_325.0, epsilon = 1e-9);
    }

    #[test]
    fn pascal_to_kilopascal() {
        let p = Pascals::new(1000.0);
        let kpa: Kilopascals = p.to();
        assert_abs_diff_eq!(kpa.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn pascal_to_bar() {
        let p = Pascals::new(100_000.0);
        let bar: Bars = p.to();
        assert_abs_diff_eq!(bar.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn bar_to_hectopascal() {
        let bar = Bars::new(1.0);
        let hpa: Hectopascals = bar.to();
        assert_abs_diff_eq!(hpa.value(), 1000.0, epsilon = 1e-9);
    }

    #[test]
    fn pascals_addition() {
        let a = Pascals::new(50.0);
        let b = Pascals::new(50.0);
        let c = a + b;
        assert_abs_diff_eq!(c.value(), 100.0, epsilon = 1e-12);
    }
}
