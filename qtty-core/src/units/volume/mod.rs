// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Volume units.
//!
//! The canonical scaling unit for this dimension is the **cubic metre** (`CubicMeter::RATIO == 1.0`).
//! All other volume units are expressed as exact ratios to cubic metres.
//!
//! This module provides:
//!
//! - **Metric cubes**: cubic millimetre, cubic centimetre, cubic metre, cubic kilometre.
//! - **Litre family**: microlitre, millilitre, centilitre, decilitre, litre.
//! - **Imperial/US**: cubic inch, cubic foot, US gallon, US fluid ounce.
//!
//! Volume units can also arise *automatically* from multiplying length × area quantities:
//!
//! ```rust
//! use qtty_core::length::Meters;
//! use qtty_core::area::SquareMeters;
//! use qtty_core::volume::CubicMeters;
//!
//! let side = Meters::new(3.0);
//! let face: SquareMeters = side * side;              // direct product
//! let vol: CubicMeters = (face * side).to();
//! assert!((vol.value() - 27.0).abs() < 1e-12);
//! ```
//!
//! ## All volume units (default)
//!
//! ```rust
//! use qtty_core::volume::*;
//!
//! macro_rules! touch {
//!     ($T:ty, $v:expr) => {{ let q = <$T>::new($v); let _c = q; assert!(q == q); }};
//! }
//!
//! touch!(CubicMeters, 1.0);    touch!(CubicKilometers, 1.0);
//! touch!(CubicCentimeters, 1.0); touch!(CubicMillimeters, 1.0);
//! touch!(Liters, 1.0);         touch!(Milliliters, 1.0);
//! touch!(Microliters, 1.0);    touch!(Centiliters, 1.0);
//! touch!(Deciliters, 1.0);
//! ```

use crate::{Prod, Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the volume dimension from the dimension module.
pub use crate::dimension::Volume;

/// Marker trait for any [`Unit`] whose dimension is [`Volume`].
pub trait VolumeUnit: Unit<Dim = Volume> {}
impl<T: Unit<Dim = Volume>> VolumeUnit for T {}

/// A composed volume quantity from cubing a length unit.
///
/// `CubeOf<L>` is `Quantity<Prod<Prod<L, L>, L>>` — the type produced when
/// multiplying a [`SquareOf`](super::area::SquareOf) quantity by a further
/// length quantity. Since metric area unit types are `Prod` aliases and are
/// registered as [`BuiltinUnit`](crate::unit_arithmetic::BuiltinUnit), the
/// intermediate multiplication just works.
///
/// # Examples
///
/// ```rust
/// use qtty_core::volume::{CubeOf, CubicMeter, CubicMeters};
/// use qtty_core::area::SquareMeters;
/// use qtty_core::length::Meters;
///
/// let side = Meters::new(3.0);
/// let face: SquareMeters = side * side;
/// let vol: CubeOf<_> = face * side;                 // Quantity<Prod<Prod<Meter, Meter>, Meter>>
/// let named: CubicMeters = vol.to();
/// assert!((named.value() - 27.0).abs() < 1e-12);
/// ```
pub type CubeOf<L> = Quantity<Prod<Prod<L, L>, L>>;

#[cfg(feature = "customary")]
mod customary;
#[cfg(feature = "customary")]
pub use customary::*;

// ─────────────────────────────────────────────────────────────────────────────
// SI / metric volume units
// ─────────────────────────────────────────────────────────────────────────────

/// Cubic metre (SI derived unit of volume).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "m³", dimension = Volume, ratio = 1.0)]
pub struct CubicMeter;
/// A quantity measured in cubic metres.
pub type CubicMeters = Quantity<CubicMeter>;

/// Cubic kilometre (`1e9 m³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "km³", dimension = Volume, ratio = 1e9)]
pub struct CubicKilometer;
/// A quantity measured in cubic kilometres.
pub type CubicKilometers = Quantity<CubicKilometer>;

/// Cubic centimetre (`1e-6 m³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cm³", dimension = Volume, ratio = 1e-6)]
pub struct CubicCentimeter;
/// A quantity measured in cubic centimetres.
pub type CubicCentimeters = Quantity<CubicCentimeter>;

/// Cubic millimetre (`1e-9 m³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mm³", dimension = Volume, ratio = 1e-9)]
pub struct CubicMillimeter;
/// A quantity measured in cubic millimetres.
pub type CubicMillimeters = Quantity<CubicMillimeter>;

// ─────────────────────────────────────────────────────────────────────────────
// Litre family
// ─────────────────────────────────────────────────────────────────────────────

/// Litre (`1e-3 m³`, exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "L", dimension = Volume, ratio = 1e-3)]
pub struct Liter;
/// A quantity measured in litres.
pub type Liters = Quantity<Liter>;

/// Millilitre (`1e-6 m³`, exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mL", dimension = Volume, ratio = 1e-6)]
pub struct Milliliter;
/// A quantity measured in millilitres.
pub type Milliliters = Quantity<Milliliter>;

/// Microlitre (`1e-9 m³`, exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "µL", dimension = Volume, ratio = 1e-9)]
pub struct Microliter;
/// A quantity measured in microlitres.
pub type Microliters = Quantity<Microliter>;

/// Centilitre (`1e-5 m³`, exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cL", dimension = Volume, ratio = 1e-5)]
pub struct Centiliter;
/// A quantity measured in centilitres.
pub type Centiliters = Quantity<Centiliter>;

/// Decilitre (`1e-4 m³`, exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "dL", dimension = Volume, ratio = 1e-4)]
pub struct Deciliter;
/// A quantity measured in decilitres.
pub type Deciliters = Quantity<Deciliter>;

/// Canonical list of all volume units.
///
/// Pass a macro identifier as the single argument; it will be invoked with all
/// volume unit types as its token list. Drives:
/// - `impl_unit_from_conversions!` — bidirectional `From` impls between all pairs.
/// - `impl_unit_cross_unit_ops!` — cross-unit `PartialEq`/`PartialOrd` (feature-gated).
/// - `assert_units_are_builtin!` — compile-time check that every unit is in
///   `register_builtin_units!` (under `#[cfg(test)]`).
///
/// The macro is exported (`#[doc(hidden)]`) so the `qtty` facade can use it
/// in compile-time consistency checks (`inventory_consistency.rs`).
///
/// ```rust,ignore
/// volume_units!(crate::impl_unit_from_conversions);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! volume_units {
    ($cb:path) => {
        $cb!(
            CubicMeter,
            CubicKilometer,
            CubicCentimeter,
            CubicMillimeter,
            Liter,
            Milliliter,
            Microliter,
            Centiliter,
            Deciliter
        );
    };
}

// Generate all bidirectional From implementations between volume units.
volume_units!(crate::impl_unit_from_conversions);

// Optional cross-unit operator support (`==`, `<`, etc.).
#[cfg(feature = "cross-unit-ops")]
volume_units!(crate::impl_unit_cross_unit_ops);

// Compile-time check: every unit in the inventory is registered as BuiltinUnit.
#[cfg(test)]
volume_units!(crate::assert_units_are_builtin);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn liter_to_cubic_meter() {
        let l = Liters::new(1.0);
        let m: CubicMeters = l.to();
        assert_abs_diff_eq!(m.value(), 0.001, epsilon = 1e-15);
    }

    #[test]
    fn milliliter_to_liter() {
        let ml = Milliliters::new(1000.0);
        let l: Liters = ml.to();
        assert_abs_diff_eq!(l.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn cubic_cm_to_ml() {
        let cc = CubicCentimeters::new(1.0);
        let ml: Milliliters = cc.to();
        assert_abs_diff_eq!(ml.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn us_gallon_to_liter() {
        let g = UsGallons::new(1.0);
        let l: Liters = g.to();
        assert_abs_diff_eq!(l.value(), 3.785_411_784, epsilon = 1e-6);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn cubic_foot_to_liter() {
        let cf = CubicFeet::new(1.0);
        let l: Liters = cf.to();
        assert_abs_diff_eq!(l.value(), 28.316_846_592, epsilon = 1e-6);
    }

    #[test]
    fn length_times_area_to_volume() {
        use crate::area::{SquareMeter, SquareMeters};
        use crate::length::{Meter, Meters};
        use crate::Prod;

        let side = Meters::new(3.0);
        let face: SquareMeters = (side * side).to();
        let vol_prod: Quantity<Prod<SquareMeter, Meter>> = face * side;
        let vol: CubicMeters = vol_prod.to();
        assert_abs_diff_eq!(vol.value(), 27.0, epsilon = 1e-12);
    }

    #[test]
    fn cubic_km_to_liter() {
        let ckm = CubicKilometers::new(1.0);
        let l: Liters = ckm.to();
        assert_abs_diff_eq!(l.value(), 1e12, epsilon = 1e3);
    }

    #[test]
    fn cubic_mm_to_cubic_cm() {
        let mm3 = CubicMillimeters::new(1000.0);
        let cm3: CubicCentimeters = mm3.to();
        assert_abs_diff_eq!(cm3.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn microliter_to_milliliter() {
        let ul = Microliters::new(1000.0);
        let ml: Milliliters = ul.to();
        assert_abs_diff_eq!(ml.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn centiliter_to_liter() {
        let cl = Centiliters::new(100.0);
        let l: Liters = cl.to();
        assert_abs_diff_eq!(l.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn deciliter_to_liter() {
        let dl = Deciliters::new(10.0);
        let l: Liters = dl.to();
        assert_abs_diff_eq!(l.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn cubic_inch_to_cubic_cm() {
        let cin = CubicInches::new(1.0);
        let cc: CubicCentimeters = cin.to();
        // 1 in³ = 16.387064 cm³
        assert_abs_diff_eq!(cc.value(), 16.387_064, epsilon = 1e-4);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn us_fluid_ounce_to_milliliter() {
        let floz = UsFluidOunces::new(1.0);
        let ml: Milliliters = floz.to();
        // 1 US fl oz ≈ 29.5735 mL
        assert_abs_diff_eq!(ml.value(), 29.573_529_562_5, epsilon = 1e-6);
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(CubicMeter::SYMBOL, "m³");
        assert_eq!(Liter::SYMBOL, "L");
        assert_eq!(Milliliter::SYMBOL, "mL");
        #[cfg(feature = "customary")]
        assert_eq!(UsGallon::SYMBOL, "gal");
    }
}
