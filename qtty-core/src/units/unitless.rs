// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Dimensionless helpers.
//!
//! This module contains small adapters for working with dimensionless values.
//!
//! `Quantity<Unitless>` represents a true dimensionless ratio — for example, the result
//! of dividing two quantities with the same unit (`km / km`). It is **not** produced by
//! implicitly converting a dimensioned quantity; such erasure would silently discard
//! physical meaning.
//!
//! # Obtaining `Quantity<Unitless>`
//!
//! The canonical way to obtain a dimensionless quantity is through same-unit division:
//!
//! ```rust
//! use qtty_core::length::Meters;
//! use qtty_core::{Quantity, Unitless};
//!
//! let ratio: Quantity<Unitless> = Meters::new(3.0) / Meters::new(6.0);
//! assert!((ratio.value() - 0.5).abs() < 1e-12);
//! ```
//!
//! # Raw unit erasure (explicit, lossy)
//!
//! If you need to strip the unit tag for adapter or debugging purposes, use
//! [`Quantity::erase_unit_raw`]. The name makes the loss of dimensional meaning obvious:
//!
//! ```rust
//! use qtty_core::length::Kilometers;
//! use qtty_core::{Quantity, Unitless};
//!
//! let km = Kilometers::new(1.0);
//! let u: Quantity<Unitless> = km.erase_unit_raw();
//! assert_eq!(u.value(), 1.0); // raw stored number, NOT 1000.0 m
//! ```

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::units::length::Meters;
    use crate::units::time::Seconds;
    use crate::Unit;
    use crate::{Quantity, Unitless};
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
    // Same-unit division produces Unitless
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn same_unit_division_gives_unitless() {
        let a = Meters::new(10.0);
        let b = Meters::new(5.0);
        let ratio: Quantity<Unitless> = a / b;
        assert_abs_diff_eq!(ratio.value(), 2.0, epsilon = 1e-12);
    }

    #[test]
    fn same_unit_division_time() {
        let a = Seconds::new(100.0);
        let b = Seconds::new(20.0);
        let ratio: Quantity<Unitless> = a / b;
        assert_abs_diff_eq!(ratio.value(), 5.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Explicit erase_unit_raw
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn erase_unit_raw_preserves_value() {
        let m = Meters::new(42.0);
        let u: Quantity<Unitless> = m.erase_unit_raw();
        assert_eq!(u.value(), 42.0);
    }

    #[test]
    fn erase_unit_raw_non_canonical_preserves_raw() {
        use crate::units::length::Kilometers;
        let km = Kilometers::new(1.0);
        let u: Quantity<Unitless> = km.erase_unit_raw();
        assert_eq!(u.value(), 1.0); // NOT 1000.0
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
    }
}
