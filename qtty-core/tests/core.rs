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
    assert_eq!(a.add(b).value(), 10.0);
}

#[test]
fn const_sub() {
    let a = TU::new(10.0);
    let b = TU::new(3.0);
    assert_eq!(a.sub(b).value(), 7.0);
}

#[test]
fn const_mul() {
    let a = TU::new(4.0);
    let b = TU::new(5.0);
    assert_eq!(Quantity::mul(&a, b).value(), 20.0);
}

#[test]
fn const_div() {
    let a = TU::new(20.0);
    let b = TU::new(4.0);
    assert_eq!(Quantity::div(&a, b).value(), 5.0);
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
    let result: TU = rate * time;
    assert!((result.value() - 20.0).abs() < 1e-12);
}

#[test]
fn per_multiplication_commutative() {
    let rate: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(5.0);
    let time = Dtu::new(4.0);
    let result1: TU = rate * time;
    let result2: TU = time * rate;
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
