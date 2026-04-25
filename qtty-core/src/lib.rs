// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Core type system for strongly typed physical quantities.
//!
//! `qtty-core` provides a minimal, zero-cost units model:
//!
//! - A *unit* is a zero-sized marker type implementing [`Unit`].
//! - A value tagged with a unit is a [`Quantity<U, S>`], where `S` is the scalar
//!   type (defaults to `f64`). Supported scalars include `f64`, `f32`, signed
//!   integers (`i8`–`i128`), and optionally `Rational64`.
//! - Conversion is an explicit, type-checked scaling via [`Quantity::to`] (for
//!   [`Real`] scalars) or [`Quantity::to_lossy`] (for [`Exact`] scalars).
//! - Derived units like velocity are expressed as [`Per<N, D>`] (e.g. `Meter/Second`).
//!
//! Most users should depend on `qtty` (the facade crate) unless they need direct access to these primitives.
//!
//! # What this crate solves
//!
//! - Compile-time separation of dimensions (length vs time vs angle, …).
//! - Zero runtime overhead for unit tags (phantom types only).
//! - Full dimensional arithmetic: `m * m → Prod<Meter, Meter>`,
//!   `m / s → Per<Meter, Second>`, `m / m → Quantity<Unitless>`.
//!   Named derived units (e.g. `SquareMeter`) are obtained via `.to()`.
//! - Automatic compile-time verification that multiplied/divided quantities
//!   produce the correct dimension.
//!
//! # What this crate does not try to solve
//!
//! - General-purpose symbolic simplification of arbitrary unit expressions.
//!   Products and quotients are structural (`Prod<A, B>`, `Per<A, B>`); use `.to()` to convert to named units.
//!
//! # Quick start
//!
//! Convert between predefined units:
//!
//! ```rust
//! use qtty_core::length::{Kilometers, Meter};
//!
//! let km = Kilometers::new(1.25);
//! let m = km.to::<Meter>();
//! assert!((m.value() - 1250.0).abs() < 1e-12);
//! ```
//!
//! Compose derived units using `/`:
//!
//! ```rust
//! use qtty_core::length::{Meter, Meters};
//! use qtty_core::time::{Second, Seconds};
//! use qtty_core::velocity::Velocity;
//!
//! let d = Meters::new(100.0);
//! let t = Seconds::new(20.0);
//! let v: Velocity<Meter, Second> = d / t;
//! assert!((v.value() - 5.0).abs() < 1e-12);
//! ```
//!
//! # `no_std`
//!
//! Disable default features to build `qtty-core` without `std`:
//!
//! ```toml
//! [dependencies]
//! qtty-core = { version = "0.6.1", default-features = false }
//! ```
//!
//! When `std` is disabled, floating-point math that isn't available in `core` is provided via `libm`.
//!
//! # Feature flags
//!
//! - `std` (default): enables `std` support.
//! - `cross-unit-ops` (default): enables direct cross-unit comparison operators (`==`, `<`, etc.) for built-in unit catalogs.
//! - `serde`: enables `serde` support for `Quantity<U, S>`; serialization is the raw scalar value.
//! - `pyo3`: enables PyO3 bindings for Python interop via `#[pyclass]` and `#[pymethods]`.
//!
//! # Panics and errors
//!
//! This crate does not define an error type and does not return `Result` from its core operations. For floating-point
//! scalars (`f64`, `f32`), arithmetic follows IEEE-754 behavior (NaN and infinities propagate). For integer
//! scalars, `abs()` uses saturating semantics at the minimum value (e.g. `i32::MIN.abs()` returns `i32::MAX`
//! instead of panicking). Standard integer overflow rules still apply to addition, subtraction, and multiplication.
//!
//! # SemVer and stability
//!
//! This crate is currently `0.x`. Expect breaking changes between minor versions until `1.0`.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![recursion_limit = "512"]

#[cfg(not(feature = "std"))]
extern crate libm;

// ─────────────────────────────────────────────────────────────────────────────
// Core modules
// ─────────────────────────────────────────────────────────────────────────────

mod dimension;
#[cfg(feature = "diesel")]
mod feature_diesel;
#[cfg(feature = "pyo3")]
mod feature_pyo3;
#[cfg(feature = "serde")]
mod feature_serde;
#[cfg(feature = "tiberius")]
mod feature_tiberius;
mod macros;
mod quantity;
pub mod scalar;
mod unit;
/// Stable unit arithmetic layer: [`UnitDiv`] and [`UnitMul`] traits.
pub mod unit_arithmetic;

// ─────────────────────────────────────────────────────────────────────────────
// Public re-exports of core types
// ─────────────────────────────────────────────────────────────────────────────

pub use dimension::{
    // Derived dimensions
    Acceleration,
    // Additional base dimensions (less commonly used)
    AmountOfSubstance,
    // Base dimensions
    Angular,
    AngularRate,
    Area,
    Current,
    Dimension,
    Dimensionless,
    Energy,
    Force,
    Length,
    LuminousIntensity,
    Mass,
    Power,
    Temperature,
    Time,
    Velocity,
    Volume,
};

// Implementation machinery — public for advanced downstream use but not part
// of the recommended API surface.
#[doc(hidden)]
pub use dimension::{Dim, DimDiv, DimMul};
pub use quantity::{
    Quantity, Quantity32, Quantity64, QuantityI128, QuantityI16, QuantityI32, QuantityI64,
    QuantityI8,
};
pub use scalar::{Exact, IntegerScalar, Real, Scalar, Transcendental};
pub use unit::{Per, Prod, Unit};
pub use unit_arithmetic::{QuantityDivOutput, SameDivOutput, UnitDiv, UnitMul};

#[cfg(feature = "scalar-rational")]
pub use quantity::QuantityRational;

#[cfg(all(feature = "serde", feature = "std"))]
pub use feature_serde::serde_with_unit;

#[cfg(feature = "serde")]
pub use feature_serde::serde_scalar;

// ─────────────────────────────────────────────────────────────────────────────
// Predefined unit modules (grouped by dimension)
// ─────────────────────────────────────────────────────────────────────────────

/// Predefined unit modules (grouped by dimension).
///
/// These are defined in `qtty-core` so they can implement formatting and helper traits without running into Rust's
/// orphan rules.
pub mod units;

pub use units::acceleration;
pub use units::angular;
pub use units::angular_rate;
pub use units::area;
pub use units::energy;
pub use units::force;
pub use units::length;
pub use units::mass;
pub use units::power;
pub use units::time;
pub use units::velocity;
pub use units::volume;

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Test dimension and unit for lib.rs tests
    // ─────────────────────────────────────────────────────────────────────────────

    // Use Length as the test dimension (it's a type alias for Dim<P1, Z0, …>).
    type TestDim = Length;

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub enum TestUnit {}
    impl Unit for TestUnit {
        const RATIO: f64 = 1.0;
        type Dim = TestDim;
        const SYMBOL: &'static str = "tu";
    }
    impl core::fmt::Display for Quantity<TestUnit> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} tu", self.value())
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub enum DoubleTestUnit {}
    impl Unit for DoubleTestUnit {
        const RATIO: f64 = 2.0;
        type Dim = TestDim;
        const SYMBOL: &'static str = "dtu";
    }
    impl core::fmt::Display for Quantity<DoubleTestUnit> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} dtu", self.value())
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub enum HalfTestUnit {}
    impl Unit for HalfTestUnit {
        const RATIO: f64 = 0.5;
        type Dim = TestDim;
        const SYMBOL: &'static str = "htu";
    }
    impl core::fmt::Display for Quantity<HalfTestUnit> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} htu", self.value())
        }
    }

    type TU = Quantity<TestUnit>;
    type Dtu = Quantity<DoubleTestUnit>;

    // ─────────────────────────────────────────────────────────────────────────────
    // Quantity core behavior
    // ─────────────────────────────────────────────────────────────────────────────

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
    fn quantity_has_scalar_layout() {
        assert_eq!(
            core::mem::size_of::<Quantity<TestUnit>>(),
            core::mem::size_of::<f64>()
        );
        assert_eq!(
            core::mem::align_of::<Quantity<TestUnit>>(),
            core::mem::align_of::<f64>()
        );
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Conversion via `to`
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn quantity_conversion_to_same_unit() {
        let q = TU::new(10.0);
        let converted = q.to::<TestUnit>();
        assert_eq!(converted.value(), 10.0);
    }

    #[test]
    fn quantity_conversion_to_different_unit() {
        // 1 DoubleTestUnit = 2 TestUnit (in canonical terms)
        // So 10 TU -> 10 * (1.0 / 2.0) = 5 DTU
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

    // ─────────────────────────────────────────────────────────────────────────────
    // Const helper methods: add/sub/mul/div/min
    // ─────────────────────────────────────────────────────────────────────────────

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
        assert_eq!(a.min_const(b).value(), 3.0);
        assert_eq!(b.min_const(a).value(), 3.0);
    }

    #[test]
    fn const_max() {
        let a = TU::new(3.0);
        let b = TU::new(5.0);
        // When self < other, max_const returns other (the else branch)
        assert_eq!(a.max_const(b).value(), 5.0);
        // When self > other, max_const returns self (the if branch)
        assert_eq!(b.max_const(a).value(), 5.0);
    }

    #[test]
    fn sum_quantities_owned() {
        let qs = vec![TU::new(1.0), TU::new(2.0), TU::new(3.0)];
        let total: TU = qs.into_iter().sum();
        assert_eq!(total.value(), 6.0);
    }

    #[test]
    fn sum_quantities_by_ref() {
        let qs = [TU::new(1.0), TU::new(2.0), TU::new(3.0)];
        let total: TU = qs.iter().sum();
        assert_eq!(total.value(), 6.0);
    }

    #[test]
    fn sum_quantities_into_quantity() {
        let qs = vec![TU::new(1.0), TU::new(2.0), TU::new(3.0)];
        let total: TU = qs.into_iter().sum();
        assert_eq!(total.value(), 6.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Operator traits: Add, Sub, Mul, Div, Neg, Rem
    // ─────────────────────────────────────────────────────────────────────────────

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

    // ─────────────────────────────────────────────────────────────────────────────
    // Assignment operators: AddAssign, SubAssign, DivAssign
    // ─────────────────────────────────────────────────────────────────────────────

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
        q /= 4.0;
        assert_eq!(q.value(), 5.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Division: same-unit yields raw scalar, cross-unit yields Per<N, D>
    // ─────────────────────────────────────────────────────────────────────────────

    // Register test units for arithmetic.
    impl_unit_arithmetic_pairs!(TestUnit, DoubleTestUnit, HalfTestUnit);

    #[test]
    fn division_same_unit_gives_raw_scalar() {
        let a = TU::new(100.0);
        let b = TU::new(20.0);
        let ratio: f64 = a / b;
        assert!((ratio - 5.0).abs() < 1e-12);
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
        // UnitMul: Per<TU, DTU> * DTU → TU (recovery impl)
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

    // ─────────────────────────────────────────────────────────────────────────────
    // asin_angle / acos_angle / atan_angle on Quantity<Per<TU,DTU>> (Dimensionless)
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn dimensionless_asin_angle() {
        use crate::unit::Per;
        use crate::units::angular::Radian;
        // Per<TestUnit, DoubleTestUnit> has Dim = Dimensionless → asin_angle available
        let ratio: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(0.5);
        let result: Quantity<Radian> = ratio.asin_angle();
        assert!((result.value() - 0.5_f64.asin()).abs() < 1e-12);
    }

    #[test]
    fn dimensionless_asin_angle_boundary_values() {
        use crate::unit::Per;
        let one: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(1.0);
        assert!((one.asin_angle().value() - core::f64::consts::FRAC_PI_2).abs() < 1e-12);

        let neg_one: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(-1.0);
        assert!((neg_one.asin_angle().value() - (-core::f64::consts::FRAC_PI_2)).abs() < 1e-12);

        let zero: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(0.0);
        assert!((zero.asin_angle().value() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn same_unit_ratio_is_raw_scalar() {
        // Same-unit division now returns the raw scalar directly (not a Quantity).
        let ratio: f64 = TU::new(1.0) / TU::new(2.0);
        assert!((ratio - 0.5).abs() < 1e-12);
    }

    #[test]
    fn asin_angle_to_degrees() {
        use crate::unit::Per;
        use crate::units::angular::{Degree, Radian};
        let ratio: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(0.5);
        let angle: Quantity<Radian> = ratio.asin_angle();
        let deg: Quantity<Degree> = angle.to();
        assert!((deg.value() - 30.0).abs() < 1e-10);
    }

    #[test]
    fn acos_angle_typed() {
        use crate::unit::Per;
        use crate::units::angular::Radian;
        let ratio: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(0.5);
        let result: Quantity<Radian> = ratio.acos_angle();
        assert!((result.value() - 0.5_f64.acos()).abs() < 1e-12);
    }

    #[test]
    fn atan_angle_typed() {
        use crate::unit::Per;
        use crate::units::angular::Radian;
        let ratio: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(1.0);
        let result: Quantity<Radian> = ratio.atan_angle();
        assert!((result.value() - core::f64::consts::FRAC_PI_4).abs() < 1e-12);
    }

    #[test]
    fn asin_angle_sin_roundtrip() {
        use crate::unit::Per;
        use crate::units::angular::Radian;
        let ratio: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(0.75);
        let angle: Quantity<Radian> = ratio.asin_angle();
        let back = angle.sin();
        assert!((back - 0.75).abs() < 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display formatting
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_simple_quantity() {
        let q = TU::new(42.5);
        let s = format!("{}", q);
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
        let s = format!("{}", q);
        assert_eq!(s, "-99.9 tu");
    }

    #[test]
    fn display_double_test_unit() {
        let q = Dtu::new(2.5);
        let s = format!("{}", q);
        assert_eq!(s, "2.5 dtu");
    }

    #[test]
    fn display_half_test_unit() {
        type Htu = Quantity<HalfTestUnit>;
        let q = Htu::new(3.0);
        let s = format!("{}", q);
        assert_eq!(s, "3 htu");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────────

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

    // ─────────────────────────────────────────────────────────────────────────────
    // Serde tests
    // ─────────────────────────────────────────────────────────────────────────────

    // ─────────────────────────────────────────────────────────────────────────────
    // to_lossy / checked_to_lossy regression: large integer same-unit corruption
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn to_lossy_same_unit_large_i64_is_stable() {
        // Before the fix, the f64 round-trip corrupted values near i64::MAX:
        //   (i64::MAX - 1) as f64 rounds up to i64::MAX as f64,
        //   then back to i64::MAX — a silent mutation.
        let q = Quantity::<TestUnit, i64>::new(i64::MAX - 1);
        let result: Quantity<TestUnit, i64> = q.to_lossy();
        assert_eq!(result.value(), i64::MAX - 1);
    }

    #[test]
    fn checked_to_lossy_same_unit_large_i64_is_stable() {
        // The "checked" path must not report success on a mutated value.
        let q = Quantity::<TestUnit, i64>::new(i64::MAX - 1);
        let result: Option<Quantity<TestUnit, i64>> = q.checked_to_lossy();
        assert_eq!(result.map(|q| q.value()), Some(i64::MAX - 1));
    }

    #[test]
    fn checked_to_lossy_cross_unit_overflow_returns_none() {
        // 1 km in i8 meters = 1000, which overflows i8.
        use crate::length::{Kilometer, Meter};
        let km = Quantity::<Kilometer, i8>::new(1);
        assert_eq!(km.checked_to_lossy::<Meter>(), None);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // mean regression: same-sign infinities must not produce NaN
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn mean_positive_infinity_stays_infinite() {
        // The split-half overflow-avoidance path computed ∞ − ∞ = NaN.
        // The midpoint of two identical positive infinities must be +∞.
        let inf = TU::INFINITY;
        assert!(inf.mean(inf).value().is_infinite());
        assert!(inf.mean(inf).value() > 0.0);
    }

    #[test]
    fn mean_negative_infinity_stays_infinite() {
        let neg_inf = TU::NEG_INFINITY;
        assert!(neg_inf.mean(neg_inf).value().is_infinite());
        assert!(neg_inf.mean(neg_inf).value() < 0.0);
    }

    #[test]
    fn mean_finite_values_unaffected() {
        assert_eq!(TU::new(10.0).mean(TU::new(14.0)).value(), 12.0);
        assert_eq!(
            TU::new(i64::MAX as f64)
                .mean(TU::new(i64::MAX as f64))
                .value(),
            i64::MAX as f64
        );
    }

    /// Regression: mixed finite/infinite same-sign operands must stay infinite.
    ///
    /// The split-half path computed `∞ - ∞ = NaN` for these cases before the fix.
    #[test]
    fn mean_pos_infinity_with_finite_stays_infinite() {
        let inf = TU::INFINITY;
        let fin = TU::new(1.0);
        let result = inf.mean(fin);
        assert!(
            result.value().is_infinite() && result.value() > 0.0,
            "+∞.mean(1.0) must be +∞, got {:?}",
            result.value()
        );
        // symmetry
        let result2 = fin.mean(inf);
        assert!(
            result2.value().is_infinite() && result2.value() > 0.0,
            "1.0.mean(+∞) must be +∞, got {:?}",
            result2.value()
        );
    }

    #[test]
    fn mean_neg_infinity_with_finite_stays_infinite() {
        let neg_inf = TU::NEG_INFINITY;
        let fin = TU::new(-1.0);
        let result = neg_inf.mean(fin);
        assert!(
            result.value().is_infinite() && result.value() < 0.0,
            "-∞.mean(-1.0) must be -∞, got {:?}",
            result.value()
        );
        // symmetry
        let result2 = fin.mean(neg_inf);
        assert!(
            result2.value().is_infinite() && result2.value() < 0.0,
            "-1.0.mean(-∞) must be -∞, got {:?}",
            result2.value()
        );
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use serde::{Deserialize, Serialize};

        #[test]
        fn serialize_quantity() {
            let q = TU::new(42.5);
            let json = serde_json::to_string(&q).unwrap();
            assert_eq!(json, "42.5");
        }

        #[test]
        fn deserialize_quantity() {
            let json = "42.5";
            let q: TU = serde_json::from_str(json).unwrap();
            assert_eq!(q.value(), 42.5);
        }

        #[test]
        fn serde_roundtrip() {
            let original = TU::new(123.456);
            let json = serde_json::to_string(&original).unwrap();
            let restored: TU = serde_json::from_str(&json).unwrap();
            assert!((restored.value() - original.value()).abs() < 1e-12);
        }

        // ─────────────────────────────────────────────────────────────────────────
        // serde_with_unit module tests
        // ─────────────────────────────────────────────────────────────────────────

        #[derive(Serialize, Deserialize, Debug)]
        struct TestStruct {
            #[serde(with = "crate::serde_with_unit")]
            distance: TU,
        }

        #[test]
        fn serde_with_unit_serialize() {
            let data = TestStruct {
                distance: TU::new(42.5),
            };
            let json = serde_json::to_string(&data).unwrap();
            assert!(json.contains("\"value\""));
            assert!(json.contains("\"unit\""));
            assert!(json.contains("42.5"));
            assert!(json.contains("\"tu\""));
        }

        #[test]
        fn serde_with_unit_deserialize() {
            let json = r#"{"distance":{"value":42.5,"unit":"tu"}}"#;
            let data: TestStruct = serde_json::from_str(json).unwrap();
            assert_eq!(data.distance.value(), 42.5);
        }

        #[test]
        fn serde_with_unit_deserialize_no_unit_field() {
            // Should work without unit field for backwards compatibility
            let json = r#"{"distance":{"value":42.5}}"#;
            let data: TestStruct = serde_json::from_str(json).unwrap();
            assert_eq!(data.distance.value(), 42.5);
        }

        #[test]
        fn serde_with_unit_deserialize_wrong_unit() {
            let json = r#"{"distance":{"value":42.5,"unit":"wrong"}}"#;
            let result: Result<TestStruct, _> = serde_json::from_str(json);
            assert!(result.is_err());
            let err_msg = result.unwrap_err().to_string();
            assert!(err_msg.contains("unit mismatch") || err_msg.contains("expected"));
        }

        #[test]
        fn serde_with_unit_deserialize_missing_value() {
            let json = r#"{"distance":{"unit":"tu"}}"#;
            let result: Result<TestStruct, _> = serde_json::from_str(json);
            assert!(result.is_err());
            let err_msg = result.unwrap_err().to_string();
            assert!(err_msg.contains("missing field") || err_msg.contains("value"));
        }

        #[test]
        fn serde_with_unit_deserialize_duplicate_value() {
            let json = r#"{"distance":{"value":42.5,"value":100.0,"unit":"tu"}}"#;
            let result: Result<TestStruct, _> = serde_json::from_str(json);
            // This should either error or use one of the values (implementation-dependent)
            // but we're testing that it doesn't panic
            let _ = result;
        }

        #[test]
        fn serde_with_unit_deserialize_duplicate_unit() {
            let json = r#"{"distance":{"value":42.5,"unit":"tu","unit":"tu"}}"#;
            let result: Result<TestStruct, _> = serde_json::from_str(json);
            // Similar to above - just ensure no panic
            let _ = result;
        }

        #[test]
        fn serde_with_unit_deserialize_invalid_format() {
            // Test the expecting() method by providing wrong format
            let json = r#"{"distance":"not_an_object"}"#;
            let result: Result<TestStruct, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn serde_with_unit_deserialize_array() {
            // Test the expecting() method with array format
            let json = r#"{"distance":[42.5, "tu"]}"#;
            let result: Result<TestStruct, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn serde_with_unit_roundtrip() {
            let original = TestStruct {
                distance: TU::new(123.456),
            };
            let json = serde_json::to_string(&original).unwrap();
            let restored: TestStruct = serde_json::from_str(&json).unwrap();
            assert!((restored.distance.value() - original.distance.value()).abs() < 1e-12);
        }

        #[test]
        fn serde_with_unit_special_values() {
            // Note: JSON doesn't support Infinity and NaN natively.
            // serde_json serializes them as null, which can't be deserialized
            // back to f64. So we'll test with very large numbers instead.
            let test_large = TestStruct {
                distance: TU::new(1e100),
            };
            let json = serde_json::to_string(&test_large).unwrap();
            let restored: TestStruct = serde_json::from_str(&json).unwrap();
            assert!((restored.distance.value() - 1e100).abs() < 1e88);

            let test_small = TestStruct {
                distance: TU::new(-1e-100),
            };
            let json = serde_json::to_string(&test_small).unwrap();
            let restored: TestStruct = serde_json::from_str(&json).unwrap();
            assert!((restored.distance.value() + 1e-100).abs() < 1e-112);
        }
    }
}
