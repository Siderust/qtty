// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Regression tests for issues identified in the QTTY audit report (2026-04-08).
//!
//! - QTTY-001: Cross-unit comparison symmetry
//! - QTTY-002: Integer `abs()` at signed minimum
//! - QTTY-003: `to_lossy()` / `checked_to_lossy()` overflow detection

use core::cmp::Ordering;
use qtty_core::length::{Kilometer, Meter};
use qtty_core::Quantity;

// ═════════════════════════════════════════════════════════════════════════════
// QTTY-001: Cross-unit comparison symmetry
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "cross-unit-ops")]
mod cross_unit_symmetry {
    use super::*;
    use qtty_core::angular::{Degree, Radian};

    /// `a == b` iff `b == a` for Degree vs Radian.
    #[test]
    fn eq_symmetry_degree_radian() {
        let r = Quantity::<Radian>::new(-190131.7250991714);
        let d: Quantity<Degree> = r.to();
        assert_eq!(
            r == d,
            d == r,
            "PartialEq must be symmetric (Degree ↔ Radian)"
        );
    }

    /// `a == b` iff `b == a` for Meter vs Kilometer.
    #[test]
    fn eq_symmetry_meter_kilometer() {
        let km = Quantity::<Kilometer>::new(1.0);
        let m = Quantity::<Meter>::new(1000.0);
        assert!(km == m, "1 km should equal 1000 m");
        assert!(m == km, "1000 m should equal 1 km");
    }

    /// Large-magnitude conversion case.
    #[test]
    fn eq_symmetry_large_magnitude() {
        let m = Quantity::<Meter>::new(1e15);
        let km: Quantity<Kilometer> = m.to();
        assert_eq!(m == km, km == m, "symmetry at large magnitude");
    }

    /// Non-power-of-ten conversion (Degree/Radian ratio involves π).
    #[test]
    fn eq_symmetry_non_power_of_ten() {
        let d = Quantity::<Degree>::new(45.0);
        let r: Quantity<Radian> = d.to();
        assert_eq!(d == r, r == d, "symmetry for 45°");
    }

    /// `partial_cmp(a, b)` and `partial_cmp(b, a)` are order-consistent.
    #[test]
    fn partial_cmp_consistency_degree_radian() {
        let r = Quantity::<Radian>::new(-190131.7250991714);
        let d: Quantity<Degree> = r.to();
        let fwd = r.partial_cmp(&d);
        let rev = d.partial_cmp(&r);
        match (fwd, rev) {
            (Some(Ordering::Less), Some(Ordering::Greater))
            | (Some(Ordering::Greater), Some(Ordering::Less))
            | (Some(Ordering::Equal), Some(Ordering::Equal)) => {}
            (None, None) => {} // both NaN is acceptable
            other => panic!("partial_cmp inconsistent: forward={other:?}"),
        }
    }

    /// `partial_cmp(a, b)` and `partial_cmp(b, a)` consistent for Meter/Kilometer.
    #[test]
    fn partial_cmp_consistency_meter_kilometer() {
        let km = Quantity::<Kilometer>::new(2.0);
        let m = Quantity::<Meter>::new(1500.0);
        assert_eq!(km.partial_cmp(&m), Some(Ordering::Greater));
        assert_eq!(m.partial_cmp(&km), Some(Ordering::Less));
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// QTTY-002: Integer abs() at signed minimum values
// ═════════════════════════════════════════════════════════════════════════════

mod integer_abs_boundary {
    use super::*;

    #[test]
    fn abs_i8_min_saturates() {
        let q = Quantity::<Meter, i8>::new(i8::MIN);
        assert_eq!(q.abs().value(), i8::MAX);
    }

    #[test]
    fn abs_i16_min_saturates() {
        let q = Quantity::<Meter, i16>::new(i16::MIN);
        assert_eq!(q.abs().value(), i16::MAX);
    }

    #[test]
    fn abs_i32_min_saturates() {
        let q = Quantity::<Meter, i32>::new(i32::MIN);
        assert_eq!(q.abs().value(), i32::MAX);
    }

    #[test]
    fn abs_i64_min_saturates() {
        let q = Quantity::<Meter, i64>::new(i64::MIN);
        assert_eq!(q.abs().value(), i64::MAX);
    }

    #[test]
    fn abs_i128_min_saturates() {
        let q = Quantity::<Meter, i128>::new(i128::MIN);
        assert_eq!(q.abs().value(), i128::MAX);
    }

    /// Normal abs still works as expected.
    #[test]
    fn abs_normal_values() {
        assert_eq!(Quantity::<Meter, i32>::new(-42).abs().value(), 42);
        assert_eq!(Quantity::<Meter, i32>::new(42).abs().value(), 42);
        assert_eq!(Quantity::<Meter, i32>::new(0).abs().value(), 0);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// QTTY-003: to_lossy() and checked_to_lossy() behavior
// ═════════════════════════════════════════════════════════════════════════════

mod lossy_conversion {
    use super::*;

    /// Truncation: 1500 m → 1 km (truncated from 1.5).
    #[test]
    fn to_lossy_truncation() {
        let m = Quantity::<Meter, i32>::new(1500);
        let km: Quantity<Kilometer, i32> = m.to_lossy();
        assert_eq!(km.value(), 1);
    }

    /// Clipping: 1 km → i8 meters saturates at 127.
    #[test]
    fn to_lossy_clips_i8() {
        let km = Quantity::<Kilometer, i8>::new(1);
        let m: Quantity<Meter, i8> = km.to_lossy();
        assert_eq!(m.value(), 127); // saturated
    }

    /// checked_to_lossy detects overflow for i8.
    #[test]
    fn checked_to_lossy_overflow_i8() {
        let km = Quantity::<Kilometer, i8>::new(1);
        let result: Option<Quantity<Meter, i8>> = km.checked_to_lossy();
        assert!(result.is_none(), "1 km does not fit in i8 meters");
    }

    /// checked_to_lossy allows valid truncation.
    #[test]
    fn checked_to_lossy_valid_truncation() {
        let m = Quantity::<Meter, i32>::new(1500);
        let km: Option<Quantity<Kilometer, i32>> = m.checked_to_lossy();
        assert_eq!(km.unwrap().value(), 1);
    }

    /// checked_to_lossy detects overflow for i32 large values.
    #[test]
    fn checked_to_lossy_overflow_large() {
        let km = Quantity::<Kilometer, i32>::new(i32::MAX);
        let result: Option<Quantity<Meter, i32>> = km.checked_to_lossy();
        assert!(result.is_none(), "i32::MAX km overflows i32 meters");
    }

    /// checked_to_lossy with zero value always succeeds.
    #[test]
    fn checked_to_lossy_zero() {
        let km = Quantity::<Kilometer, i8>::new(0);
        let result: Option<Quantity<Meter, i8>> = km.checked_to_lossy();
        assert_eq!(result.unwrap().value(), 0);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// QTTY-004: __impl_cross_ops_one_to_many! large-magnitude overflow
//
// LightYear (ratio ≈ 9.461e15 m) and Yottameter (ratio = 1e24 m) go through
// the `_between!` helper, which previously canonicalized by multiplying both
// values by their absolute ratios, overflowing to ±inf for large magnitudes
// and producing spurious equality.  The fix scales only the smaller-ratio side
// by a factor ≤ 1, matching the safe `impl_unit_cross_unit_ops!` path.
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(all(feature = "astro", feature = "cross-unit-ops"))]
mod cross_unit_one_to_many_overflow {
    use core::cmp::Ordering;
    use qtty_core::length::{LightYear, Yottameter};
    use qtty_core::Quantity;

    /// Two physically distinct large-magnitude quantities must not compare equal.
    ///
    /// Old code: `1e300 * RATIO_LY → inf`, `2e300 * RATIO_YM → inf`, `inf == inf → true`.
    /// Fixed:    smaller-ratio side scaled by ratio ≤ 1, no overflow.
    #[test]
    fn large_magnitude_distinct_values_are_not_equal() {
        let ly = Quantity::<LightYear>::new(1e300);
        let ym = Quantity::<Yottameter>::new(2e300);
        assert_ne!(ly, ym, "1e300 ly must not equal 2e300 Ym");
        assert_ne!(ym, ly, "2e300 Ym must not equal 1e300 ly (symmetry)");
    }

    /// The spurious-equality fix must also hold for `partial_cmp`.
    #[test]
    fn large_magnitude_partial_cmp_not_equal() {
        let ly = Quantity::<LightYear>::new(1e300);
        let ym = Quantity::<Yottameter>::new(2e300);
        // LightYear has a much smaller ratio than Yottameter, so 1e300 ly < 2e300 Ym
        assert_eq!(
            ly.partial_cmp(&ym),
            Some(Ordering::Less),
            "1e300 ly < 2e300 Ym"
        );
        assert_eq!(
            ym.partial_cmp(&ly),
            Some(Ordering::Greater),
            "2e300 Ym > 1e300 ly"
        );
    }

    /// A correctly converted value must still compare equal.
    #[test]
    fn converted_value_is_equal() {
        let ly = Quantity::<LightYear>::new(1.0);
        let ym: Quantity<Yottameter> = ly.to();
        assert_eq!(ly, ym, "1 ly must equal its Yottameter equivalent");
        assert_eq!(ym, ly, "symmetry: Ym equivalent == original ly");
    }

    /// Ordering is consistent: `cmp(a, b)` reverses `cmp(b, a)`.
    #[test]
    fn partial_cmp_consistency() {
        let ly = Quantity::<LightYear>::new(1.0);
        let ym: Quantity<Yottameter> = ly.to();
        let fwd = ly.partial_cmp(&ym);
        let rev = ym.partial_cmp(&ly);
        match (fwd, rev) {
            (Some(Ordering::Less), Some(Ordering::Greater))
            | (Some(Ordering::Greater), Some(Ordering::Less))
            | (Some(Ordering::Equal), Some(Ordering::Equal)) => {}
            (None, None) => {}
            other => panic!("partial_cmp inconsistent: {other:?}"),
        }
    }
}
