// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Area units.
//!
//! The canonical scaling unit for this dimension is the **square metre** (`SquareMeter::RATIO == 1.0`).
//! All other area units are expressed as exact ratios to square metres.
//!
//! This module provides:
//!
//! - **Metric squares**: square millimetre, square centimetre, square metre, square kilometre.
//! - **Land measurement**: hectare, are.
//! - **Imperial/US**: square inch, square foot, square yard, square mile, acre.
//!
//! Area units can also arise *automatically* from multiplying two length quantities:
//!
//! ```rust
//! use qtty_core::length::{Meter, Meters};
//! use qtty_core::area::{SquareMeters, SquareMeter};
//! use qtty_core::Prod;
//!
//! let side = Meters::new(5.0);
//! let area_prod = side * side;                     // Quantity<Prod<Meter, Meter>>
//! let area: SquareMeters = area_prod.to();          // Convert to named area unit
//! assert!((area.value() - 25.0).abs() < 1e-12);
//! ```
//!
//! ## All area units (default)
//!
//! ```rust
//! use qtty_core::area::*;
//!
//! macro_rules! touch {
//!     ($T:ty, $v:expr) => {{ let q = <$T>::new($v); let _c = q; assert!(q == q); }};
//! }
//!
//! touch!(SquareMeters, 1.0);     touch!(SquareKilometers, 1.0);
//! touch!(SquareCentimeters, 1.0);touch!(SquareMillimeters, 1.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the area dimension from the dimension module.
pub use crate::dimension::Area;

/// Marker trait for any [`Unit`] whose dimension is [`Area`].
pub trait AreaUnit: Unit<Dim = Area> {}
impl<T: Unit<Dim = Area>> AreaUnit for T {}

#[cfg(feature = "land-area")]
mod land_area;
#[cfg(feature = "land-area")]
pub use land_area::*;
#[cfg(feature = "customary")]
mod customary;
#[cfg(feature = "customary")]
pub use customary::*;

// ─────────────────────────────────────────────────────────────────────────────
// SI / metric area units
// ─────────────────────────────────────────────────────────────────────────────

/// Square metre (SI derived unit of area).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "m²", dimension = Area, ratio = 1.0)]
pub struct SquareMeter;
/// A quantity measured in square metres.
pub type SquareMeters = Quantity<SquareMeter>;

/// Square kilometre (`1e6 m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "km²", dimension = Area, ratio = 1e6)]
pub struct SquareKilometer;
/// A quantity measured in square kilometres.
pub type SquareKilometers = Quantity<SquareKilometer>;

/// Square centimetre (`1e-4 m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cm²", dimension = Area, ratio = 1e-4)]
pub struct SquareCentimeter;
/// A quantity measured in square centimetres.
pub type SquareCentimeters = Quantity<SquareCentimeter>;

/// Square millimetre (`1e-6 m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mm²", dimension = Area, ratio = 1e-6)]
pub struct SquareMillimeter;
/// A quantity measured in square millimetres.
pub type SquareMillimeters = Quantity<SquareMillimeter>;

// ─────────────────────────────────────────────────────────────────────────────
// Imperial / US customary area units
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// From conversions: default (metric) units
// ─────────────────────────────────────────────────────────────────────────────
crate::impl_unit_from_conversions!(
    SquareMeter,
    SquareKilometer,
    SquareCentimeter,
    SquareMillimeter
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops!(
    SquareMeter,
    SquareKilometer,
    SquareCentimeter,
    SquareMillimeter
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn sqm_to_sqkm() {
        let a = SquareMeters::new(1_000_000.0);
        let b: SquareKilometers = a.to();
        assert_abs_diff_eq!(b.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "land-area")]
    fn hectare_to_sqm() {
        let a = Hectares::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 10_000.0, epsilon = 1e-9);
    }

    #[test]
    #[cfg(feature = "land-area")]
    fn acre_to_hectare() {
        let a = Acres::new(1.0);
        let b: Hectares = a.to();
        assert_abs_diff_eq!(b.value(), 0.404_685_642_24, epsilon = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn sqft_to_sqm() {
        let a = SquareFeet::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 0.092_903_04, epsilon = 1e-9);
    }

    #[test]
    fn length_product_to_area() {
        use crate::length::{Meter, Meters};
        use crate::Prod;

        let side = Meters::new(5.0);
        let area_prod: Quantity<Prod<Meter, Meter>> = side * side;
        let area: SquareMeters = area_prod.to();
        assert_abs_diff_eq!(area.value(), 25.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn sqmile_to_sqkm() {
        let a = SquareMiles::new(1.0);
        let b: SquareKilometers = a.to();
        assert_abs_diff_eq!(b.value(), 2.589_988_110_336, epsilon = 1e-6);
    }

    #[test]
    fn sqcm_to_sqm() {
        let a = SquareCentimeters::new(10_000.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn sqmm_to_sqcm() {
        let a = SquareMillimeters::new(100.0);
        let b: SquareCentimeters = a.to();
        assert_abs_diff_eq!(b.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "land-area")]
    fn are_to_sqm() {
        let a = Ares::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 100.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn sqinch_to_sqcm() {
        let a = SquareInches::new(1.0);
        let b: SquareCentimeters = a.to();
        // 1 in² = 6.4516 cm²
        assert_abs_diff_eq!(b.value(), 6.4516, epsilon = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn sqyard_to_sqm() {
        let a = SquareYards::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 0.836_127_36, epsilon = 1e-9);
    }

    #[test]
    fn roundtrip_sqcm_sqm() {
        let original = SquareCentimeters::new(250.0);
        let converted = original.to::<SquareMeter>();
        let back = converted.to::<SquareCentimeter>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-10);
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(SquareMeter::SYMBOL, "m²");
        #[cfg(feature = "land-area")]
        assert_eq!(Hectare::SYMBOL, "ha");
        #[cfg(feature = "land-area")]
        assert_eq!(Acre::SYMBOL, "ac");
        #[cfg(feature = "customary")]
        assert_eq!(SquareInch::SYMBOL, "in²");
    }
}
