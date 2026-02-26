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
//! use qtty_core::length::{Meter, Meters};
//! use qtty_core::area::{SquareMeter, SquareMeters};
//! use qtty_core::volume::{CubicMeters, CubicMeter};
//! use qtty_core::Prod;
//!
//! let side = Meters::new(3.0);
//! let face: SquareMeters = (side * side).to();
//! let vol_prod = face * side;                       // Quantity<Prod<SquareMeter, Meter>>
//! let vol: CubicMeters = vol_prod.to();
//! assert!((vol.value() - 27.0).abs() < 1e-12);
//! ```
//!
//! ## All volume units
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
//! touch!(Deciliters, 1.0);     touch!(CubicInches, 1.0);
//! touch!(CubicFeet, 1.0);      touch!(UsGallons, 1.0);
//! touch!(UsFluidOunces, 1.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the volume dimension from the dimension module.
pub use crate::dimension::Volume;

/// Marker trait for any [`Unit`] whose dimension is [`Volume`].
pub trait VolumeUnit: Unit<Dim = Volume> {}
impl<T: Unit<Dim = Volume>> VolumeUnit for T {}

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

// ─────────────────────────────────────────────────────────────────────────────
// Imperial / US customary volume units
// ─────────────────────────────────────────────────────────────────────────────

/// Cubic inch (`1.6387064e-5 m³`, exact: `0.0254³ m³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "in³", dimension = Volume, ratio = 1.638_706_4e-5)]
pub struct CubicInch;
/// A quantity measured in cubic inches.
pub type CubicInches = Quantity<CubicInch>;

/// Cubic foot (`0.028316846592 m³`, exact: `0.3048³ m³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ft³", dimension = Volume, ratio = 0.028_316_846_592)]
pub struct CubicFoot;
/// A quantity measured in cubic feet.
pub type CubicFeet = Quantity<CubicFoot>;

/// US liquid gallon (`0.003785411784 m³`, exact: `231 in³`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "gal", dimension = Volume, ratio = 0.003_785_411_784)]
pub struct UsGallon;
/// A quantity measured in US gallons.
pub type UsGallons = Quantity<UsGallon>;

/// US fluid ounce (`2.95735295625e-5 m³`, exact: `gal / 128`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "fl oz", dimension = Volume, ratio = 2.957_352_956_25e-5)]
pub struct UsFluidOunce;
/// A quantity measured in US fluid ounces.
pub type UsFluidOunces = Quantity<UsFluidOunce>;

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
    fn us_gallon_to_liter() {
        let g = UsGallons::new(1.0);
        let l: Liters = g.to();
        assert_abs_diff_eq!(l.value(), 3.785_411_784, epsilon = 1e-6);
    }

    #[test]
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
    fn cubic_inch_to_cubic_cm() {
        let cin = CubicInches::new(1.0);
        let cc: CubicCentimeters = cin.to();
        // 1 in³ = 16.387064 cm³
        assert_abs_diff_eq!(cc.value(), 16.387_064, epsilon = 1e-4);
    }

    #[test]
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
        assert_eq!(UsGallon::SYMBOL, "gal");
    }
}
