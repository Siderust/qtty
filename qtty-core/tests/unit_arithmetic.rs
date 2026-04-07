// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Tests for the stable unit arithmetic layer (UnitDiv / UnitMul).

use qtty_core::units::length::{Kilometer, Meter};
use qtty_core::units::mass::Kilogram;
use qtty_core::units::time::Second;
use qtty_core::*;

// ─────────────────────────────────────────────────────────────────────────────
// U / U → Unitless
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn same_unit_division_gives_unitless_meter() {
    let a = Quantity::<Meter>::new(10.0);
    let b = Quantity::<Meter>::new(4.0);
    let ratio: Quantity<Unitless> = a / b;
    assert!((ratio.value() - 2.5).abs() < 1e-12);
}

#[test]
fn same_unit_division_gives_unitless_second() {
    let a = Quantity::<Second>::new(100.0);
    let b = Quantity::<Second>::new(50.0);
    let ratio: Quantity<Unitless> = a / b;
    assert!((ratio.value() - 2.0).abs() < 1e-12);
}

#[test]
fn same_unit_division_gives_unitless_kilogram() {
    let a = Quantity::<Kilogram>::new(6.0);
    let b = Quantity::<Kilogram>::new(3.0);
    let ratio: Quantity<Unitless> = a / b;
    assert!((ratio.value() - 2.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// N / (N / D) → D
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn numerator_divided_by_per_gives_denominator() {
    let distance = Quantity::<Meter>::new(10.0);
    let velocity: Quantity<Per<Meter, Second>> = Quantity::new(2.0); // 2 m/s
    let time: Quantity<Second> = distance / velocity;
    assert!((time.value() - 5.0).abs() < 1e-12);
}

#[test]
fn kilometer_divided_by_velocity_gives_time() {
    let distance = Quantity::<Kilometer>::new(100.0);
    let velocity: Quantity<Per<Kilometer, Second>> = Quantity::new(50.0);
    let time: Quantity<Second> = distance / velocity;
    assert!((time.value() - 2.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// N / D → Per<N, D>  (cross-unit division)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn cross_unit_division_meter_second() {
    let d = Quantity::<Meter>::new(100.0);
    let t = Quantity::<Second>::new(10.0);
    let v: Quantity<Per<Meter, Second>> = d / t;
    assert!((v.value() - 10.0).abs() < 1e-12);
}

#[test]
fn cross_unit_division_kilometer_second() {
    let d = Quantity::<Kilometer>::new(50.0);
    let t = Quantity::<Second>::new(10.0);
    let v: Quantity<Per<Kilometer, Second>> = d / t;
    assert!((v.value() - 5.0).abs() < 1e-12);
}

#[test]
fn cross_unit_division_kilogram_meter() {
    let mass = Quantity::<Kilogram>::new(10.0);
    let length = Quantity::<Meter>::new(2.0);
    let density: Quantity<Per<Kilogram, Meter>> = mass / length;
    assert!((density.value() - 5.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// Per<N, D> * D → N  and  D * Per<N, D> → N
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn per_times_denominator_gives_numerator() {
    let velocity: Quantity<Per<Meter, Second>> = Quantity::new(5.0);
    let time = Quantity::<Second>::new(10.0);
    let distance: Quantity<Meter> = velocity * time;
    assert!((distance.value() - 50.0).abs() < 1e-12);
}

#[test]
fn denominator_times_per_gives_numerator() {
    let velocity: Quantity<Per<Meter, Second>> = Quantity::new(5.0);
    let time = Quantity::<Second>::new(10.0);
    let distance: Quantity<Meter> = time * velocity;
    assert!((distance.value() - 50.0).abs() < 1e-12);
}

#[test]
fn per_times_denominator_commutative() {
    let rate: Quantity<Per<Kilometer, Second>> = Quantity::new(3.0);
    let t = Quantity::<Second>::new(7.0);
    let d1: Quantity<Kilometer> = rate * t;
    let d2: Quantity<Kilometer> = t * rate;
    assert!((d1.value() - d2.value()).abs() < 1e-12);
    assert!((d1.value() - 21.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// A * B → Prod<A, B>  (plain multiplication fallback)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn plain_multiplication_meter_meter() {
    let a = Quantity::<Meter>::new(4.0);
    let b = Quantity::<Meter>::new(5.0);
    let product: Quantity<Prod<Meter, Meter>> = a * b;
    assert!((product.value() - 20.0).abs() < 1e-12);
}

#[test]
fn plain_multiplication_meter_second() {
    let a = Quantity::<Meter>::new(3.0);
    let b = Quantity::<Second>::new(4.0);
    let product: Quantity<Prod<Meter, Second>> = a * b;
    assert!((product.value() - 12.0).abs() < 1e-12);
}

#[test]
fn plain_multiplication_converts_to_named_unit() {
    use qtty_core::units::area::SquareMeter;
    let a = Quantity::<Meter>::new(4.0);
    let b = Quantity::<Meter>::new(5.0);
    let area: Quantity<SquareMeter> = (a * b).to();
    assert!((area.value() - 20.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// Custom unit tests: downstream users restore arithmetic via macros
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum CustomLengthA {}
impl Unit for CustomLengthA {
    const RATIO: f64 = 1.0;
    type Dim = Length;
    const SYMBOL: &'static str = "cla";
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum CustomLengthB {}
impl Unit for CustomLengthB {
    const RATIO: f64 = 10.0;
    type Dim = Length;
    const SYMBOL: &'static str = "clb";
}

// Register custom units for arithmetic.
qtty_core::impl_unit_arithmetic_pairs!(CustomLengthA, CustomLengthB);

#[test]
fn custom_unit_same_division_gives_unitless() {
    let a = Quantity::<CustomLengthA>::new(10.0);
    let b = Quantity::<CustomLengthA>::new(5.0);
    let ratio: Quantity<Unitless> = a / b;
    assert!((ratio.value() - 2.0).abs() < 1e-12);
}

#[test]
fn custom_unit_cross_division() {
    let a = Quantity::<CustomLengthA>::new(100.0);
    let b = Quantity::<CustomLengthB>::new(10.0);
    let ratio: Quantity<Per<CustomLengthA, CustomLengthB>> = a / b;
    assert!((ratio.value() - 10.0).abs() < 1e-12);
}

#[test]
fn custom_unit_cross_multiplication() {
    let a = Quantity::<CustomLengthA>::new(3.0);
    let b = Quantity::<CustomLengthB>::new(4.0);
    let product: Quantity<Prod<CustomLengthA, CustomLengthB>> = a * b;
    assert!((product.value() - 12.0).abs() < 1e-12);
}

#[test]
fn custom_unit_self_multiplication() {
    let a = Quantity::<CustomLengthA>::new(3.0);
    let b = Quantity::<CustomLengthA>::new(4.0);
    let product: Quantity<Prod<CustomLengthA, CustomLengthA>> = a * b;
    assert!((product.value() - 12.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// Trig on Unitless from same-unit ratios
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn same_unit_ratio_has_asin() {
    let ratio = Quantity::<Meter>::new(1.0) / Quantity::<Meter>::new(2.0);
    let angle = ratio.asin();
    assert!((angle - core::f64::consts::FRAC_PI_6).abs() < 1e-12);
}

#[test]
fn same_unit_ratio_has_acos() {
    let ratio = Quantity::<Meter>::new(1.0) / Quantity::<Meter>::new(2.0);
    let angle = ratio.acos();
    assert!((angle - core::f64::consts::FRAC_PI_3).abs() < 1e-12);
}

#[test]
fn same_unit_ratio_has_atan() {
    let ratio = Quantity::<Meter>::new(1.0) / Quantity::<Meter>::new(1.0);
    let angle = ratio.atan();
    assert!((angle - core::f64::consts::FRAC_PI_4).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// Compile-time type assignment checks
// ─────────────────────────────────────────────────────────────────────────────

/// Compile-time check: U / U → Unitless
fn _check_u_div_u_unitless() {
    fn assert_unitless(_: Quantity<Unitless>) {}
    assert_unitless(Quantity::<Meter>::new(1.0) / Quantity::<Meter>::new(1.0));
    assert_unitless(Quantity::<Second>::new(1.0) / Quantity::<Second>::new(1.0));
    assert_unitless(Quantity::<Kilogram>::new(1.0) / Quantity::<Kilogram>::new(1.0));
}

/// Compile-time check: N / (N / D) → D
fn _check_n_div_per_n_d() {
    fn assert_second(_: Quantity<Second>) {}
    assert_second(Quantity::<Meter>::new(1.0) / Quantity::<Per<Meter, Second>>::new(1.0));
}

/// Compile-time check: N / D → Per<N, D>
fn _check_cross_div() {
    fn assert_per_m_s(_: Quantity<Per<Meter, Second>>) {}
    assert_per_m_s(Quantity::<Meter>::new(1.0) / Quantity::<Second>::new(1.0));
}

/// Compile-time check: Per<N, D> * D → N  and  D * Per<N, D> → N
fn _check_per_mul_recovery() {
    fn assert_meter(_: Quantity<Meter>) {}
    assert_meter(Quantity::<Per<Meter, Second>>::new(1.0) * Quantity::<Second>::new(1.0));
    assert_meter(Quantity::<Second>::new(1.0) * Quantity::<Per<Meter, Second>>::new(1.0));
}

/// Compile-time check: A * B → Prod<A, B>
fn _check_prod_fallback() {
    fn assert_prod_m_m(_: Quantity<Prod<Meter, Meter>>) {}
    assert_prod_m_m(Quantity::<Meter>::new(1.0) * Quantity::<Meter>::new(1.0));

    fn assert_prod_m_s(_: Quantity<Prod<Meter, Second>>) {}
    assert_prod_m_s(Quantity::<Meter>::new(1.0) * Quantity::<Second>::new(1.0));
}
