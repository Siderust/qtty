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

// ─────────────────────────────────────────────────────────────────────────────
// Decimal Real methods (coverage for scalar.rs decimal_impl)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_decimal_signum_positive() {
    let val = Decimal::from(42);
    assert_eq!(Real::signum(val), Decimal::ONE);
}

#[test]
fn test_decimal_signum_negative() {
    let val = Decimal::from(-42);
    assert_eq!(Real::signum(val), Decimal::NEGATIVE_ONE);
}

#[test]
fn test_decimal_signum_zero() {
    assert_eq!(Real::signum(Decimal::ZERO), Decimal::ZERO);
}

#[test]
fn test_decimal_mul_add() {
    let result = Real::mul_add(Decimal::from(2), Decimal::from(3), Decimal::from(4));
    assert_eq!(result, Decimal::from(10));
}

#[test]
fn test_decimal_floor() {
    let val = Decimal::try_from(3.7).unwrap();
    assert_eq!(Real::floor(val), Decimal::from(3));
    let val = Decimal::try_from(-3.7).unwrap();
    assert_eq!(Real::floor(val), Decimal::from(-4));
}

#[test]
fn test_decimal_ceil() {
    let val = Decimal::try_from(3.2).unwrap();
    assert_eq!(Real::ceil(val), Decimal::from(4));
    let val = Decimal::try_from(-3.2).unwrap();
    assert_eq!(Real::ceil(val), Decimal::from(-3));
}

#[test]
fn test_decimal_round() {
    let val = Decimal::try_from(3.5).unwrap();
    assert_eq!(Real::round(val), Decimal::from(4));
    let val = Decimal::try_from(3.4).unwrap();
    assert_eq!(Real::round(val), Decimal::from(3));
}

#[test]
fn test_decimal_trunc() {
    let val = Decimal::try_from(3.9).unwrap();
    assert_eq!(Real::trunc(val), Decimal::from(3));
    let val = Decimal::try_from(-3.9).unwrap();
    assert_eq!(Real::trunc(val), Decimal::from(-3));
}

#[test]
fn test_decimal_fract() {
    let val = Decimal::try_from(3.75).unwrap();
    let frac = Real::fract(val);
    assert!((Exact::to_f64_approx(frac) - 0.75).abs() < 0.01);
}

#[test]
fn test_decimal_powf() {
    let result = Real::powf(Decimal::from(2), Decimal::from(10));
    assert!((Exact::to_f64_approx(result) - 1024.0).abs() < 1.0);
}

#[test]
fn test_decimal_powi() {
    let result = Real::powi(Decimal::from(3), 4);
    assert_eq!(result, Decimal::from(81));
}

#[test]
fn test_decimal_sqrt() {
    let result = Real::sqrt(Decimal::from(16));
    assert_eq!(result, Decimal::from(4));
}

#[test]
fn test_decimal_cbrt() {
    let result = Real::cbrt(Decimal::from(27));
    assert!((Exact::to_f64_approx(result) - 3.0).abs() < 0.01);
}

#[test]
fn test_decimal_ln() {
    let result = Real::ln(Decimal::ONE);
    assert!((Exact::to_f64_approx(result)).abs() < 0.01);
}

#[test]
fn test_decimal_log10() {
    let result = Real::log10(Decimal::from(100));
    assert!((Exact::to_f64_approx(result) - 2.0).abs() < 0.01);
}

#[test]
fn test_decimal_log2() {
    let result = Real::log2(Decimal::from(8));
    assert!((Exact::to_f64_approx(result) - 3.0).abs() < 0.01);
}

#[test]
fn test_decimal_log_base() {
    let result = Real::log(Decimal::from(1000), Decimal::from(10));
    assert!((Exact::to_f64_approx(result) - 3.0).abs() < 0.01);
}

#[test]
fn test_decimal_exp() {
    let result = Real::exp(Decimal::ZERO);
    assert!((Exact::to_f64_approx(result) - 1.0).abs() < 0.01);
}

#[test]
fn test_decimal_exp2() {
    let result = Real::exp2(Decimal::from(10));
    assert!((Exact::to_f64_approx(result) - 1024.0).abs() < 1.0);
}

#[test]
fn test_decimal_hypot() {
    let result = Real::hypot(Decimal::from(3), Decimal::from(4));
    assert!((Exact::to_f64_approx(result) - 5.0).abs() < 0.01);
}

#[test]
fn test_decimal_rem_euclid_negative() {
    // Test negative remainder path
    let val = Decimal::from(-10);
    let modulus = Decimal::from(3);
    let result = Scalar::rem_euclid(val, modulus);
    assert!((Exact::to_f64_approx(result) - 2.0).abs() < 0.01);
}

// ─────────────────────────────────────────────────────────────────────────────
// Commutative multiplication: Decimal * Quantity
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_decimal_commutative_mul() {
    let q = Quantity::<Meter, Decimal>::new(Decimal::from(5));
    let result = Decimal::from(3) * q;
    assert_eq!(result.value(), Decimal::from(15));
    let result2 = q * Decimal::from(3);
    assert_eq!(result2.value(), Decimal::from(15));
}

// ─────────────────────────────────────────────────────────────────────────────
// Decimal Quantity: to_lossy, Display, to, cast
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_decimal_quantity_to() {
    use qtty_core::length::Kilometer;
    let km = Quantity::<Kilometer, Decimal>::new(Decimal::from(1));
    let m: Quantity<Meter, Decimal> = km.to();
    assert_eq!(m.value(), Decimal::from(1000));
}

#[test]
fn test_decimal_quantity_display() {
    let m = Quantity::<Meter, Decimal>::new(Decimal::from(42));
    assert_eq!(format!("{}", m), "42 m");
}

#[test]
fn test_decimal_to_lossy() {
    use qtty_core::length::Kilometer;
    let m = Quantity::<Meter, Decimal>::new(Decimal::from(1500));
    let km: Quantity<Kilometer, Decimal> = m.to_lossy();
    // 1500/1000 = 1.5, to_f64_approx and back should give close to 1.5
    assert!((Exact::to_f64_approx(km.value()) - 1.5).abs() < 0.01);
}
