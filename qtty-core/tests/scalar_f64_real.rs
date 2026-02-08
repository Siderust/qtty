//! Tests for f64 Real trait methods (coverage for scalar.rs).

use qtty_core::scalar::{Real, Scalar, Transcendental};

// ─────────────────────────────────────────────────────────────────────────────
// Scalar methods
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f64_rem_euclid_positive() {
    assert_eq!(Scalar::rem_euclid(10.0_f64, 3.0), 1.0);
    assert_eq!(Scalar::rem_euclid(7.0_f64, 2.5), 2.0);
}

#[test]
fn f64_rem_euclid_negative() {
    let r = Scalar::rem_euclid(-10.0_f64, 3.0);
    assert!((r - 2.0).abs() < 1e-15);
}

// ─────────────────────────────────────────────────────────────────────────────
// Real methods
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f64_signum() {
    assert_eq!(Real::signum(5.0_f64), 1.0);
    assert_eq!(Real::signum(-5.0_f64), -1.0);
    // IEEE 754: signum(+0.0) is not 0, but implementation-defined
    // Rust's f64::signum(0.0) returns 1.0
    assert_eq!(Real::signum(0.0_f64), 1.0);
    assert!(Real::signum(f64::NAN).is_nan());
}

#[test]
fn f64_is_finite() {
    assert!(Real::is_finite(42.0_f64));
    assert!(!Real::is_finite(f64::INFINITY));
    assert!(!Real::is_finite(f64::NEG_INFINITY));
    assert!(!Real::is_finite(f64::NAN));
}

#[test]
fn f64_is_nan() {
    assert!(Real::is_nan(f64::NAN));
    assert!(!Real::is_nan(0.0_f64));
}

#[test]
fn f64_is_infinite() {
    assert!(Real::is_infinite(f64::INFINITY));
    assert!(Real::is_infinite(f64::NEG_INFINITY));
    assert!(!Real::is_infinite(42.0_f64));
}

#[test]
fn f64_mul_add() {
    let result = Real::mul_add(2.0_f64, 3.0, 4.0);
    assert!((result - 10.0).abs() < 1e-15);
}

#[test]
fn f64_floor() {
    assert_eq!(Real::floor(3.7_f64), 3.0);
    assert_eq!(Real::floor(-3.7_f64), -4.0);
    assert_eq!(Real::floor(5.0_f64), 5.0);
}

#[test]
fn f64_ceil() {
    assert_eq!(Real::ceil(3.2_f64), 4.0);
    assert_eq!(Real::ceil(-3.2_f64), -3.0);
    assert_eq!(Real::ceil(5.0_f64), 5.0);
}

#[test]
fn f64_round() {
    assert_eq!(Real::round(3.5_f64), 4.0);
    assert_eq!(Real::round(3.4_f64), 3.0);
    assert_eq!(Real::round(-3.5_f64), -4.0);
}

#[test]
fn f64_trunc() {
    assert_eq!(Real::trunc(3.9_f64), 3.0);
    assert_eq!(Real::trunc(-3.9_f64), -3.0);
}

#[test]
fn f64_fract() {
    let f = Real::fract(3.75_f64);
    assert!((f - 0.75).abs() < 1e-15);
    let f = Real::fract(-3.75_f64);
    assert!((f - (-0.75)).abs() < 1e-15);
}

#[test]
fn f64_powf() {
    let r = Real::powf(2.0_f64, 10.0);
    assert!((r - 1024.0).abs() < 1e-10);
}

#[test]
fn f64_powi() {
    let r = Real::powi(2.0_f64, 10);
    assert!((r - 1024.0).abs() < 1e-10);
    let r = Real::powi(3.0_f64, 0);
    assert!((r - 1.0).abs() < 1e-15);
}

#[test]
fn f64_sqrt() {
    assert!((Real::sqrt(16.0_f64) - 4.0).abs() < 1e-15);
    assert!((Real::sqrt(2.0_f64) - core::f64::consts::SQRT_2).abs() < 1e-15);
}

#[test]
fn f64_cbrt() {
    assert!((Real::cbrt(27.0_f64) - 3.0).abs() < 1e-15);
    assert!((Real::cbrt(-8.0_f64) - (-2.0)).abs() < 1e-15);
}

#[test]
fn f64_ln() {
    assert!((Real::ln(1.0_f64)).abs() < 1e-15);
    assert!((Real::ln(core::f64::consts::E) - 1.0).abs() < 1e-15);
}

#[test]
fn f64_log10() {
    assert!((Real::log10(100.0_f64) - 2.0).abs() < 1e-15);
    assert!((Real::log10(1.0_f64)).abs() < 1e-15);
}

#[test]
fn f64_log2() {
    assert!((Real::log2(8.0_f64) - 3.0).abs() < 1e-15);
    assert!((Real::log2(1.0_f64)).abs() < 1e-15);
}

#[test]
fn f64_log() {
    // log base 10 of 1000
    assert!((Real::log(1000.0_f64, 10.0) - 3.0).abs() < 1e-12);
    // log base 2 of 16
    assert!((Real::log(16.0_f64, 2.0) - 4.0).abs() < 1e-12);
}

#[test]
fn f64_exp() {
    assert!((Real::exp(0.0_f64) - 1.0).abs() < 1e-15);
    assert!((Real::exp(1.0_f64) - core::f64::consts::E).abs() < 1e-14);
}

#[test]
fn f64_exp2() {
    assert!((Real::exp2(0.0_f64) - 1.0).abs() < 1e-15);
    assert!((Real::exp2(10.0_f64) - 1024.0).abs() < 1e-10);
}

#[test]
fn f64_hypot() {
    assert!((Real::hypot(3.0_f64, 4.0) - 5.0).abs() < 1e-15);
    assert!((Real::hypot(5.0_f64, 12.0) - 13.0).abs() < 1e-13);
}

// ─────────────────────────────────────────────────────────────────────────────
// Real constants
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f64_real_constants() {
    assert!((f64::PI - core::f64::consts::PI).abs() < 1e-15);
    assert!((f64::TAU - core::f64::consts::TAU).abs() < 1e-15);
    assert!((f64::E - core::f64::consts::E).abs() < 1e-15);
    assert!(f64::INFINITY.is_infinite());
    assert!(f64::NEG_INFINITY.is_infinite());
    assert!(f64::NAN.is_nan());
}

#[test]
fn f64_from_to_f64_identity() {
    let val = 123.456_f64;
    assert_eq!(f64::from_f64(val), val);
    assert_eq!(val.to_f64(), val);
}

// ─────────────────────────────────────────────────────────────────────────────
// Transcendental methods — f64
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f64_sin() {
    let pi = core::f64::consts::PI;
    assert!((Transcendental::sin(0.0_f64)).abs() < 1e-15);
    assert!((Transcendental::sin(pi / 2.0) - 1.0).abs() < 1e-15);
    assert!((Transcendental::sin(pi)).abs() < 1e-15);
}

#[test]
fn f64_cos() {
    let pi = core::f64::consts::PI;
    assert!((Transcendental::cos(0.0_f64) - 1.0).abs() < 1e-15);
    assert!((Transcendental::cos(pi / 2.0)).abs() < 1e-15);
    assert!((Transcendental::cos(pi) + 1.0).abs() < 1e-15);
}

#[test]
fn f64_tan() {
    let pi = core::f64::consts::PI;
    assert!((Transcendental::tan(0.0_f64)).abs() < 1e-15);
    assert!((Transcendental::tan(pi / 4.0) - 1.0).abs() < 1e-14);
}

#[test]
fn f64_sin_cos() {
    let pi = core::f64::consts::PI;
    let (s, c) = Transcendental::sin_cos(pi / 6.0_f64);
    assert!((s - 0.5).abs() < 1e-15);
    assert!((c - (3.0_f64).sqrt() / 2.0).abs() < 1e-15);
}

#[test]
fn f64_asin() {
    assert!((Transcendental::asin(0.5_f64) - core::f64::consts::FRAC_PI_6).abs() < 1e-15);
    assert!((Transcendental::asin(1.0_f64) - core::f64::consts::FRAC_PI_2).abs() < 1e-15);
}

#[test]
fn f64_acos() {
    assert!((Transcendental::acos(0.5_f64) - core::f64::consts::FRAC_PI_3).abs() < 1e-15);
    assert!((Transcendental::acos(1.0_f64)).abs() < 1e-15);
}

#[test]
fn f64_atan() {
    assert!((Transcendental::atan(1.0_f64) - core::f64::consts::FRAC_PI_4).abs() < 1e-15);
    assert!((Transcendental::atan(0.0_f64)).abs() < 1e-15);
}

#[test]
fn f64_atan2() {
    let pi = core::f64::consts::PI;
    // atan2(1, 1) = PI/4
    assert!((Transcendental::atan2(1.0_f64, 1.0) - pi / 4.0).abs() < 1e-15);
    // atan2(0, -1) = PI
    assert!((Transcendental::atan2(0.0_f64, -1.0) - pi).abs() < 1e-15);
    // atan2(-1, 0) = -PI/2
    assert!((Transcendental::atan2(-1.0_f64, 0.0) + pi / 2.0).abs() < 1e-15);
}

#[test]
fn f64_sinh() {
    assert!((Transcendental::sinh(0.0_f64)).abs() < 1e-15);
    // sinh(1) ≈ 1.1752
    assert!((Transcendental::sinh(1.0_f64) - 1.0_f64.sinh()).abs() < 1e-15);
}

#[test]
fn f64_cosh() {
    assert!((Transcendental::cosh(0.0_f64) - 1.0).abs() < 1e-15);
    assert!((Transcendental::cosh(1.0_f64) - 1.0_f64.cosh()).abs() < 1e-15);
}

#[test]
fn f64_tanh() {
    assert!((Transcendental::tanh(0.0_f64)).abs() < 1e-15);
    assert!((Transcendental::tanh(1.0_f64) - 1.0_f64.tanh()).abs() < 1e-15);
}

#[test]
fn f64_asinh() {
    assert!((Transcendental::asinh(0.0_f64)).abs() < 1e-15);
    assert!((Transcendental::asinh(1.0_f64) - 1.0_f64.asinh()).abs() < 1e-15);
}

#[test]
fn f64_acosh() {
    assert!((Transcendental::acosh(1.0_f64)).abs() < 1e-15);
    assert!((Transcendental::acosh(2.0_f64) - 2.0_f64.acosh()).abs() < 1e-15);
}

#[test]
fn f64_atanh() {
    assert!((Transcendental::atanh(0.0_f64)).abs() < 1e-15);
    assert!((Transcendental::atanh(0.5_f64) - 0.5_f64.atanh()).abs() < 1e-15);
}
