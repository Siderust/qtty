//! Integration tests for integer scalar support.
//!
//! Tests that Quantity<U, iNN> works correctly for i8, i16, i32, i64, i128
//! with arithmetic, Display, lossy conversion, and derived units (Per).

use qtty_core::length::{Kilometer, Meter};
use qtty_core::scalar::{Exact, IntegerScalar};
use qtty_core::time::Second;
use qtty_core::{Per, Quantity, QuantityI32, QuantityI64, Simplify, Unitless};

// ─────────────────────────────────────────────────────────────────────────────
// Basic construction and value access
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_quantity_new_and_value() {
    let m: Quantity<Meter, i32> = Quantity::new(42);
    assert_eq!(m.value(), 42);
}

#[test]
fn test_i64_quantity_new_and_value() {
    let m: Quantity<Meter, i64> = Quantity::new(1_000_000_000);
    assert_eq!(m.value(), 1_000_000_000);
}

#[test]
fn test_i8_quantity_new_and_value() {
    let m: Quantity<Meter, i8> = Quantity::new(100);
    assert_eq!(m.value(), 100);
}

#[test]
fn test_i16_quantity_new_and_value() {
    let m: Quantity<Meter, i16> = Quantity::new(30_000);
    assert_eq!(m.value(), 30_000);
}

#[test]
fn test_i128_quantity_new_and_value() {
    let m: Quantity<Meter, i128> = Quantity::new(i128::MAX);
    assert_eq!(m.value(), i128::MAX);
}

#[test]
fn test_zero_and_one() {
    assert_eq!(Quantity::<Meter, i32>::zero().value(), 0);
    assert_eq!(Quantity::<Meter, i32>::one().value(), 1);
    assert_eq!(Quantity::<Meter, i64>::zero().value(), 0_i64);
    assert_eq!(Quantity::<Meter, i64>::one().value(), 1_i64);
}

// ─────────────────────────────────────────────────────────────────────────────
// Arithmetic operations
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_add_sub() {
    let a = Quantity::<Meter, i32>::new(10);
    let b = Quantity::<Meter, i32>::new(3);
    assert_eq!((a + b).value(), 13);
    assert_eq!((a - b).value(), 7);
}

#[test]
fn test_i32_mul_div_scalar() {
    let a = Quantity::<Meter, i32>::new(10);
    assert_eq!((a * 3).value(), 30);
    assert_eq!((a / 3).value(), 3); // integer truncation: 10/3 = 3
}

#[test]
fn test_i32_neg() {
    let a = Quantity::<Meter, i32>::new(5);
    assert_eq!((-a).value(), -5);
}

#[test]
fn test_i32_abs() {
    let a = Quantity::<Meter, i32>::new(-7);
    assert_eq!(a.abs().value(), 7);
}

#[test]
fn test_i32_min_max() {
    let a = Quantity::<Meter, i32>::new(3);
    let b = Quantity::<Meter, i32>::new(7);
    assert_eq!(a.min(b).value(), 3);
    assert_eq!(a.max(b).value(), 7);
}

#[test]
fn test_i32_add_assign() {
    let mut a = Quantity::<Meter, i32>::new(10);
    a += Quantity::new(5);
    assert_eq!(a.value(), 15);
}

#[test]
fn test_i32_sub_assign() {
    let mut a = Quantity::<Meter, i32>::new(10);
    a -= Quantity::new(3);
    assert_eq!(a.value(), 7);
}

// ─────────────────────────────────────────────────────────────────────────────
// Commutative multiplication: scalar * Quantity
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_commutative_mul() {
    let a = Quantity::<Meter, i32>::new(5);
    assert_eq!((3_i32 * a).value(), 15);
    assert_eq!((a * 3_i32).value(), 15);
}

#[test]
fn test_i64_commutative_mul() {
    let a = Quantity::<Meter, i64>::new(5);
    assert_eq!((3_i64 * a).value(), 15);
}

#[test]
fn test_i8_commutative_mul() {
    let a = Quantity::<Meter, i8>::new(5);
    assert_eq!((3_i8 * a).value(), 15);
}

// ─────────────────────────────────────────────────────────────────────────────
// Rem (modulo) for integers
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_rem() {
    let a = Quantity::<Meter, i32>::new(17);
    assert_eq!((a % 5).value(), 2);
}

#[test]
fn test_i64_rem() {
    let a = Quantity::<Meter, i64>::new(17);
    assert_eq!((a % 5_i64).value(), 2);
}

// ─────────────────────────────────────────────────────────────────────────────
// PartialEq and From
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_partial_eq_scalar() {
    let a = Quantity::<Meter, i32>::new(42);
    assert_eq!(a, 42);
}

#[test]
fn test_i32_from_scalar() {
    let a: Quantity<Meter, i32> = 42.into();
    assert_eq!(a.value(), 42);
}

// ─────────────────────────────────────────────────────────────────────────────
// Display
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_display() {
    let m = Quantity::<Meter, i32>::new(100);
    assert_eq!(format!("{}", m), "100 m");
}

#[test]
fn test_i64_display() {
    let km = Quantity::<Kilometer, i64>::new(42);
    assert_eq!(format!("{}", km), "42 Km");
}

// ─────────────────────────────────────────────────────────────────────────────
// Per (derived units)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_per_division() {
    let distance = Quantity::<Meter, i32>::new(100);
    let time = Quantity::<Second, i32>::new(10);
    let velocity: Quantity<Per<Meter, Second>, i32> = distance / time;
    assert_eq!(velocity.value(), 10);
}

#[test]
fn test_i32_per_multiplication() {
    let velocity = Quantity::<Per<Meter, Second>, i32>::new(10);
    let time = Quantity::<Second, i32>::new(5);
    let distance: Quantity<Meter, i32> = velocity * time;
    assert_eq!(distance.value(), 50);
}

#[test]
fn test_i32_per_display() {
    let v = Quantity::<Per<Meter, Second>, i32>::new(10);
    assert_eq!(format!("{}", v), "10 m/s");
}

// ─────────────────────────────────────────────────────────────────────────────
// Simplify (Per<U,U> -> Unitless)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_simplify() {
    let a = Quantity::<Meter, i32>::new(10);
    let b = Quantity::<Meter, i32>::new(2);
    let ratio = a / b;
    let unitless: Quantity<Unitless, i32> = ratio.simplify();
    assert_eq!(unitless.value(), 5);
}

// ─────────────────────────────────────────────────────────────────────────────
// Lossy conversion (to_lossy)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_to_lossy_meters_to_km() {
    let m = Quantity::<Meter, i32>::new(1500);
    let km: Quantity<Kilometer, i32> = m.to_lossy();
    assert_eq!(km.value(), 1); // 1500m = 1.5km, truncated to 1
}

#[test]
fn test_i32_to_lossy_km_to_meters() {
    let km = Quantity::<Kilometer, i32>::new(2);
    let m: Quantity<Meter, i32> = km.to_lossy();
    assert_eq!(m.value(), 2000);
}

#[test]
fn test_i64_to_lossy_meters_to_km() {
    let m = Quantity::<Meter, i64>::new(5_500_000);
    let km: Quantity<Kilometer, i64> = m.to_lossy();
    assert_eq!(km.value(), 5500);
}

#[test]
fn test_i32_to_lossy_exact_conversion() {
    // 3000m = 3km exactly
    let m = Quantity::<Meter, i32>::new(3000);
    let km: Quantity<Kilometer, i32> = m.to_lossy();
    assert_eq!(km.value(), 3);
}

// ─────────────────────────────────────────────────────────────────────────────
// Const methods
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_const_add() {
    let a = Quantity::<Meter, i32>::new(10);
    let b = Quantity::<Meter, i32>::new(5);
    assert_eq!(a.const_add(b).value(), 15);
}

#[test]
fn test_i32_const_sub() {
    let a = Quantity::<Meter, i32>::new(10);
    let b = Quantity::<Meter, i32>::new(3);
    assert_eq!(a.const_sub(b).value(), 7);
}

#[test]
fn test_i32_const_mul() {
    let a = Quantity::<Meter, i32>::new(5);
    assert_eq!(a.const_mul(3).value(), 15);
}

#[test]
fn test_i32_const_div() {
    let a = Quantity::<Meter, i32>::new(15);
    assert_eq!(a.const_div(3).value(), 5);
}

#[test]
fn test_i32_min_max_const() {
    let a = Quantity::<Meter, i32>::new(3);
    let b = Quantity::<Meter, i32>::new(7);
    assert_eq!(a.min_const(b).value(), 3);
    assert_eq!(a.max_const(b).value(), 7);
}

#[test]
fn test_i64_const_methods() {
    let a = Quantity::<Meter, i64>::new(100);
    let b = Quantity::<Meter, i64>::new(200);
    assert_eq!(a.const_add(b).value(), 300);
    assert_eq!(a.const_mul(3).value(), 300);
}

// ─────────────────────────────────────────────────────────────────────────────
// Type aliases
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_quantity_i32_alias() {
    let m: QuantityI32<Meter> = QuantityI32::new(42);
    assert_eq!(m.value(), 42);
}

#[test]
fn test_quantity_i64_alias() {
    let m: QuantityI64<Meter> = QuantityI64::new(42);
    assert_eq!(m.value(), 42);
}

// ─────────────────────────────────────────────────────────────────────────────
// Trait bounds verification
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_integer_scalar_is_exact() {
    fn accepts_exact<S: Exact>(_: S) {}
    accepts_exact(42_i32);
    accepts_exact(42_i64);
    accepts_exact(42_i8);
    accepts_exact(42_i16);
    accepts_exact(42_i128);
}

#[test]
fn test_integer_scalar_marker() {
    fn accepts_integer<S: IntegerScalar>(_: S) {}
    accepts_integer(42_i32);
    accepts_integer(42_i64);
    accepts_integer(42_i8);
    accepts_integer(42_i16);
    accepts_integer(42_i128);
}

// ─────────────────────────────────────────────────────────────────────────────
// Edge cases
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i32_integer_division_truncation() {
    // Verify that integer division truncates toward zero
    let a = Quantity::<Meter, i32>::new(10);
    assert_eq!((a / 3).value(), 3); // 10/3 = 3
    assert_eq!((a / 4).value(), 2); // 10/4 = 2
    assert_eq!((-a / 3).value(), -3); // -10/3 = -3
}

#[test]
fn test_i32_negative_quantities() {
    let a = Quantity::<Meter, i32>::new(-10);
    let b = Quantity::<Meter, i32>::new(3);
    assert_eq!((a + b).value(), -7);
    assert_eq!((a - b).value(), -13);
    assert_eq!(a.abs().value(), 10);
}

#[test]
fn test_i32_div_assign() {
    let mut a = Quantity::<Meter, i32>::new(10);
    let b = Quantity::<Meter, i32>::new(5);
    a /= b;
    assert_eq!(a.value(), 2);
}
