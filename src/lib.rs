//! # Units Module
//!
//! This module provides a comprehensive set of strongly-typed units and utilities
//! for astronomical and scientific calculations. It is designed to ensure correctness,
//! clarity, and ease of use when working with various units of measurement.
//!
//! ## Features
//! - **Time Units**: Includes representations for Days, Years, Julian Years, and Centuries.
//! - **Angular Units**: Provides types for Degrees, Radians, DMS (Degrees, Minutes, Seconds), HMS (HourAngles, Minutes, Seconds), and Arcseconds.
//! - **Length Units**: Includes types for meters and astronomical units (AstronomicalUnits).
//! - **Velocity Units**: Provides types for meters per second and kilometers per second.
//! - **Mass Units**: Includes types for kilograms and solar masses.
//! - **Power Units**: Includes types for watts and solar luminosity.
//! - **Arithmetic Operations**: Supports arithmetic operations between compatible units, ensuring type safety.
//!
//! ## Example Usage
//! ```rust
//! use siderust_units::*;
//!
//! // Angular Units
//! let degrees = Degrees::new(180.0);
//! let radians = degrees.to::<Radian>();
//! let dms = Degrees::from_dms(12, 34, 56.0);
//!
//! // Mass Units
//! let mass_kg = Kilograms::new(5.0);
//! let mass_solar = SolarMasses::new(2.0);
//!
//! // Conversions
//! let dms_to_decimal = dms.value();
//!
//! assert_eq!(radians.value(), std::f64::consts::PI);
//! ```
//!
//! ## Modules
//! - [`time`]: Time-related units and utilities.
//! - [`angular`]: Angular measurement units and utilities.
//! - [`length`]: Length units and utilities.
//! - [`velocity`]: Velocity-related units and utilities.
//! - [`mass`]: Mass-related units and utilities.
//! - [`power`]: Power-related units and utilities.

pub mod angular;
pub mod frequency;
pub mod length;
pub mod mass;
pub mod power;
pub mod time;
pub mod unitless;
pub mod velocity;

pub use angular::*;
pub use frequency::*;
pub use length::*;
pub use mass::*;
pub use power::*;
pub use time::*;
pub use unitless::*;
pub use velocity::*;

use core::cmp::*;
use core::marker::PhantomData;
use core::ops::*;
use std::fmt::*;

/// Marker trait for **dimensions** (Length, Time, Mass …).
///
/// A *dimension* is the category that distinguishes a metre from a second.
/// You usually model each dimension as an empty enum:
///
/// ```rust
/// use siderust_units::Dimension;
/// #[derive(Debug)]
/// pub enum Length {}
/// impl Dimension for Length {}
/// ```
pub trait Dimension {}

/// Trait implemented by every **unit** type generated through [`define_unit!`].
///
/// * `RATIO` expresses how many of this unit fit into the *canonical* unit
///   of the same dimension.
///   Example: The ratio for centimetres is `1.000` because `1 km = 1.000 m`.
///
/// * `SYMBOL` is the printable string (e.g. `"m"` or `"cm"`).
///
/// * `Dim` ties the unit to its underlying [`Dimension`].
///
/// # Safety
/// The trait is `Copy + 'static`, so types must be zero-sized marker enums.
pub trait Unit: Copy + PartialEq + Debug + 'static {
    /// Unit-to-canonical conversion factor.
    const RATIO: f64;

    /// Dimension to which this unit belongs.
    type Dim: Dimension;

    /// Printable symbol, shown by [`Display`](std::fmt::Display).
    const SYMBOL: &'static str;
}

/// Dimension formed by dividing one [`Dimension`] by another.
///
/// This is used to model composite dimensions such as `Length/Time`
/// for velocities or `Angular/Time` for frequencies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DivDim<N: Dimension, D: Dimension>(PhantomData<(N, D)>);
impl<N: Dimension, D: Dimension> Dimension for DivDim<N, D> {}

/// Unit representing the division of two other units.
///
/// `Per<N, D>` corresponds to `N / D` and carries both the
/// dimensional information and the scaling ratio between the
/// constituent units. It is generic over any numerator and
/// denominator units, which allows implementing arithmetic
/// generically for all pairs without bespoke macros.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Per<N: Unit, D: Unit>(PhantomData<(N, D)>);

impl<N: Unit, D: Unit> Unit for Per<N, D> {
    const RATIO: f64 = N::RATIO / D::RATIO;
    type Dim = DivDim<N::Dim, D::Dim>;
    // The symbol is constructed at formatting time since generic
    // constants cannot concatenate at compile time.
    const SYMBOL: &'static str = "";
}

impl<N: Unit, D: Unit> Display for Quantity<Per<N, D>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}/{}", self.value(), N::SYMBOL, D::SYMBOL)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Quantity<U: Unit>(f64, PhantomData<U>);

impl<U: Unit + Copy> Quantity<U> {
    pub const NAN: Self = Self::new(f64::NAN);

    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    pub const fn value(self) -> f64 {
        self.0
    }

    pub fn abs(self) -> Self {
        Self::new(self.0.abs())
    }

    pub const fn to<T: Unit<Dim = U::Dim>>(self) -> Quantity<T> {
        Quantity::<T>::new(self.0 * (U::RATIO / T::RATIO))
    }

    pub const fn min(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value().min(other.value()))
    }

    pub const fn add(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() + other.value())
    }

    pub const fn sub(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() - other.value())
    }

    pub const fn div(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() / other.value())
    }

    pub const fn mul(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() * other.value())
    }
}

impl<U: Unit> Add for Quantity<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

impl<U: Unit> AddAssign for Quantity<U> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<U: Unit> Sub for Quantity<U> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

impl<U: Unit> SubAssign for Quantity<U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<U: Unit> Mul<f64> for Quantity<U> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.0 * rhs)
    }
}

impl<U: Unit> Mul<Quantity<U>> for f64 {
    type Output = Quantity<U>;
    fn mul(self, rhs: Quantity<U>) -> Self::Output {
        rhs * self
    }
}

impl<U: Unit> Div<f64> for Quantity<U> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::new(self.0 / rhs)
    }
}

impl<N: Unit, D: Unit> Mul<Quantity<D>> for Quantity<Per<N, D>> {
    type Output = Quantity<N>;

    fn mul(self, rhs: Quantity<D>) -> Self::Output {
        Quantity::<N>::new(self.0 * rhs.value())
    }
}

impl<N: Unit, D: Unit> Mul<Quantity<Per<N, D>>> for Quantity<D> {
    type Output = Quantity<N>;

    fn mul(self, rhs: Quantity<Per<N, D>>) -> Self::Output {
        rhs * self
    }
}

impl<U: Unit> DivAssign for Quantity<U> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<U: Unit> Rem<f64> for Quantity<U> {
    type Output = Self;
    fn rem(self, rhs: f64) -> Self {
        Self::new(self.0 % rhs)
    }
}

impl<U: Unit> PartialEq<f64> for Quantity<U> {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl<U: Unit> Neg for Quantity<U> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.0)
    }
}

impl<U: Unit> From<f64> for Quantity<U> {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

/* TODO: Requires specialization (nightly) see #16

impl<N: Unit, D: Unit> Div<Quantity<D>> for Quantity<N> {
    type Output = Quantity<Per<N, D>>;

    fn div(self, rhs: Quantity<D>) -> Self::Output {
        Quantity::<Per<N, D>>::new(self.0 / rhs.0)
    }
}

impl<N: Unit, D: Unit> Div<Quantity<Per<N, D>>> for Quantity<N> {
    type Output = Quantity<D>;

    fn div(self, rhs: Quantity<Per<N, D>>) -> Self::Output {
        Quantity::<D>::new(self.0 / rhs.0)
    }
}

*/

impl<N: Unit, D: Unit> std::ops::Div<Quantity<D>> for Quantity<N> {
    type Output = Quantity<Per<N, D>>;
    fn div(self, rhs: Quantity<D>) -> Self::Output {
        Quantity::new(self.value() / rhs.value())
    }
}

pub trait Simplify {
    type Out: Unit;
    fn simplify(self) -> Quantity<Self::Out>;
}

// U/U → Unitless
impl<U: Unit> Simplify for Quantity<Per<U, U>> {
    type Out = Unitless;
    fn simplify(self) -> Quantity<Unitless> {
        Quantity::new(self.value())
    }
}

// N / (N/D) → D
impl<N: Unit, D: Unit> Simplify for Quantity<Per<N, Per<N, D>>> {
    type Out = D;
    fn simplify(self) -> Quantity<D> {
        Quantity::new(self.value())
    }
}

impl<U: Unit> Quantity<Per<U, U>> {
    #[inline]
    pub fn asin(&self) -> f64 {
        self.value().asin()
    }
}

/// Generate a **unit type** and its [`Display`] implementation.
#[macro_export]
macro_rules! define_unit {
    ($symbol:expr, $name:ident, $dim:ty, $ratio:expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub enum $name {}
        impl Unit for $name {
            const RATIO: f64 = $ratio;
            type Dim = $dim;
            const SYMBOL: &'static str = stringify!($symbol);
        }
        impl std::fmt::Display for Quantity<$name> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.value(), <$name>::SYMBOL)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Test dimension and unit for lib.rs tests
    // ─────────────────────────────────────────────────────────────────────────────
    #[derive(Debug)]
    pub enum TestDim {}
    impl Dimension for TestDim {}

    define_unit!("tu", TestUnit, TestDim, 1.0);
    define_unit!("dtu", DoubleTestUnit, TestDim, 2.0);
    define_unit!("htu", HalfTestUnit, TestDim, 0.5);

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
        assert_abs_diff_eq!(converted.value(), 5.0, epsilon = 1e-12);
    }

    #[test]
    fn quantity_conversion_roundtrip() {
        let original = TU::new(100.0);
        let converted = original.to::<DoubleTestUnit>();
        let back = converted.to::<TestUnit>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
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
        // Use explicit method call to avoid trait method resolution
        assert_eq!(Quantity::mul(&a, b).value(), 20.0);
    }

    #[test]
    fn const_div() {
        let a = TU::new(20.0);
        let b = TU::new(4.0);
        // Use explicit method call to avoid trait method resolution
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
        assert_abs_diff_eq!(ratio.value(), 5.0, epsilon = 1e-12);
    }

    #[test]
    fn per_ratio_conversion() {
        // Create km/s equivalent using our test units
        let v1: Quantity<Per<DoubleTestUnit, TestUnit>> = Quantity::new(10.0);
        // Convert to base units: ratio = 2.0 / 1.0 = 2.0
        let v2: Quantity<Per<TestUnit, TestUnit>> = v1.to();
        // 10 * (2.0 / 1.0) = 20
        assert_abs_diff_eq!(v2.value(), 20.0, epsilon = 1e-12);
    }

    #[test]
    fn per_multiplication_recovers_numerator() {
        // (N/D) * D = N
        let rate: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(5.0);
        let time = Dtu::new(4.0);
        let result: TU = rate * time;
        assert_abs_diff_eq!(result.value(), 20.0, epsilon = 1e-12);
    }

    #[test]
    fn per_multiplication_commutative() {
        let rate: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(5.0);
        let time = Dtu::new(4.0);
        let result1: TU = rate * time;
        let result2: TU = time * rate;
        assert_abs_diff_eq!(result1.value(), result2.value(), epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Simplify trait
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn simplify_per_u_u_to_unitless() {
        let ratio: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(1.23456);
        let unitless: Quantity<Unitless> = ratio.simplify();
        assert_abs_diff_eq!(unitless.value(), 1.23456, epsilon = 1e-12);
    }

    #[test]
    fn simplify_per_n_per_n_d_to_d() {
        // N / (N/D) -> D
        // Construct explicitly since division impl is not available
        let q: Quantity<Per<TestUnit, Per<TestUnit, DoubleTestUnit>>> = Quantity::new(7.5);
        let simplified: Dtu = q.simplify();
        assert_abs_diff_eq!(simplified.value(), 7.5, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Quantity<Per<U,U>>::asin()
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn per_u_u_asin() {
        let ratio: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(0.5);
        let result = ratio.asin();
        assert_abs_diff_eq!(result, 0.5_f64.asin(), epsilon = 1e-12);
    }

    #[test]
    fn per_u_u_asin_boundary_values() {
        // asin(1) = π/2
        let one: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(1.0);
        assert_abs_diff_eq!(one.asin(), std::f64::consts::FRAC_PI_2, epsilon = 1e-12);

        // asin(-1) = -π/2
        let neg_one: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(-1.0);
        assert_abs_diff_eq!(
            neg_one.asin(),
            -std::f64::consts::FRAC_PI_2,
            epsilon = 1e-12
        );

        // asin(0) = 0
        let zero: Quantity<Per<TestUnit, TestUnit>> = Quantity::new(0.0);
        assert_abs_diff_eq!(zero.asin(), 0.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display formatting
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_simple_quantity() {
        let q = TU::new(42.5);
        let s = format!("{}", q);
        // Note: stringify! in define_unit! macro includes quotes around the symbol
        assert_eq!(s, "42.5 \"tu\"");
    }

    #[test]
    fn display_per_quantity() {
        let q: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(2.5);
        let s = format!("{}", q);
        // Note: stringify! in define_unit! macro includes quotes around symbols
        assert_eq!(s, "2.5 \"tu\"/\"dtu\"");
    }

    #[test]
    fn display_negative_value() {
        let q = TU::new(-99.9);
        let s = format!("{}", q);
        // Note: stringify! in define_unit! macro includes quotes around the symbol
        assert_eq!(s, "-99.9 \"tu\"");
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
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_add_commutative(a in -1e10..1e10f64, b in -1e10..1e10f64) {
            let qa = TU::new(a);
            let qb = TU::new(b);
            assert_abs_diff_eq!((qa + qb).value(), (qb + qa).value(), epsilon = 1e-9);
        }

        #[test]
        fn prop_sub_anticommutative(a in -1e10..1e10f64, b in -1e10..1e10f64) {
            let qa = TU::new(a);
            let qb = TU::new(b);
            assert_abs_diff_eq!((qa - qb).value(), -(qb - qa).value(), epsilon = 1e-9);
        }

        #[test]
        fn prop_mul_f64_associative(v in -1e5..1e5f64, s1 in -1e5..1e5f64, s2 in -1e5..1e5f64) {
            let q = TU::new(v);
            let r1 = (q * s1) * s2;
            let r2 = q * (s1 * s2);
            assert_relative_eq!(r1.value(), r2.value(), max_relative = 1e-10);
        }

        #[test]
        fn prop_neg_involution(v in -1e10..1e10f64) {
            let q = TU::new(v);
            assert_abs_diff_eq!((-(-q)).value(), q.value(), epsilon = 1e-12);
        }

        #[test]
        fn prop_abs_non_negative(v in -1e10..1e10f64) {
            let q = TU::new(v);
            assert!(q.abs().value() >= 0.0);
        }

        #[test]
        fn prop_conversion_roundtrip(v in -1e10..1e10f64) {
            let original = TU::new(v);
            let converted = original.to::<DoubleTestUnit>();
            let back = converted.to::<TestUnit>();
            assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
        }

        #[test]
        fn prop_division_yields_correct_ratio(
            num in 1e-10..1e10f64,
            den in 1e-10..1e10f64
        ) {
            let n = TU::new(num);
            let d = Dtu::new(den);
            let ratio: Quantity<Per<TestUnit, DoubleTestUnit>> = n / d;
            assert_relative_eq!(ratio.value(), num / den, max_relative = 1e-12);
        }

        #[test]
        fn prop_per_mul_inverse(
            rate_val in 1e-5..1e5f64,
            time_val in 1e-5..1e5f64
        ) {
            // (N/D) * D = N, value should be rate * time
            let rate: Quantity<Per<TestUnit, DoubleTestUnit>> = Quantity::new(rate_val);
            let time = Dtu::new(time_val);
            let result: TU = rate * time;
            assert_relative_eq!(result.value(), rate_val * time_val, max_relative = 1e-12);
        }
    }
}
