//! Tests for f32 Scalar + Real + Transcendental trait methods (coverage for scalar.rs).

use qtty_core::scalar::{Real, Scalar, Transcendental};

// ─────────────────────────────────────────────────────────────────────────────
// Scalar methods for f32
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_scalar_zero_one() {
    assert_eq!(f32::ZERO, 0.0_f32);
    assert_eq!(f32::ONE, 1.0_f32);
}

#[test]
fn f32_abs() {
    assert_eq!(Scalar::abs(-5.0_f32), 5.0);
    assert_eq!(Scalar::abs(5.0_f32), 5.0);
    assert_eq!(Scalar::abs(0.0_f32), 0.0);
}

#[test]
fn f32_min() {
    assert_eq!(Scalar::min(3.0_f32, 7.0), 3.0);
    assert_eq!(Scalar::min(7.0_f32, 3.0), 3.0);
    assert_eq!(Scalar::min(-1.0_f32, 1.0), -1.0);
}

#[test]
fn f32_max() {
    assert_eq!(Scalar::max(3.0_f32, 7.0), 7.0);
    assert_eq!(Scalar::max(7.0_f32, 3.0), 7.0);
    assert_eq!(Scalar::max(-1.0_f32, 1.0), 1.0);
}

#[test]
fn f32_rem_euclid() {
    let r = Scalar::rem_euclid(10.0_f32, 3.0);
    assert!((r - 1.0).abs() < 1e-6);
    let r = Scalar::rem_euclid(-10.0_f32, 3.0);
    assert!((r - 2.0).abs() < 1e-6);
}

// ─────────────────────────────────────────────────────────────────────────────
// Real constants and conversions
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_real_constants() {
    assert!((f32::PI - core::f32::consts::PI).abs() < 1e-6);
    assert!((f32::TAU - core::f32::consts::TAU).abs() < 1e-5);
    assert!((f32::E - core::f32::consts::E).abs() < 1e-6);
    assert!(f32::INFINITY.is_infinite());
    assert!(f32::NEG_INFINITY.is_infinite());
    assert!(f32::NAN.is_nan());
}

#[test]
fn f32_from_to_f64() {
    let val = f32::from_f64(42.5);
    assert!((val - 42.5_f32).abs() < 1e-6);
    assert!((val.to_f64() - 42.5).abs() < 1e-6);
}

// ─────────────────────────────────────────────────────────────────────────────
// Real methods
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_signum() {
    assert_eq!(Real::signum(5.0_f32), 1.0);
    assert_eq!(Real::signum(-5.0_f32), -1.0);
    assert!(Real::signum(f32::NAN).is_nan());
}

#[test]
fn f32_is_nan() {
    assert!(Real::is_nan(f32::NAN));
    assert!(!Real::is_nan(0.0_f32));
}

#[test]
fn f32_is_infinite() {
    assert!(Real::is_infinite(f32::INFINITY));
    assert!(Real::is_infinite(f32::NEG_INFINITY));
    assert!(!Real::is_infinite(42.0_f32));
}

#[test]
fn f32_is_finite() {
    assert!(Real::is_finite(42.0_f32));
    assert!(!Real::is_finite(f32::INFINITY));
    assert!(!Real::is_finite(f32::NAN));
}

#[test]
fn f32_mul_add() {
    let r = Real::mul_add(2.0_f32, 3.0, 4.0);
    assert!((r - 10.0).abs() < 1e-6);
}

#[test]
fn f32_floor() {
    assert_eq!(Real::floor(3.7_f32), 3.0);
    assert_eq!(Real::floor(-3.7_f32), -4.0);
}

#[test]
fn f32_ceil() {
    assert_eq!(Real::ceil(3.2_f32), 4.0);
    assert_eq!(Real::ceil(-3.2_f32), -3.0);
}

#[test]
fn f32_round() {
    assert_eq!(Real::round(3.5_f32), 4.0);
    assert_eq!(Real::round(3.4_f32), 3.0);
    assert_eq!(Real::round(-3.5_f32), -4.0);
}

#[test]
fn f32_trunc() {
    assert_eq!(Real::trunc(3.9_f32), 3.0);
    assert_eq!(Real::trunc(-3.9_f32), -3.0);
}

#[test]
fn f32_fract() {
    let f = Real::fract(3.75_f32);
    assert!((f - 0.75).abs() < 1e-6);
}

#[test]
fn f32_powf() {
    let r = Real::powf(2.0_f32, 10.0);
    assert!((r - 1024.0).abs() < 1e-2);
}

#[test]
fn f32_powi() {
    let r = Real::powi(2.0_f32, 10);
    assert!((r - 1024.0).abs() < 1e-2);
}

#[test]
fn f32_sqrt() {
    assert!((Real::sqrt(16.0_f32) - 4.0).abs() < 1e-6);
    assert!((Real::sqrt(2.0_f32) - core::f32::consts::SQRT_2).abs() < 1e-6);
}

#[test]
fn f32_cbrt() {
    assert!((Real::cbrt(27.0_f32) - 3.0).abs() < 1e-5);
    assert!((Real::cbrt(-8.0_f32) + 2.0).abs() < 1e-5);
}

#[test]
fn f32_ln() {
    assert!((Real::ln(1.0_f32)).abs() < 1e-6);
    assert!((Real::ln(core::f32::consts::E) - 1.0).abs() < 1e-6);
}

#[test]
fn f32_log10() {
    assert!((Real::log10(100.0_f32) - 2.0).abs() < 1e-5);
}

#[test]
fn f32_log2() {
    assert!((Real::log2(8.0_f32) - 3.0).abs() < 1e-5);
}

#[test]
fn f32_log() {
    assert!((Real::log(1000.0_f32, 10.0) - 3.0).abs() < 1e-3);
}

#[test]
fn f32_exp() {
    assert!((Real::exp(0.0_f32) - 1.0).abs() < 1e-6);
    assert!((Real::exp(1.0_f32) - core::f32::consts::E).abs() < 1e-5);
}

#[test]
fn f32_exp2() {
    assert!((Real::exp2(0.0_f32) - 1.0).abs() < 1e-6);
    assert!((Real::exp2(10.0_f32) - 1024.0).abs() < 1e-2);
}

#[test]
fn f32_hypot() {
    assert!((Real::hypot(3.0_f32, 4.0) - 5.0).abs() < 1e-6);
}

// ─────────────────────────────────────────────────────────────────────────────
// Transcendental methods — f32
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_sin() {
    let pi = core::f32::consts::PI;
    assert!((Transcendental::sin(0.0_f32)).abs() < 1e-6);
    assert!((Transcendental::sin(pi / 2.0) - 1.0).abs() < 1e-6);
}

#[test]
fn f32_cos() {
    let pi = core::f32::consts::PI;
    assert!((Transcendental::cos(0.0_f32) - 1.0).abs() < 1e-6);
    assert!((Transcendental::cos(pi) + 1.0).abs() < 1e-6);
}

#[test]
fn f32_tan() {
    let pi = core::f32::consts::PI;
    assert!((Transcendental::tan(0.0_f32)).abs() < 1e-6);
    assert!((Transcendental::tan(pi / 4.0) - 1.0).abs() < 1e-5);
}

#[test]
fn f32_sin_cos() {
    let pi = core::f32::consts::PI;
    let (s, c) = Transcendental::sin_cos(pi / 6.0_f32);
    assert!((s - 0.5).abs() < 1e-6);
    assert!((c - (3.0_f32).sqrt() / 2.0).abs() < 1e-6);
}

#[test]
fn f32_asin() {
    assert!((Transcendental::asin(0.5_f32) - core::f32::consts::FRAC_PI_6).abs() < 1e-6);
    assert!((Transcendental::asin(1.0_f32) - core::f32::consts::FRAC_PI_2).abs() < 1e-6);
}

#[test]
fn f32_acos() {
    assert!((Transcendental::acos(0.5_f32) - core::f32::consts::FRAC_PI_3).abs() < 1e-6);
    assert!((Transcendental::acos(1.0_f32)).abs() < 1e-6);
}

#[test]
fn f32_atan() {
    assert!((Transcendental::atan(1.0_f32) - core::f32::consts::FRAC_PI_4).abs() < 1e-6);
}

#[test]
fn f32_atan2() {
    let pi = core::f32::consts::PI;
    assert!((Transcendental::atan2(1.0_f32, 1.0) - pi / 4.0).abs() < 1e-6);
    assert!((Transcendental::atan2(0.0_f32, -1.0) - pi).abs() < 1e-6);
}

#[test]
fn f32_sinh() {
    assert!((Transcendental::sinh(0.0_f32)).abs() < 1e-6);
    assert!((Transcendental::sinh(1.0_f32) - 1.0_f32.sinh()).abs() < 1e-6);
}

#[test]
fn f32_cosh() {
    assert!((Transcendental::cosh(0.0_f32) - 1.0).abs() < 1e-6);
    assert!((Transcendental::cosh(1.0_f32) - 1.0_f32.cosh()).abs() < 1e-6);
}

#[test]
fn f32_tanh() {
    assert!((Transcendental::tanh(0.0_f32)).abs() < 1e-6);
    assert!((Transcendental::tanh(1.0_f32) - 1.0_f32.tanh()).abs() < 1e-6);
}

#[test]
fn f32_asinh() {
    assert!((Transcendental::asinh(0.0_f32)).abs() < 1e-6);
    assert!((Transcendental::asinh(1.0_f32) - 1.0_f32.asinh()).abs() < 1e-6);
}

#[test]
fn f32_acosh() {
    assert!((Transcendental::acosh(1.0_f32)).abs() < 1e-6);
    assert!((Transcendental::acosh(2.0_f32) - 2.0_f32.acosh()).abs() < 1e-6);
}

#[test]
fn f32_atanh() {
    assert!((Transcendental::atanh(0.0_f32)).abs() < 1e-6);
    assert!((Transcendental::atanh(0.5_f32) - 0.5_f32.atanh()).abs() < 1e-6);
}
