//! # Unitless Module
//!
//! Support for dimensionless quantities and conversions.

use crate::{Quantity, Unitless};
use crate::units::length::LengthUnit;

impl<U: LengthUnit> From<Quantity<U>> for Quantity<Unitless> {
    fn from(length: Quantity<U>) -> Self {
        Self::new(length.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Unit;
    use crate::units::length::Meters;
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
    // Conversion from length
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn from_length() {
        let m = Meters::new(42.0);
        let u: Quantity<Unitless> = m.into();
        assert_eq!(u.value(), 42.0);
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
    }
}
