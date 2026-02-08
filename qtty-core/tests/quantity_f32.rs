//! Tests for Quantity<U, f32> methods (coverage for quantity.rs f32 paths).

use qtty_core::*;

#[derive(Debug)]
pub enum TestDim {}
impl Dimension for TestDim {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum TestUnit {}
impl Unit for TestUnit {
    const RATIO: f64 = 1.0;
    type Dim = TestDim;
    const SYMBOL: &'static str = "tu";
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum DoubleTestUnit {}
impl Unit for DoubleTestUnit {
    const RATIO: f64 = 2.0;
    type Dim = TestDim;
    const SYMBOL: &'static str = "dtu";
}

type TU32 = Quantity<TestUnit, f32>;

// ─────────────────────────────────────────────────────────────────────────────
// Const methods for f32
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_const_add() {
    let a = TU32::new(3.0);
    let b = TU32::new(7.0);
    assert_eq!(a.const_add(b).value(), 10.0_f32);
}

#[test]
fn f32_const_sub() {
    let a = TU32::new(10.0);
    let b = TU32::new(3.0);
    assert_eq!(a.const_sub(b).value(), 7.0_f32);
}

#[test]
fn f32_const_mul() {
    let a = TU32::new(4.0);
    assert_eq!(a.const_mul(5.0).value(), 20.0_f32);
}

#[test]
fn f32_const_div() {
    let a = TU32::new(20.0);
    assert_eq!(a.const_div(4.0).value(), 5.0_f32);
}

#[test]
fn f32_to_const() {
    let q = TU32::new(10.0);
    let converted: Quantity<DoubleTestUnit, f32> = q.to_const();
    assert!((converted.value() - 5.0).abs() < 1e-6);
}

#[test]
fn f32_min_const() {
    let a = TU32::new(3.0);
    let b = TU32::new(7.0);
    assert_eq!(a.min_const(b).value(), 3.0_f32);
    assert_eq!(b.min_const(a).value(), 3.0_f32);
}

#[test]
fn f32_max_const() {
    let a = TU32::new(3.0);
    let b = TU32::new(7.0);
    assert_eq!(a.max_const(b).value(), 7.0_f32);
    assert_eq!(b.max_const(a).value(), 7.0_f32);
}

// ─────────────────────────────────────────────────────────────────────────────
// Real-specific Quantity methods for f32
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_quantity_nan_constant() {
    assert!(TU32::NAN.value().is_nan());
}

#[test]
fn f32_quantity_infinity() {
    assert!(TU32::INFINITY.value().is_infinite());
    assert!(TU32::NEG_INFINITY.value().is_infinite());
}

#[test]
fn f32_quantity_is_nan() {
    assert!(TU32::NAN.is_nan());
    assert!(!TU32::new(42.0).is_nan());
}

#[test]
fn f32_quantity_is_infinite() {
    assert!(TU32::INFINITY.is_infinite());
    assert!(!TU32::new(42.0).is_infinite());
}

#[test]
fn f32_quantity_is_finite() {
    assert!(TU32::new(42.0).is_finite());
    assert!(!TU32::INFINITY.is_finite());
    assert!(!TU32::NAN.is_finite());
}

#[test]
fn f32_quantity_signum() {
    assert_eq!(TU32::new(42.0).signum(), 1.0_f32);
    assert_eq!(TU32::new(-42.0).signum(), -1.0_f32);
}

#[test]
fn f32_quantity_sqrt() {
    let q = TU32::new(16.0);
    assert!((q.sqrt().value() - 4.0).abs() < 1e-6);
}

#[test]
fn f32_quantity_cast() {
    let q = TU32::new(42.5);
    let q_f64: Quantity<TestUnit, f64> = q.cast();
    assert!((q_f64.value() - 42.5).abs() < 1e-6);
}

#[test]
fn f32_quantity_to() {
    let q = TU32::new(10.0);
    let converted: Quantity<DoubleTestUnit, f32> = q.to();
    assert!((converted.value() - 5.0).abs() < 1e-6);
}

#[test]
fn f32_quantity_abs() {
    assert_eq!(TU32::new(-5.0).abs().value(), 5.0_f32);
}

#[test]
fn f32_quantity_min_max() {
    let a = TU32::new(3.0);
    let b = TU32::new(7.0);
    assert_eq!(a.min(b).value(), 3.0_f32);
    assert_eq!(a.max(b).value(), 7.0_f32);
}

// ─────────────────────────────────────────────────────────────────────────────
// Commutative multiplication: f32 * Quantity
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_commutative_mul() {
    let q = TU32::new(5.0);
    assert_eq!((3.0_f32 * q).value(), 15.0_f32);
    assert_eq!((q * 3.0_f32).value(), 15.0_f32);
}

// ─────────────────────────────────────────────────────────────────────────────
// Operators: Add/Sub/Neg/Div/Rem
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_quantity_add_sub() {
    let a = TU32::new(10.0);
    let b = TU32::new(3.0);
    assert_eq!((a + b).value(), 13.0_f32);
    assert_eq!((a - b).value(), 7.0_f32);
}

#[test]
fn f32_quantity_neg() {
    assert_eq!((-TU32::new(5.0)).value(), -5.0_f32);
}

#[test]
fn f32_quantity_div_scalar() {
    assert_eq!((TU32::new(15.0) / 3.0_f32).value(), 5.0_f32);
}

#[test]
fn f32_quantity_rem() {
    assert_eq!((TU32::new(10.0) % 3.0_f32).value(), 1.0_f32);
}

#[test]
fn f32_quantity_partial_eq() {
    let q = TU32::new(42.0);
    assert!(q == 42.0_f32);
}

#[test]
fn f32_quantity_from() {
    let q: TU32 = 42.0_f32.into();
    assert_eq!(q.value(), 42.0_f32);
}

#[test]
fn f32_quantity_zero_one() {
    assert_eq!(TU32::zero().value(), 0.0_f32);
    assert_eq!(TU32::one().value(), 1.0_f32);
}

// ─────────────────────────────────────────────────────────────────────────────
// Division producing Per type with f32
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_division_creates_per() {
    let num = TU32::new(100.0);
    let den = Quantity::<DoubleTestUnit, f32>::new(20.0);
    let ratio: Quantity<Per<TestUnit, DoubleTestUnit>, f32> = num / den;
    assert!((ratio.value() - 5.0).abs() < 1e-6);
}

#[test]
fn f32_per_mul_recovers_numerator() {
    let rate: Quantity<Per<TestUnit, DoubleTestUnit>, f32> = Quantity::new(5.0);
    let den = Quantity::<DoubleTestUnit, f32>::new(4.0);
    let result: TU32 = rate * den;
    assert!((result.value() - 20.0).abs() < 1e-6);
}

#[test]
fn f32_simplify() {
    let ratio: Quantity<Per<TestUnit, TestUnit>, f32> = Quantity::new(2.5);
    let unitless: Quantity<Unitless, f32> = ratio.simplify();
    assert!((unitless.value() - 2.5).abs() < 1e-6);
}

// ─────────────────────────────────────────────────────────────────────────────
// Assign ops
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_add_assign() {
    let mut q = TU32::new(5.0);
    q += TU32::new(3.0);
    assert_eq!(q.value(), 8.0_f32);
}

#[test]
fn f32_sub_assign() {
    let mut q = TU32::new(10.0);
    q -= TU32::new(3.0);
    assert_eq!(q.value(), 7.0_f32);
}

#[test]
fn f32_div_assign() {
    let mut q = TU32::new(20.0);
    q /= TU32::new(4.0);
    assert_eq!(q.value(), 5.0_f32);
}

// ─────────────────────────────────────────────────────────────────────────────
// value_ref
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn f32_value_ref() {
    let q = TU32::new(42.5);
    assert_eq!(*q.value_ref(), 42.5_f32);
}
