// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Mass-density units (feature: `density`).
//!
//! The canonical scaling unit for this dimension is the
//! **kilogram per cubic metre** (`KilogramPerCubicMeter::RATIO == 1.0`).
//! All other density units are expressed as exact ratios to kg·m⁻³.
//!
//! ```rust
//! use qtty_core::density::{KilogramsPerCubicMeter, GramPerCubicCentimeter};
//!
//! let gcc = KilogramsPerCubicMeter::new(1_000.0);
//! let d = gcc.to::<GramPerCubicCentimeter>();
//! assert!((d.value() - 1.0).abs() < 1e-12);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the density dimension from the dimension module.
pub use crate::dimension::Density;

/// Marker trait for any [`Unit`] whose dimension is [`Density`].
pub trait DensityUnit: Unit<Dim = Density> {}
impl<T: Unit<Dim = Density>> DensityUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// SI density units
// ─────────────────────────────────────────────────────────────────────────────

/// Kilogram per cubic metre — SI coherent derived unit of mass density (kg·m⁻³).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kg/m³", dimension = Density, ratio = 1.0)]
pub struct KilogramPerCubicMeter;
/// Type alias shorthand for [`KilogramPerCubicMeter`].
pub type KgM3 = KilogramPerCubicMeter;
/// A quantity measured in kilograms per cubic metre.
pub type KilogramsPerCubicMeter = Quantity<KgM3>;
/// One kilogram per cubic metre.
pub const KILOGRAM_PER_CUBIC_METER: KilogramsPerCubicMeter = KilogramsPerCubicMeter::new(1.0);

/// Gram per cubic centimetre — 1 g/cm³ = 1 000 kg/m³ (exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "g/cm³", dimension = Density, ratio = 1_000.0)]
pub struct GramPerCubicCentimeter;
/// Type alias shorthand for [`GramPerCubicCentimeter`].
pub type GCm3 = GramPerCubicCentimeter;
/// A quantity measured in grams per cubic centimetre.
pub type GramsPerCubicCentimeter = Quantity<GCm3>;
/// One gram per cubic centimetre.
pub const GRAM_PER_CUBIC_CENTIMETER: GramsPerCubicCentimeter = GramsPerCubicCentimeter::new(1.0);

/// Gram per millilitre — numerically identical to g/cm³ (1 g/mL = 1 000 kg/m³).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "g/mL", dimension = Density, ratio = 1_000.0)]
pub struct GramPerMilliliter;
/// Type alias shorthand for [`GramPerMilliliter`].
pub type GmL = GramPerMilliliter;
/// A quantity measured in grams per millilitre.
pub type GramsPerMilliliter = Quantity<GmL>;
/// One gram per millilitre.
pub const GRAM_PER_MILLILITER: GramsPerMilliliter = GramsPerMilliliter::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Customary density units
// ─────────────────────────────────────────────────────────────────────────────

/// Pound per cubic foot — 1 lb/ft³ ≈ 16.018 463 373 kg/m³.
#[cfg(feature = "customary")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lb/ft³", dimension = Density, ratio = 16.018_463_373)]
pub struct PoundPerCubicFoot;
/// Type alias shorthand for [`PoundPerCubicFoot`].
#[cfg(feature = "customary")]
pub type LbFt3 = PoundPerCubicFoot;
/// A quantity measured in pounds per cubic foot.
#[cfg(feature = "customary")]
pub type PoundsPerCubicFoot = Quantity<LbFt3>;
/// One pound per cubic foot.
#[cfg(feature = "customary")]
pub const POUND_PER_CUBIC_FOOT: PoundsPerCubicFoot = PoundsPerCubicFoot::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macros
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of always-available density units.
#[macro_export]
#[doc(hidden)]
macro_rules! density_units {
    ($cb:path) => {
        $cb!(
            KilogramPerCubicMeter,
            GramPerCubicCentimeter,
            GramPerMilliliter
        );
    };
}

/// Canonical list of `customary`-gated density units.
#[cfg(feature = "customary")]
#[macro_export]
#[doc(hidden)]
macro_rules! density_customary_units {
    ($cb:path) => {
        $cb!(PoundPerCubicFoot);
    };
}

density_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
density_units!(crate::impl_unit_cross_unit_ops);

// Cross-feature: customary density conversions (SI ↔ customary and intra-customary).
// impl_unit_from_conversions_between! handles intra-extra pairs too, so we do NOT
// call density_customary_units! separately.
#[cfg(feature = "customary")]
crate::impl_unit_from_conversions_between!(
    KilogramPerCubicMeter, GramPerCubicCentimeter, GramPerMilliliter;
    PoundPerCubicFoot
);

#[cfg(all(feature = "customary", feature = "cross-unit-ops"))]
crate::impl_unit_cross_unit_ops_between!(
    KilogramPerCubicMeter, GramPerCubicCentimeter, GramPerMilliliter;
    PoundPerCubicFoot
);

#[cfg(test)]
density_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn gcc_to_kgm3() {
        let gcc = GramsPerCubicCentimeter::new(1.0);
        let kgm3: KilogramsPerCubicMeter = gcc.to();
        assert_abs_diff_eq!(kgm3.value(), 1_000.0, epsilon = 1e-9);
    }

    #[test]
    fn gml_equals_gcc() {
        let gml = GramsPerMilliliter::new(2.5);
        let gcc: GramsPerCubicCentimeter = gml.to();
        assert_abs_diff_eq!(gcc.value(), 2.5, epsilon = 1e-12);
    }

    #[test]
    fn water_density_at_4c() {
        // Water at 4 °C ≈ 1 000 kg/m³ = 1 g/cm³
        let kgm3 = KilogramsPerCubicMeter::new(1_000.0);
        let gcc: GramsPerCubicCentimeter = kgm3.to();
        assert_abs_diff_eq!(gcc.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn lbft3_to_kgm3() {
        let lbft3 = PoundsPerCubicFoot::new(1.0);
        let kgm3: KilogramsPerCubicMeter = lbft3.to();
        assert_abs_diff_eq!(kgm3.value(), 16.018_463_373, epsilon = 1e-6);
    }
}
