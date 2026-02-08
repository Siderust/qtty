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

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the area dimension from the dimension module.
pub use crate::dimension::Area;

/// Marker trait for any [`Unit`] whose dimension is [`Area`].
pub trait AreaUnit: Unit<Dim = Area> {}
impl<T: Unit<Dim = Area>> AreaUnit for T {}

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

// ─────────────────────────────────────────────────────────────────────────────
// Imperial / US customary area units
// ─────────────────────────────────────────────────────────────────────────────

/// Square inch (`6.4516e-4 m²`, exact: `0.0254² m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "in²", dimension = Area, ratio = 6.4516e-4)]
pub struct SquareInch;
/// A quantity measured in square inches.
pub type SquareInches = Quantity<SquareInch>;

/// Square foot (`0.09290304 m²`, exact: `0.3048² m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ft²", dimension = Area, ratio = 0.09290304)]
pub struct SquareFoot;
/// A quantity measured in square feet.
pub type SquareFeet = Quantity<SquareFoot>;

/// Square yard (`0.83612736 m²`, exact: `0.9144² m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "yd²", dimension = Area, ratio = 0.83612736)]
pub struct SquareYard;
/// A quantity measured in square yards.
pub type SquareYards = Quantity<SquareYard>;

/// Square mile (`2_589_988.110336 m²`, exact: `1609.344² m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mi²", dimension = Area, ratio = 2_589_988.110_336)]
pub struct SquareMile;
/// A quantity measured in square miles.
pub type SquareMiles = Quantity<SquareMile>;

/// Acre (exactly `4046.8564224 m²`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ac", dimension = Area, ratio = 4_046.856_422_4)]
pub struct Acre;
/// A quantity measured in acres.
pub type Acres = Quantity<Acre>;

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
    fn hectare_to_sqm() {
        let a = Hectares::new(1.0);
        let b: SquareMeters = a.to();
        assert_abs_diff_eq!(b.value(), 10_000.0, epsilon = 1e-9);
    }

    #[test]
    fn acre_to_hectare() {
        let a = Acres::new(1.0);
        let b: Hectares = a.to();
        assert_abs_diff_eq!(b.value(), 0.404_685_642_24, epsilon = 1e-9);
    }

    #[test]
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
    fn sqmile_to_sqkm() {
        let a = SquareMiles::new(1.0);
        let b: SquareKilometers = a.to();
        assert_abs_diff_eq!(b.value(), 2.589_988_110_336, epsilon = 1e-6);
    }
}
