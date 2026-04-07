// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Dimensionless helpers.
//!
//! This module contains small adapters for working with dimensionless values.
//!
//! # Warning: unit erasure preserves the raw stored number
//!
//! The provided conversion from a dimensioned quantity to a unitless quantity is *lossy*: it drops
//! the unit type **without performing any normalization to the canonical (SI) unit**. The numeric
//! value is preserved as-is, which means the result depends on which unit the source quantity was
//! stored in.
//!
//! ```rust
//! use qtty_core::time::Seconds;
//! use qtty_core::length::{Meters, Kilometers};
//! use qtty_core::{Quantity, Unitless};
//!
//! // Canonical unit: value matches SI.
//! let t = Seconds::new(3.0);
//! let u: Quantity<Unitless> = t.into();
//! assert_eq!(u.value(), 3.0);
//!
//! // Non-canonical unit: value is NOT converted to metres first!
//! let km = Kilometers::new(1.0);
//! let u: Quantity<Unitless> = km.into();
//! assert_eq!(u.value(), 1.0); // NOT 1000.0
//! ```
//!
//! If you need the value in canonical (SI base) units before erasing, convert first:
//!
//! ```rust
//! use qtty_core::length::{Meter, Kilometers};
//! use qtty_core::{Quantity, Unitless};
//!
//! let km = Kilometers::new(1.0);
//! let m = km.to::<Meter>();
//! let u: Quantity<Unitless> = m.into();
//! assert_eq!(u.value(), 1000.0);
//! ```

use crate::dimension::{
    Acceleration, AmountOfSubstance, Angular, Area, Current, Energy, Force, FrequencyDim, Length,
    LuminousIntensity, Mass, Power, Temperature, Time, VelocityDim, Volume,
};
use crate::scalar::Scalar;
use crate::{Quantity, Unit, Unitless};

trait SupportedDimension {}
impl SupportedDimension for Length {}
impl SupportedDimension for Time {}
impl SupportedDimension for Mass {}
impl SupportedDimension for Temperature {}
impl SupportedDimension for Current {}
impl SupportedDimension for AmountOfSubstance {}
impl SupportedDimension for LuminousIntensity {}
impl SupportedDimension for Angular {}
impl SupportedDimension for Area {}
impl SupportedDimension for Volume {}
impl SupportedDimension for VelocityDim {}
impl SupportedDimension for Acceleration {}
impl SupportedDimension for Force {}
impl SupportedDimension for Energy {}
impl SupportedDimension for Power {}
impl SupportedDimension for FrequencyDim {}

trait DimensionedUnit: Unit {}
impl<U: Unit> DimensionedUnit for U where U::Dim: SupportedDimension {}

impl<U: DimensionedUnit, S: Scalar> From<Quantity<U, S>> for Quantity<Unitless, S> {
    #[inline]
    fn from(quantity: Quantity<U, S>) -> Self {
        Self::new(quantity.value())
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::units::angular::Degrees;
    use crate::units::length::Meters;
    use crate::units::mass::Kilogram;
    use crate::units::mass::Kilograms;
    use crate::units::time::Seconds;
    use crate::Unit;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic Unitless behavior
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unitless_new_and_value() {
        let u: Quantity<Unitless> = Quantity::new(42.0);
        assert_eq!(u.value(), 42.0);
    }

    #[test]
    fn unitless_from_f64() {
        let u: Quantity<Unitless> = 1.23456.into();
        assert_abs_diff_eq!(u.value(), 1.23456, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display formatting
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_unitless() {
        let u: Quantity<Unitless> = Quantity::new(123.456);
        let s = format!("{}", u);
        assert_eq!(s, "123.456");
    }

    #[test]
    fn display_unitless_integer() {
        let u: Quantity<Unitless> = Quantity::new(42.0);
        let s = format!("{}", u);
        assert_eq!(s, "42");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Conversion from dimensioned quantities
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn from_length() {
        let m = Meters::new(42.0);
        let u: Quantity<Unitless> = m.into();
        assert_eq!(u.value(), 42.0);
    }

    #[test]
    fn from_non_canonical_unit_preserves_raw_value() {
        use crate::units::length::Kilometers;
        // Kilometers::new(1.0) → unitless should give 1.0, NOT 1000.0.
        // This verifies the documented behavior: no SI normalization on erasure.
        let km = Kilometers::new(1.0);
        let u: Quantity<Unitless> = km.into();
        assert_eq!(u.value(), 1.0);
    }

    #[test]
    fn from_time() {
        let t = Seconds::new(5.0);
        let u: Quantity<Unitless> = t.into();
        assert_eq!(u.value(), 5.0);
    }

    #[test]
    fn from_mass() {
        let m = Kilograms::new(2.5);
        let u: Quantity<Unitless> = m.into();
        assert_eq!(u.value(), 2.5);
    }

    #[test]
    fn from_angular() {
        let a = Degrees::new(90.0);
        let u: Quantity<Unitless> = a.into();
        assert_eq!(u.value(), 90.0);
    }

    #[test]
    fn from_mass_preserves_non_default_scalar_type() {
        let m: Quantity<Kilogram, i32> = Quantity::new(7);
        let u: Quantity<Unitless, i32> = m.into();
        assert_eq!(u.value(), 7);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Arithmetic operations
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unitless_addition() {
        let a: Quantity<Unitless> = Quantity::new(3.0);
        let b: Quantity<Unitless> = Quantity::new(4.0);
        assert_eq!((a + b).value(), 7.0);
    }

    #[test]
    fn unitless_subtraction() {
        let a: Quantity<Unitless> = Quantity::new(10.0);
        let b: Quantity<Unitless> = Quantity::new(3.0);
        assert_eq!((a - b).value(), 7.0);
    }

    #[test]
    fn unitless_multiplication() {
        let a: Quantity<Unitless> = Quantity::new(3.0);
        assert_eq!((a * 4.0).value(), 12.0);
    }

    #[test]
    fn unitless_division() {
        let a: Quantity<Unitless> = Quantity::new(12.0);
        assert_eq!((a / 4.0).value(), 3.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Unit trait implementation
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unitless_ratio() {
        assert_eq!(Unitless::RATIO, 1.0);
    }

    #[test]
    fn unitless_symbol() {
        assert_eq!(Unitless::SYMBOL, "");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_unitless_arithmetic(a in -1e6..1e6f64, b in -1e6..1e6f64) {
            let qa: Quantity<Unitless> = Quantity::new(a);
            let qb: Quantity<Unitless> = Quantity::new(b);

            // Addition is commutative
            prop_assert!((((qa + qb).value() - (qb + qa).value()).abs() < 1e-9));

            // Value is preserved
            prop_assert!(((qa + qb).value() - (a + b)).abs() < 1e-9);
        }

        #[test]
        fn prop_from_length_preserves_value(v in -1e6..1e6f64) {
            let m = Meters::new(v);
            let u: Quantity<Unitless> = m.into();
            prop_assert!((u.value() - v).abs() < 1e-12);
        }

        #[test]
        fn prop_from_time_preserves_value(v in -1e6..1e6f64) {
            let t = Seconds::new(v);
            let u: Quantity<Unitless> = t.into();
            prop_assert!((u.value() - v).abs() < 1e-12);
        }
    }
}
