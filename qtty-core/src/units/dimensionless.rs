// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Named dimensionless units for domain-specific ratios.
//!
//! These unit markers all have the [`Dimensionless`] dimension and a ratio of
//! `1.0`, but their names preserve semantic intent in public APIs. They are
//! useful when a raw scalar would otherwise hide whether a value is an optical
//! depth, airmass, reflectance-like fraction, or refractivity.

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the dimensionless dimension from the dimension module.
pub use crate::dimension::Dimensionless;

/// Marker trait for any [`Unit`] whose dimension is [`Dimensionless`].
pub trait DimensionlessUnit: Unit<Dim = Dimensionless> {}
impl<T: Unit<Dim = Dimensionless>> DimensionlessUnit for T {}

/// Atmospheric vertical or slant optical depth `tau`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "", dimension = Dimensionless, ratio = 1.0)]
pub struct OpticalDepth;

/// A quantity measured as optical depth.
pub type OpticalDepths = Quantity<OpticalDepth>;

/// Airmass `X`, the path-length multiplier relative to zenith.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "", dimension = Dimensionless, ratio = 1.0)]
pub struct Airmass;

/// A quantity measured as airmass.
pub type Airmasses = Quantity<Airmass>;

/// Transmittance `T`, the surviving fraction of incident flux.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "", dimension = Dimensionless, ratio = 1.0)]
pub struct Transmittance;

/// A quantity measured as transmittance.
pub type Transmittances = Quantity<Transmittance>;

/// Bond or geometric albedo.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "", dimension = Dimensionless, ratio = 1.0)]
pub struct Albedo;

/// A quantity measured as albedo.
pub type Albedos = Quantity<Albedo>;

/// Illuminated fraction of a planetary or lunar disk.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "", dimension = Dimensionless, ratio = 1.0)]
pub struct IlluminationFraction;

/// A quantity measured as illuminated fraction.
pub type IlluminationFractions = Quantity<IlluminationFraction>;

/// Atmospheric refractivity `n - 1`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "", dimension = Dimensionless, ratio = 1.0)]
pub struct Refractivity;

/// A quantity measured as refractivity.
pub type Refractivities = Quantity<Refractivity>;

/// A generic dimensionless ratio (e.g. `v/c`, efficiency, scale factor).
///
/// Use this when no more specific named dimensionless unit applies.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "", dimension = Dimensionless, ratio = 1.0)]
pub struct Ratio;

/// A quantity measured as a dimensionless ratio.
pub type Ratios = Quantity<Ratio>;

/// Canonical list of named dimensionless units.
#[macro_export]
#[doc(hidden)]
macro_rules! dimensionless_units {
    ($cb:path) => {
        $cb!(
            OpticalDepth,
            Airmass,
            Transmittance,
            Albedo,
            IlluminationFraction,
            Refractivity,
            Ratio
        );
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn named_dimensionless_values_round_trip() {
        assert_eq!(OpticalDepths::new(0.5).value(), 0.5);
        assert_eq!(Airmasses::new(1.2).value(), 1.2);
        assert_eq!(Transmittances::new(0.8).value(), 0.8);
        assert_eq!(Albedos::new(0.3).value(), 0.3);
        assert_eq!(IlluminationFractions::new(0.75).value(), 0.75);
        assert_eq!(Refractivities::new(2.7e-4).value(), 2.7e-4);
    }
}
