use qtty_core::*;

// Use Length as the test dimension.
type TestDim = Length;

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum HalfTestUnit {}
impl Unit for HalfTestUnit {
    const RATIO: f64 = 0.5;
    type Dim = TestDim;
    const SYMBOL: &'static str = "htu";
}

type TU = Quantity<TestUnit>;
type Dtu = Quantity<DoubleTestUnit>;

#[test]
fn quantity_new_and_value() {
    let q = TU::new(42.0);
    assert_eq!(q.value(), 42.0);
}

#[test]
fn quantity_nan_constant() {
    assert!(TU::NAN.value().is_nan());
}

#[test]
fn quantity_abs() {
    assert_eq!(TU::new(-5.0).abs().value(), 5.0);
    assert_eq!(TU::new(5.0).abs().value(), 5.0);
    assert_eq!(TU::new(0.0).abs().value(), 0.0);
}

#[test]
fn quantity_ceil() {
    assert_eq!(TU::new(3.2).ceil().value(), 4.0);
    assert_eq!(TU::new(-3.2).ceil().value(), -3.0);
    assert_eq!(TU::new(5.0).ceil().value(), 5.0);
}

#[test]
fn quantity_from_f64() {
    let q: TU = 123.456.into();
    assert_eq!(q.value(), 123.456);
}

#[test]
fn quantity_conversion_to_same_unit() {
    let q = TU::new(10.0);
    let converted = q.to::<TestUnit>();
    assert_eq!(converted.value(), 10.0);
}

#[test]
fn quantity_conversion_to_different_unit() {
    let q = TU::new(10.0);
    let converted = q.to::<DoubleTestUnit>();
    assert!((converted.value() - 5.0).abs() < 1e-12);
}

#[test]
fn quantity_conversion_roundtrip() {
    let original = TU::new(100.0);
    let converted = original.to::<DoubleTestUnit>();
    let back = converted.to::<TestUnit>();
    assert!((back.value() - original.value()).abs() < 1e-12);
}

#[test]
fn const_add() {
    let a = TU::new(3.0);
    let b = TU::new(7.0);
    assert_eq!(a.const_add(b).value(), 10.0);
}

#[test]
fn const_sub() {
    let a = TU::new(10.0);
    let b = TU::new(3.0);
    assert_eq!(a.const_sub(b).value(), 7.0);
}

#[test]
fn const_mul() {
    let a = TU::new(4.0);
    let b = 5.0;
    assert_eq!(a.const_mul(b).value(), 20.0);
}

#[test]
fn const_div() {
    let a = TU::new(20.0);
    let b = 4.0;
    assert_eq!(a.const_div(b).value(), 5.0);
}

#[test]
fn const_min() {
    let a = TU::new(5.0);
    let b = TU::new(3.0);
    assert_eq!(a.min(b).value(), 3.0);
    assert_eq!(b.min(a).value(), 3.0);
}

#[test]
fn operator_add() {
    let a = TU::new(3.0);
    let b = TU::new(7.0);
    assert_eq!((a + b).value(), 10.0);
}

#[test]
fn operator_sub() {
    let a = TU::new(10.0);
    let b = TU::new(3.0);
    assert_eq!((a - b).value(), 7.0);
}

#[test]
fn operator_mul_by_f64() {
    let q = TU::new(5.0);
    assert_eq!((q * 3.0).value(), 15.0);
    assert_eq!((3.0 * q).value(), 15.0);
}

#[test]
fn operator_div_by_f64() {
    let q = TU::new(15.0);
    assert_eq!((q / 3.0).value(), 5.0);
}

#[test]
fn operator_neg() {
    let q = TU::new(5.0);
    assert_eq!((-q).value(), -5.0);
    assert_eq!((-(-q)).value(), 5.0);
}

#[test]
fn operator_rem() {
    let q = TU::new(10.0);
    assert_eq!((q % 3.0).value(), 1.0);
}

#[test]
fn operator_add_assign() {
    let mut q = TU::new(5.0);
    q += TU::new(3.0);
    assert_eq!(q.value(), 8.0);
}

#[test]
fn operator_sub_assign() {
    let mut q = TU::new(10.0);
    q -= TU::new(3.0);
    assert_eq!(q.value(), 7.0);
}

#[test]
fn operator_div_assign() {
    let mut q = TU::new(20.0);
    q /= TU::new(4.0);
    assert_eq!(q.value(), 5.0);
}

#[test]
fn partial_eq_f64() {
    let q = TU::new(5.0);
    assert!(q == 5.0);
    assert!(!(q == 4.0));
}

#[test]
fn division_creates_per_type() {
    let num = TU::new(100.0);
    let den = Dtu::new(20.0);
    let ratio: Quantity<Per<TestUnit, DoubleTestUnit>> = num / den;
    assert!((ratio.value() - 5.0).abs() < 1e-12);
}

#[test]
fn per_ratio_conversion() {
    let v1: Quantity<Per<DoubleTestUnit, TestUnit>> = Quantity::new(10.0);
    let v2: Quantity<Per<TestUnit, TestUnit>> = v1.to();
    assert!((v2.value() - 20.0).abs() < 1e-12);
}

#[test]
fn per_multiplication_recovers_numerator() {
    let rate: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(5.0);
    let time = Dtu::new(4.0);
    let result: TU = (rate * time).to();
    assert!((result.value() - 20.0).abs() < 1e-12);
}

#[test]
fn per_multiplication_commutative() {
    let rate: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(5.0);
    let time = Dtu::new(4.0);
    let result1: TU = (rate * time).to();
    let result2: TU = (time * rate).to();
    assert!((result1.value() - result2.value()).abs() < 1e-12);
}

#[test]
fn simplify_per_u_u_to_unitless() {
    let ratio: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(1.23456);
    let unitless: Quantity<Unitless> = ratio.simplify();
    assert!((unitless.value() - 1.23456).abs() < 1e-12);
}

#[test]
fn simplify_per_n_per_n_d_to_d() {
    let q: Quantity<Per<TestUnit, Per<TestUnit, DoubleTestUnit>>> = Quantity::new(7.5);
    let simplified: Dtu = q.simplify();
    assert!((simplified.value() - 7.5).abs() < 1e-12);
}

#[test]
fn per_u_u_asin() {
    let ratio: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(0.5);
    let result = ratio.asin();
    assert!((result - 0.5_f64.asin()).abs() < 1e-12);
}

#[test]
fn per_u_u_asin_boundary_values() {
    let one: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(1.0);
    assert!((one.asin() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);

    let neg_one: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(-1.0);
    assert!((neg_one.asin() - (-core::f64::consts::FRAC_PI_2)).abs() < 1e-12);

    let zero: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(0.0);
    assert!((zero.asin() - 0.0).abs() < 1e-12);
}

#[test]
fn display_simple_quantity() {
    let q = TU::new(42.5);
    let s = format!("{} {}", q.value(), TestUnit::SYMBOL);
    assert_eq!(s, "42.5 tu");
}

#[test]
fn display_per_quantity() {
    let q: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(2.5);
    let s = format!("{}", q);
    assert_eq!(s, "2.5 tu/dtu");
}

#[test]
fn display_negative_value() {
    let q = TU::new(-99.9);
    let s = format!("{} {}", q.value(), TestUnit::SYMBOL);
    assert_eq!(s, "-99.9 tu");
}

#[test]
fn edge_case_zero() {
    let zero = TU::new(0.0);
    assert_eq!(zero.value(), 0.0);
    assert_eq!((-zero).value(), 0.0);
    assert_eq!(zero.abs().value(), 0.0);
}

#[test]
fn edge_case_negative_values() {
    let neg = TU::new(-10.0);
    let pos = TU::new(5.0);

    assert_eq!((neg + pos).value(), -5.0);
    assert_eq!((neg - pos).value(), -15.0);
    assert_eq!((neg * 2.0).value(), -20.0);
    assert_eq!(neg.abs().value(), 10.0);
}

#[test]
fn edge_case_large_values() {
    let large = TU::new(1e100);
    let small = TU::new(1e-100);
    assert_eq!(large.value(), 1e100);
    assert_eq!(small.value(), 1e-100);
}

#[test]
fn edge_case_infinity() {
    let inf = TU::new(f64::INFINITY);
    let neg_inf = TU::new(f64::NEG_INFINITY);

    assert!(inf.value().is_infinite());
    assert!(neg_inf.value().is_infinite());
    assert_eq!(inf.value().signum(), 1.0);
    assert_eq!(neg_inf.value().signum(), -1.0);
}

// ─────────────────────────────────────────────────────────────────────────
// Additional method coverage tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_is_nan() {
    let nan = TU::new(f64::NAN);
    let normal = TU::new(42.0);
    assert!(nan.is_nan());
    assert!(!normal.is_nan());
}

#[test]
fn test_is_infinite() {
    let inf = TU::new(f64::INFINITY);
    let neg_inf = TU::new(f64::NEG_INFINITY);
    let normal = TU::new(42.0);
    assert!(inf.is_infinite());
    assert!(neg_inf.is_infinite());
    assert!(!normal.is_infinite());
}

#[test]
fn test_is_finite() {
    let normal = TU::new(42.0);
    let inf = TU::new(f64::INFINITY);
    let nan = TU::new(f64::NAN);
    assert!(normal.is_finite());
    assert!(!inf.is_finite());
    assert!(!nan.is_finite());
}

#[test]
fn test_signum() {
    let pos = TU::new(42.0);
    let neg = TU::new(-42.0);
    assert_eq!(pos.signum(), 1.0);
    assert_eq!(neg.signum(), -1.0);
}

#[test]
fn test_sqrt() {
    let q = TU::new(16.0);
    let sqrt_q = q.sqrt();
    assert!((sqrt_q.value() - 4.0).abs() < 1e-12);
}

#[test]
fn test_cast() {
    let q = TU::new(42.5);
    let q_f32: Quantity<TestUnit, f32> = q.cast();
    assert!((q_f32.value() - 42.5).abs() < 0.01);
}

#[test]
fn test_max() {
    let a = TU::new(5.0);
    let b = TU::new(10.0);
    assert_eq!(a.max(b).value(), 10.0);
    assert_eq!(b.max(a).value(), 10.0);
}

#[test]
fn test_acos() {
    let ratio: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(0.5);
    let result = ratio.acos();
    assert!((result - 0.5_f64.acos()).abs() < 1e-12);
}

#[test]
fn test_atan() {
    let ratio: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(1.0);
    let result = ratio.atan();
    assert!((result - core::f64::consts::FRAC_PI_4).abs() < 1e-12);
}

#[test]
fn test_to_lossy() {
    use qtty_core::units::length::{Kilometer, Meter};
    let km: Quantity<Kilometer, i32> = Quantity::new(5);
    let m: Quantity<Meter, i32> = km.to_lossy();
    assert_eq!(m.value(), 5000);
}

#[test]
fn test_value_ref() {
    let q = TU::new(42.5);
    assert_eq!(*q.value_ref(), 42.5);
}

#[test]
fn test_zero_one() {
    let zero = TU::zero();
    let one = TU::one();
    assert_eq!(zero.value(), 0.0);
    assert_eq!(one.value(), 1.0);
}

#[test]
fn test_min_const() {
    let a = TU::new(3.0);
    let b = TU::new(7.0);
    assert_eq!(a.min_const(b).value(), 3.0);
}

#[test]
fn test_max_const() {
    let a = TU::new(3.0);
    let b = TU::new(7.0);
    assert_eq!(a.max_const(b).value(), 7.0);
}

#[test]
fn test_to_const() {
    let q = TU::new(10.0);
    let converted: Dtu = q.to_const();
    assert!((converted.value() - 5.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────
// Comparison operator tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn partial_ord_same_unit() {
    let a = TU::new(3.0);
    let b = TU::new(7.0);
    assert!(a < b);
    assert!(a <= b);
    assert!(b > a);
    assert!(b >= a);
    assert!(a <= a);
    assert!(a >= a);
}

#[test]
fn partial_ord_scalar_f64() {
    let q = TU::new(5.0);
    assert!(q > 3.0);
    assert!(q >= 5.0);
    assert!(q < 10.0);
    assert!(q <= 5.0);
    assert!(q >= 4.99);
    assert!(q <= 5.01);
}

#[test]
fn partial_ord_scalar_negative() {
    let q = TU::new(-2.0);
    assert!(q < 0.0);
    assert!(q > -5.0);
    assert!(q <= -2.0);
    assert!(q >= -2.0);
}

#[test]
fn partial_ord_nan_returns_none() {
    let nan = TU::new(f64::NAN);
    let normal = TU::new(5.0);
    // NaN comparisons are unordered
    assert!(nan.partial_cmp(&normal).is_none());
    assert!(!(nan == normal));
    // NaN vs scalar
    assert!(nan.partial_cmp(&5.0).is_none());
    assert!(!(nan == 5.0));
}

#[test]
fn eq_ord_integers() {
    use core::cmp::Ordering;
    let a: Quantity<TestUnit, i32> = Quantity::new(3);
    let b: Quantity<TestUnit, i32> = Quantity::new(7);
    let c: Quantity<TestUnit, i32> = Quantity::new(3);

    // Eq
    assert_eq!(a, c);
    assert_ne!(a, b);

    // Ord
    assert_eq!(a.cmp(&b), Ordering::Less);
    assert_eq!(b.cmp(&a), Ordering::Greater);
    assert_eq!(a.cmp(&c), Ordering::Equal);
}

#[test]
fn ord_integers_sort() {
    let mut quantities: Vec<Quantity<TestUnit, i32>> = vec![
        Quantity::new(5),
        Quantity::new(1),
        Quantity::new(3),
        Quantity::new(2),
        Quantity::new(4),
    ];
    quantities.sort();
    let values: Vec<i32> = quantities.iter().map(|q| q.value()).collect();
    assert_eq!(values, vec![1, 2, 3, 4, 5]);
}

#[test]
fn ord_integers_min_max() {
    let a: Quantity<TestUnit, i32> = Quantity::new(3);
    let b: Quantity<TestUnit, i32> = Quantity::new(7);
    assert_eq!(a.min(b).value(), 3);
    assert_eq!(a.max(b).value(), 7);
}

#[test]
fn ord_integers_btreemap() {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<Quantity<TestUnit, i32>, &str> = BTreeMap::new();
    map.insert(Quantity::new(3), "three");
    map.insert(Quantity::new(1), "one");
    map.insert(Quantity::new(2), "two");
    let keys: Vec<i32> = map.keys().map(|q| q.value()).collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

// ─────────────────────────────────────────────────────────────────────────
// Cross-unit comparison tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn cross_unit_eq_method() {
    // DoubleTestUnit has ratio 2.0, so 5 dtu = 10 tu
    let dtu = Dtu::new(5.0);
    let tu = TU::new(10.0);
    assert!(dtu.eq_unit(&tu));
    assert!(tu.eq_unit(&dtu));
}

#[test]
fn cross_unit_cmp_method() {
    use core::cmp::Ordering;
    let dtu = Dtu::new(5.0); // = 10 tu
    let tu_less = TU::new(3.0);
    let tu_more = TU::new(20.0);
    let tu_eq = TU::new(10.0);

    assert_eq!(dtu.cmp_unit(&tu_less), Some(Ordering::Greater));
    assert_eq!(dtu.cmp_unit(&tu_more), Some(Ordering::Less));
    assert_eq!(dtu.cmp_unit(&tu_eq), Some(Ordering::Equal));
}

#[test]
fn cross_unit_operators_via_macro() {
    use qtty_core::units::length::{Kilometer, Meter};

    let km: Quantity<Kilometer> = Quantity::new(1.0);
    let m: Quantity<Meter> = Quantity::new(500.0);

    // 1 km > 500 m
    assert!(km > m);
    assert!(km >= m);
    assert!(m < km);
    assert!(m <= km);
    assert!(km != m);

    // 1 km == 1000 m
    let m_eq: Quantity<Meter> = Quantity::new(1000.0);
    assert!(km == m_eq);
    assert!(km >= m_eq);
    assert!(km <= m_eq);
}

#[test]
fn cross_unit_operators_f32() {
    use qtty_core::units::length::{Kilometer, Meter};

    let km: Quantity<Kilometer, f32> = Quantity::new(2.0_f32);
    let m: Quantity<Meter, f32> = Quantity::new(1500.0_f32);

    assert!(km > m);
    assert!(m < km);
}

#[test]
fn cross_unit_nan_comparison() {
    use qtty_core::units::length::{Kilometer, Meter};

    let km_nan: Quantity<Kilometer> = Quantity::new(f64::NAN);
    let m: Quantity<Meter> = Quantity::new(1000.0);

    assert!(!(km_nan == m));
    assert!(km_nan.partial_cmp(&m).is_none());
}
