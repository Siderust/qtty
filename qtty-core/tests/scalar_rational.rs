#![cfg(feature = "scalar-rational")]

use num_rational::{Rational32, Rational64};
use qtty_core::length::Meter;
use qtty_core::scalar::{Exact, Scalar};
use qtty_core::Quantity;

// ─────────────────────────────────────────────────────────────────────────
// Rational64 tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_rational64_scalar_basic() {
    assert_eq!(Rational64::ZERO, Rational64::from_integer(0));
    assert_eq!(Rational64::ONE, Rational64::from_integer(1));
}

#[test]
fn test_rational64_abs() {
    let val = Rational64::from_integer(-5);
    assert_eq!(Scalar::abs(val), Rational64::from_integer(5));
}

#[test]
fn test_rational64_min_max() {
    let a = Rational64::from_integer(3);
    let b = Rational64::from_integer(7);
    assert_eq!(Scalar::min(a, b), Rational64::from_integer(3));
    assert_eq!(Scalar::max(a, b), Rational64::from_integer(7));
}

#[test]
fn test_rational64_rem_euclid() {
    let val = Rational64::from_integer(17);
    let modulus = Rational64::from_integer(5);
    assert_eq!(
        Scalar::rem_euclid(val, modulus),
        Rational64::from_integer(2)
    );
}

#[test]
fn test_rational64_exact_conversion() {
    let val = Rational64::new(3, 4);
    let f64_val = Exact::to_f64_approx(val);
    assert_eq!(f64_val, 0.75);
    let back: Rational64 = Exact::from_f64_approx(f64_val);
    assert!((Exact::to_f64_approx(back) - 0.75).abs() < 0.01);
}

#[test]
fn test_rational64_quantity() {
    let m = Quantity::<Meter, Rational64>::new(Rational64::from_integer(100));
    assert_eq!(m.value(), Rational64::from_integer(100));
}

#[test]
fn test_rational64_quantity_arithmetic() {
    let a = Quantity::<Meter, Rational64>::new(Rational64::from_integer(10));
    let b = Quantity::<Meter, Rational64>::new(Rational64::from_integer(5));
    let sum = a + b;
    assert_eq!(sum.value(), Rational64::from_integer(15));
}

// ─────────────────────────────────────────────────────────────────────────
// Rational32 tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_rational32_scalar_basic() {
    assert_eq!(Rational32::ZERO, Rational32::from_integer(0));
    assert_eq!(Rational32::ONE, Rational32::from_integer(1));
}

#[test]
fn test_rational32_abs() {
    let val = Rational32::from_integer(-5);
    assert_eq!(Scalar::abs(val), Rational32::from_integer(5));
}

#[test]
fn test_rational32_min_max() {
    let a = Rational32::from_integer(3);
    let b = Rational32::from_integer(7);
    assert_eq!(Scalar::min(a, b), Rational32::from_integer(3));
    assert_eq!(Scalar::max(a, b), Rational32::from_integer(7));
}

#[test]
fn test_rational32_rem_euclid() {
    let val = Rational32::from_integer(17);
    let modulus = Rational32::from_integer(5);
    assert_eq!(
        Scalar::rem_euclid(val, modulus),
        Rational32::from_integer(2)
    );
}

#[test]
fn test_rational32_exact_conversion() {
    let val = Rational32::new(1, 2);
    let f64_val = Exact::to_f64_approx(val);
    assert_eq!(f64_val, 0.5);
    let back: Rational32 = Exact::from_f64_approx(f64_val);
    assert!((Exact::to_f64_approx(back) - 0.5).abs() < 0.01);
}

#[test]
fn test_rational32_quantity() {
    let m = Quantity::<Meter, Rational32>::new(Rational32::from_integer(100));
    assert_eq!(m.value(), Rational32::from_integer(100));
}

#[test]
fn test_rational32_quantity_arithmetic() {
    let a = Quantity::<Meter, Rational32>::new(Rational32::from_integer(10));
    let b = Quantity::<Meter, Rational32>::new(Rational32::from_integer(5));
    let sum = a + b;
    assert_eq!(sum.value(), Rational32::from_integer(15));
}

// ─────────────────────────────────────────────────────────────────────────────
// Negative rem_euclid (coverage for the `if r < ZERO` branch)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_rational64_rem_euclid_negative() {
    // -7 rem_euclid 3 should be 2 (not -1)
    let val = Rational64::from_integer(-7);
    let modulus = Rational64::from_integer(3);
    let result = Scalar::rem_euclid(val, modulus);
    assert_eq!(result, Rational64::from_integer(2));
}

#[test]
fn test_rational64_rem_euclid_fractional_negative() {
    // -1/2 rem_euclid 1 should be 1/2
    let val = Rational64::new(-1, 2);
    let modulus = Rational64::from_integer(1);
    let result = Scalar::rem_euclid(val, modulus);
    assert_eq!(result, Rational64::new(1, 2));
}

#[test]
fn test_rational32_rem_euclid_negative() {
    let val = Rational32::from_integer(-7);
    let modulus = Rational32::from_integer(3);
    let result = Scalar::rem_euclid(val, modulus);
    assert_eq!(result, Rational32::from_integer(2));
}

#[test]
fn test_rational32_rem_euclid_fractional_negative() {
    let val = Rational32::new(-1, 2);
    let modulus = Rational32::from_integer(1);
    let result = Scalar::rem_euclid(val, modulus);
    assert_eq!(result, Rational32::new(1, 2));
}

// ─────────────────────────────────────────────────────────────────────────────
// Abs negative branch
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_rational64_abs_positive() {
    let val = Rational64::from_integer(5);
    assert_eq!(Scalar::abs(val), Rational64::from_integer(5));
}

#[test]
fn test_rational32_abs_positive() {
    let val = Rational32::from_integer(5);
    assert_eq!(Scalar::abs(val), Rational32::from_integer(5));
}

// ─────────────────────────────────────────────────────────────────────────────
// Min/Max else branches
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_rational64_min_first_smaller() {
    let a = Rational64::from_integer(3);
    let b = Rational64::from_integer(7);
    assert_eq!(Scalar::min(a, b), a);
}

#[test]
fn test_rational64_min_second_smaller() {
    let a = Rational64::from_integer(7);
    let b = Rational64::from_integer(3);
    assert_eq!(Scalar::min(a, b), b);
}

#[test]
fn test_rational64_max_first_larger() {
    let a = Rational64::from_integer(7);
    let b = Rational64::from_integer(3);
    assert_eq!(Scalar::max(a, b), a);
}

#[test]
fn test_rational64_max_second_larger() {
    let a = Rational64::from_integer(3);
    let b = Rational64::from_integer(7);
    assert_eq!(Scalar::max(a, b), b);
}

#[test]
fn test_rational32_min_second_smaller() {
    let a = Rational32::from_integer(7);
    let b = Rational32::from_integer(3);
    assert_eq!(Scalar::min(a, b), b);
}

#[test]
fn test_rational32_max_second_larger() {
    let a = Rational32::from_integer(3);
    let b = Rational32::from_integer(7);
    assert_eq!(Scalar::max(a, b), b);
}

// ─────────────────────────────────────────────────────────────────────────────
// Commutative multiplication: Rational * Quantity
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_rational64_commutative_mul() {
    let q = Quantity::<Meter, Rational64>::new(Rational64::from_integer(5));
    let result = Rational64::from_integer(3) * q;
    assert_eq!(result.value(), Rational64::from_integer(15));
}

#[test]
fn test_rational32_commutative_mul() {
    let q = Quantity::<Meter, Rational32>::new(Rational32::from_integer(5));
    let result = Rational32::from_integer(3) * q;
    assert_eq!(result.value(), Rational32::from_integer(15));
}

// ─────────────────────────────────────────────────────────────────────────────
// to_lossy for rational types
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_rational64_to_lossy() {
    use qtty_core::length::Kilometer;
    let km = Quantity::<Kilometer, Rational64>::new(Rational64::from_integer(2));
    let m: Quantity<Meter, Rational64> = km.to_lossy();
    let val_f64 = Exact::to_f64_approx(m.value());
    assert!((val_f64 - 2000.0).abs() < 1.0);
}

#[test]
fn test_rational32_to_lossy() {
    use qtty_core::length::Kilometer;
    let km = Quantity::<Kilometer, Rational32>::new(Rational32::from_integer(2));
    let m: Quantity<Meter, Rational32> = km.to_lossy();
    let val_f64 = Exact::to_f64_approx(m.value());
    assert!((val_f64 - 2000.0).abs() < 1.0);
}
