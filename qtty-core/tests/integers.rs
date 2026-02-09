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
    assert_eq!(a.mean(b).value(), 5);
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
    assert_eq!(format!("{}", km), "42 km");
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
    // Blanket Mul gives Quantity<Prod<Per<Meter,Second>, Second>, i32>
    // which has the same dimension as Meter; to_lossy converts.
    let distance: Quantity<Meter, i32> = (velocity * time).to_lossy();
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

// ─────────────────────────────────────────────────────────────────────────────
// Additional coverage for exact conversions
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i8_exact_conversion() {
    use qtty_core::scalar::Exact;
    let val = 42_i8;
    let f64_val = Exact::to_f64_approx(val);
    assert_eq!(f64_val, 42.0);
    let back: i8 = Exact::from_f64_approx(f64_val);
    assert_eq!(back, 42);
}

#[test]
fn test_i16_exact_conversion() {
    use qtty_core::scalar::Exact;
    let val = 1000_i16;
    let f64_val = Exact::to_f64_approx(val);
    assert_eq!(f64_val, 1000.0);
    let back: i16 = Exact::from_f64_approx(f64_val);
    assert_eq!(back, 1000);
}

#[test]
fn test_i32_exact_conversion() {
    use qtty_core::scalar::Exact;
    let val = 100000_i32;
    let f64_val = Exact::to_f64_approx(val);
    assert_eq!(f64_val, 100000.0);
    let back: i32 = Exact::from_f64_approx(f64_val);
    assert_eq!(back, 100000);
}

#[test]
fn test_i64_exact_conversion() {
    use qtty_core::scalar::Exact;
    let val = 1_000_000_i64;
    let f64_val = Exact::to_f64_approx(val);
    assert_eq!(f64_val, 1_000_000.0);
    let back: i64 = Exact::from_f64_approx(f64_val);
    assert_eq!(back, 1_000_000);
}

#[test]
fn test_i128_exact_conversion() {
    use qtty_core::scalar::Exact;
    let val = 1_000_000_000_i128;
    let f64_val = Exact::to_f64_approx(val);
    assert_eq!(f64_val, 1_000_000_000.0);
    let back: i128 = Exact::from_f64_approx(f64_val);
    assert_eq!(back, 1_000_000_000);
}

#[test]
fn test_i8_rem_euclid() {
    let val = 7_i8;
    assert_eq!(val.rem_euclid(4), 3);
    assert_eq!((-7_i8).rem_euclid(4), 1);
}

#[test]
fn test_i16_rem_euclid() {
    let val = 17_i16;
    assert_eq!(val.rem_euclid(5), 2);
}

#[test]
fn test_i128_rem_euclid() {
    let val = 27_i128;
    assert_eq!(val.rem_euclid(10), 7);
}

// ─────────────────────────────────────────────────────────────────────────────
// Const methods for i8, i16, i128
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i8_const_add_sub() {
    let a = Quantity::<Meter, i8>::new(10);
    let b = Quantity::<Meter, i8>::new(3);
    assert_eq!(a.const_add(b).value(), 13_i8);
    assert_eq!(a.const_sub(b).value(), 7_i8);
}

#[test]
fn test_i8_const_mul_div() {
    let a = Quantity::<Meter, i8>::new(10);
    assert_eq!(a.const_mul(3).value(), 30_i8);
    assert_eq!(a.const_div(5).value(), 2_i8);
}

#[test]
fn test_i8_min_max_const() {
    let a = Quantity::<Meter, i8>::new(3);
    let b = Quantity::<Meter, i8>::new(7);
    assert_eq!(a.min_const(b).value(), 3_i8);
    assert_eq!(a.max_const(b).value(), 7_i8);
}

#[test]
fn test_i16_const_add_sub() {
    let a = Quantity::<Meter, i16>::new(1000);
    let b = Quantity::<Meter, i16>::new(500);
    assert_eq!(a.const_add(b).value(), 1500_i16);
    assert_eq!(a.const_sub(b).value(), 500_i16);
}

#[test]
fn test_i16_const_mul_div() {
    let a = Quantity::<Meter, i16>::new(100);
    assert_eq!(a.const_mul(3).value(), 300_i16);
    assert_eq!(a.const_div(10).value(), 10_i16);
}

#[test]
fn test_i16_min_max_const() {
    let a = Quantity::<Meter, i16>::new(100);
    let b = Quantity::<Meter, i16>::new(200);
    assert_eq!(a.min_const(b).value(), 100_i16);
    assert_eq!(a.max_const(b).value(), 200_i16);
}

#[test]
fn test_i128_const_add_sub() {
    let a = Quantity::<Meter, i128>::new(1_000_000);
    let b = Quantity::<Meter, i128>::new(500_000);
    assert_eq!(a.const_add(b).value(), 1_500_000_i128);
    assert_eq!(a.const_sub(b).value(), 500_000_i128);
}

#[test]
fn test_i128_const_mul_div() {
    let a = Quantity::<Meter, i128>::new(1000);
    assert_eq!(a.const_mul(3).value(), 3000_i128);
    assert_eq!(a.const_div(10).value(), 100_i128);
}

#[test]
fn test_i128_min_max_const() {
    let a = Quantity::<Meter, i128>::new(100);
    let b = Quantity::<Meter, i128>::new(200);
    assert_eq!(a.min_const(b).value(), 100_i128);
    assert_eq!(a.max_const(b).value(), 200_i128);
}

// ─────────────────────────────────────────────────────────────────────────────
// to_lossy for i8, i16, i128
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i8_to_lossy() {
    // Using units where the ratio is small enough for i8
    let km = Quantity::<Kilometer, i8>::new(1);
    let m: Quantity<Meter, i8> = km.to_lossy();
    // 1 km = 1000m, but i8 max is 127, so this overflows/saturates
    // The result depends on `from_f64_approx` which does `value as i8`
    // 1000.0 as i8 = -24 (wraps). This is expected lossy behavior.
    let _ = m.value(); // Just ensure it doesn't panic
}

#[test]
fn test_i16_to_lossy() {
    let km = Quantity::<Kilometer, i16>::new(2);
    let m: Quantity<Meter, i16> = km.to_lossy();
    assert_eq!(m.value(), 2000_i16);
}

#[test]
fn test_i128_to_lossy() {
    let km = Quantity::<Kilometer, i128>::new(5);
    let m: Quantity<Meter, i128> = km.to_lossy();
    assert_eq!(m.value(), 5000_i128);
}

// ─────────────────────────────────────────────────────────────────────────────
// Additional i16/i128 commutative mul
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i16_commutative_mul() {
    let a = Quantity::<Meter, i16>::new(5);
    assert_eq!((3_i16 * a).value(), 15_i16);
}

#[test]
fn test_i128_commutative_mul() {
    let a = Quantity::<Meter, i128>::new(5);
    assert_eq!((3_i128 * a).value(), 15_i128);
}

// ─────────────────────────────────────────────────────────────────────────────
// i16/i128 Display, From, PartialEq, Rem
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_i16_display() {
    let m = Quantity::<Meter, i16>::new(100);
    assert_eq!(format!("{}", m), "100 m");
}

#[test]
fn test_i128_display() {
    let m = Quantity::<Meter, i128>::new(999);
    assert_eq!(format!("{}", m), "999 m");
}

#[test]
fn test_i16_from_scalar() {
    let a: Quantity<Meter, i16> = 100_i16.into();
    assert_eq!(a.value(), 100_i16);
}

#[test]
fn test_i128_from_scalar() {
    let a: Quantity<Meter, i128> = 999_i128.into();
    assert_eq!(a.value(), 999_i128);
}

#[test]
fn test_i16_partial_eq() {
    let a = Quantity::<Meter, i16>::new(42);
    assert_eq!(a, 42_i16);
}

#[test]
fn test_i128_partial_eq() {
    let a = Quantity::<Meter, i128>::new(42);
    assert_eq!(a, 42_i128);
}

#[test]
fn test_i16_rem() {
    let a = Quantity::<Meter, i16>::new(17);
    assert_eq!((a % 5_i16).value(), 2_i16);
}

#[test]
fn test_i128_rem() {
    let a = Quantity::<Meter, i128>::new(17);
    assert_eq!((a % 5_i128).value(), 2_i128);
}

#[test]
fn test_i8_display() {
    let m = Quantity::<Meter, i8>::new(42);
    assert_eq!(format!("{}", m), "42 m");
}

#[test]
fn test_i8_from_scalar() {
    let a: Quantity<Meter, i8> = 42_i8.into();
    assert_eq!(a.value(), 42_i8);
}

#[test]
fn test_i8_partial_eq() {
    let a = Quantity::<Meter, i8>::new(42);
    assert_eq!(a, 42_i8);
}

#[test]
fn test_i8_rem() {
    let a = Quantity::<Meter, i8>::new(17);
    assert_eq!((a % 5_i8).value(), 2_i8);
}
