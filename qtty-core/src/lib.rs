//! Core type system for strongly typed physical quantities.
//!
//! `qtty-core` provides a minimal, zero-cost units model:
//!
//! - A *unit* is a zero-sized marker type implementing [`Unit`].
//! - A value tagged with a unit is a [`Quantity<U>`], backed by an `f64`.
//! - Conversion is an explicit, type-checked scaling via [`Quantity::to`].
//! - Derived units like velocity are expressed as [`Per<N, D>`] (e.g. `Meter/Second`).
//!
//! Most users should depend on `qtty` (the facade crate) unless they need direct access to these primitives.
//!
//! # What this crate solves
//!
//! - Compile-time separation of dimensions (length vs time vs angle, …).
//! - Zero runtime overhead for unit tags (phantom types only).
//! - A small vocabulary to express derived units via type aliases (`Per`, `DivDim`).
//!
//! # What this crate does not try to solve
//!
//! - Exact arithmetic (`Quantity` is `f64`).
//! - General-purpose symbolic simplification of arbitrary unit expressions.
//! - Automatic tracking of exponent dimensions (`m^2`, `s^-1`, …); only the expression forms represented by the
//!   provided types are modeled.
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
//! qtty-core = { version = "0.1.0", default-features = false }
//! ```
//!
//! When `std` is disabled, floating-point math that isn't available in `core` is provided via `libm`.
//!
//! # Feature flags
//!
//! - `std` (default): enables `std` support.
//! - `serde`: enables `serde` support for `Quantity<U>`; serialization is the raw `f64` value only.
//!
//! # Panics and errors
//!
//! This crate does not define an error type and does not return `Result` from its core operations. Conversions and
//! arithmetic are pure `f64` computations; they do not panic on their own, but they follow IEEE-754 behavior (NaN and
//! infinities propagate according to the underlying operation).
//!
//! # SemVer and stability
//!
//! This crate is currently `0.x`. Expect breaking changes between minor versions until `1.0`.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(not(feature = "std"))]
extern crate libm;

// ─────────────────────────────────────────────────────────────────────────────
// Core modules
// ─────────────────────────────────────────────────────────────────────────────

mod dimension;
mod macros;
mod quantity;
mod unit;

// ─────────────────────────────────────────────────────────────────────────────
// Public re-exports of core types
// ─────────────────────────────────────────────────────────────────────────────

pub use dimension::{Dimension, Dimensionless, DivDim};
pub use quantity::Quantity;
pub use unit::{Per, Simplify, Unit, Unitless};

// ─────────────────────────────────────────────────────────────────────────────
// Predefined unit modules (grouped by dimension)
// ─────────────────────────────────────────────────────────────────────────────

/// Predefined unit modules (grouped by dimension).
///
/// These are defined in `qtty-core` so they can implement formatting and helper traits without running into Rust's
/// orphan rules.
pub mod units;

pub use units::angular;
pub use units::frequency;
pub use units::length;
pub use units::mass;
pub use units::power;
pub use units::time;
pub use units::unitless;
pub use units::velocity;

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Test dimension and unit for lib.rs tests
    // ─────────────────────────────────────────────────────────────────────────────
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
        q /= TU::new(4.0);
        assert_eq!(q.value(), 5.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // PartialEq<f64>
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn partial_eq_f64() {
        let q = TU::new(5.0);
        assert!(q == 5.0);
        assert!(!(q == 4.0));
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Division yielding Per<N, D>
    // ─────────────────────────────────────────────────────────────────────────────

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

    // ─────────────────────────────────────────────────────────────────────────────
    // Simplify trait
    // ─────────────────────────────────────────────────────────────────────────────

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

    // ─────────────────────────────────────────────────────────────────────────────
    // Quantity<Per<U,U>>::asin()
    // ─────────────────────────────────────────────────────────────────────────────

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

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;

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
    }
}
