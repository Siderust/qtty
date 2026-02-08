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
