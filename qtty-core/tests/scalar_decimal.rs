#![cfg(feature = "scalar-decimal")]

use qtty_core::length::Meter;
use qtty_core::scalar::{Exact, Real, Scalar};
use qtty_core::Quantity;
use rust_decimal::Decimal;

#[test]
fn test_decimal_scalar_basic() {
    assert_eq!(Decimal::ZERO, Decimal::from(0));
    assert_eq!(Decimal::ONE, Decimal::from(1));
}

#[test]
fn test_decimal_abs() {
    let val = Decimal::from(-5);
    assert_eq!(Scalar::abs(val), Decimal::from(5));
}

#[test]
fn test_decimal_min_max() {
    let a = Decimal::from(3);
    let b = Decimal::from(7);
    assert_eq!(Scalar::min(a, b), Decimal::from(3));
    assert_eq!(Scalar::max(a, b), Decimal::from(7));
}

#[test]
fn test_decimal_rem_euclid() {
    let val = Decimal::from(17);
    let modulus = Decimal::from(5);
    assert_eq!(Scalar::rem_euclid(val, modulus), Decimal::from(2));
}

#[test]
fn test_decimal_real_constants() {
    let pi = Decimal::PI;
    assert!(pi.to_f64() > 3.14);
    assert!(pi.to_f64() < 3.15);
}

#[test]
fn test_decimal_from_to_f64() {
    let val = Decimal::from_f64(42.5);
    assert!((val.to_f64() - 42.5).abs() < 0.01);
}

#[test]
fn test_decimal_is_nan_infinite() {
    let val = Decimal::from(100);
    assert!(!val.is_nan());
    assert!(!val.is_infinite());
    assert!(val.is_finite());
}

#[test]
fn test_decimal_exact_conversion() {
    let val = Decimal::from(1000);
    let f64_val = Exact::to_f64_approx(val);
    assert_eq!(f64_val, 1000.0);
    let back: Decimal = Exact::from_f64_approx(f64_val);
    assert_eq!(back, Decimal::from(1000));
}

#[test]
fn test_decimal_quantity() {
    let m = Quantity::<Meter, Decimal>::new(Decimal::from(100));
    assert_eq!(m.value(), Decimal::from(100));
}

#[test]
fn test_decimal_quantity_arithmetic() {
    let a = Quantity::<Meter, Decimal>::new(Decimal::from(10));
    let b = Quantity::<Meter, Decimal>::new(Decimal::from(5));
    let sum = a + b;
    assert_eq!(sum.value(), Decimal::from(15));
}
