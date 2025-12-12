//! # Units Core
//!
//! This crate provides the core types and traits for zero-cost strongly-typed
//! physical units. All type information is resolved at compile time using phantom
//! types, ensuring no runtime overhead.
//!
//! ## Core Types
//!
//! - [`Dimension`] - Marker trait for physical dimensions (Length, Time, Mass, etc.)
//! - [`Unit`] - Trait for unit types with conversion ratio and dimension
//! - [`Quantity<U>`] - The main wrapper type that holds a value with its unit
//! - [`Per<N, D>`] - Generic composite unit for division (e.g., velocity = length/time)
//! - [`DivDim<N, D>`] - Composite dimension type
//!
//! ## Example
//!
//! ```rust
//! use units_core::*;
//!
//! // Define a dimension
//! pub enum Length {}
//! impl Dimension for Length {}
//!
//! // Define units (typically done via derive macro)
//! #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
//! pub enum Meter {}
//! impl Unit for Meter {
//!     const RATIO: f64 = 1.0;
//!     type Dim = Length;
//!     const SYMBOL: &'static str = "m";
//! }
//!
//! // Use quantities
//! let distance: Quantity<Meter> = Quantity::new(100.0);
//! assert_eq!(distance.value(), 100.0);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate libm;

use core::cmp::*;
use core::marker::PhantomData;
use core::ops::*;
use core::fmt::{Display, Formatter, Result, Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Deserializer, Serializer};

// Unit modules - defined in this crate to satisfy orphan rules
pub mod units;

// Re-export commonly used units
pub use units::angular;
pub use units::frequency;
pub use units::length;
pub use units::mass;
pub use units::power;
pub use units::time;
pub use units::unitless;
pub use units::velocity;

/// Marker trait for **dimensions** (Length, Time, Mass …).
///
/// A *dimension* is the category that distinguishes a metre from a second.
/// You usually model each dimension as an empty enum:
///
/// ```rust
/// use units_core::Dimension;
/// #[derive(Debug)]
/// pub enum Length {}
/// impl Dimension for Length {}
/// ```
pub trait Dimension {}

/// Trait implemented by every **unit** type.
///
/// * `RATIO` expresses how many of this unit fit into the *canonical* unit
///   of the same dimension.
///   Example: The ratio for kilometres is `1000.0` because `1 km = 1000 m`.
///
/// * `SYMBOL` is the printable string (e.g. `"m"` or `"km"`).
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

    /// Printable symbol, shown by [`Display`](core::fmt::Display).
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

/// A quantity with a specific unit.
///
/// `Quantity<U>` wraps an `f64` value together with phantom type information
/// about its unit `U`. This enables compile-time dimensional analysis while
/// maintaining zero runtime cost.
///
/// # Examples
///
/// ```rust
/// use units_core::{Quantity, Unit, Dimension};
///
/// pub enum Length {}
/// impl Dimension for Length {}
///
/// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
/// pub enum Meter {}
/// impl Unit for Meter {
///     const RATIO: f64 = 1.0;
///     type Dim = Length;
///     const SYMBOL: &'static str = "m";
/// }
///
/// let x = Quantity::<Meter>::new(5.0);
/// let y = Quantity::<Meter>::new(3.0);
/// let sum = x + y;
/// assert_eq!(sum.value(), 8.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Quantity<U: Unit>(f64, PhantomData<U>);

impl<U: Unit + Copy> Quantity<U> {
    /// A constant representing NaN for this quantity type.
    pub const NAN: Self = Self::new(f64::NAN);

    /// Creates a new quantity with the given value.
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    /// Returns the raw numeric value.
    #[inline]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns the absolute value.
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.0.abs())
    }

    /// Converts this quantity to another unit of the same dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use units_core::{Quantity, Unit, Dimension};
    ///
    /// pub enum Length {}
    /// impl Dimension for Length {}
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    /// pub enum Meter {}
    /// impl Unit for Meter {
    ///     const RATIO: f64 = 1.0;
    ///     type Dim = Length;
    ///     const SYMBOL: &'static str = "m";
    /// }
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    /// pub enum Kilometer {}
    /// impl Unit for Kilometer {
    ///     const RATIO: f64 = 1000.0;
    ///     type Dim = Length;
    ///     const SYMBOL: &'static str = "km";
    /// }
    ///
    /// let km = Quantity::<Kilometer>::new(1.0);
    /// let m: Quantity<Meter> = km.to();
    /// assert_eq!(m.value(), 1000.0);
    /// ```
    #[inline]
    pub const fn to<T: Unit<Dim = U::Dim>>(self) -> Quantity<T> {
        Quantity::<T>::new(self.0 * (U::RATIO / T::RATIO))
    }

    /// Returns the minimum of this quantity and another.
    #[inline]
    pub const fn min(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value().min(other.value()))
    }

    /// Const addition of two quantities.
    #[inline]
    pub const fn add(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() + other.value())
    }

    /// Const subtraction of two quantities.
    #[inline]
    pub const fn sub(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() - other.value())
    }

    /// Const division of two quantities (returns same unit, useful for ratios).
    #[inline]
    pub const fn div(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() / other.value())
    }

    /// Const multiplication of two quantities (returns same unit).
    #[inline]
    pub const fn mul(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() * other.value())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Operator implementations
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit> Add for Quantity<U> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

impl<U: Unit> AddAssign for Quantity<U> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<U: Unit> Sub for Quantity<U> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

impl<U: Unit> SubAssign for Quantity<U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<U: Unit> Mul<f64> for Quantity<U> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.0 * rhs)
    }
}

impl<U: Unit> Mul<Quantity<U>> for f64 {
    type Output = Quantity<U>;
    #[inline]
    fn mul(self, rhs: Quantity<U>) -> Self::Output {
        rhs * self
    }
}

impl<U: Unit> Div<f64> for Quantity<U> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self::new(self.0 / rhs)
    }
}

impl<N: Unit, D: Unit> Mul<Quantity<D>> for Quantity<Per<N, D>> {
    type Output = Quantity<N>;

    #[inline]
    fn mul(self, rhs: Quantity<D>) -> Self::Output {
        Quantity::<N>::new(self.0 * rhs.value())
    }
}

impl<N: Unit, D: Unit> Mul<Quantity<Per<N, D>>> for Quantity<D> {
    type Output = Quantity<N>;

    #[inline]
    fn mul(self, rhs: Quantity<Per<N, D>>) -> Self::Output {
        rhs * self
    }
}

impl<U: Unit> DivAssign for Quantity<U> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<U: Unit> Rem<f64> for Quantity<U> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f64) -> Self {
        Self::new(self.0 % rhs)
    }
}

impl<U: Unit> PartialEq<f64> for Quantity<U> {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl<U: Unit> Neg for Quantity<U> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.0)
    }
}

impl<U: Unit> From<f64> for Quantity<U> {
    #[inline]
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl<N: Unit, D: Unit> Div<Quantity<D>> for Quantity<N> {
    type Output = Quantity<Per<N, D>>;
    #[inline]
    fn div(self, rhs: Quantity<D>) -> Self::Output {
        Quantity::new(self.value() / rhs.value())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Simplify trait for composite units
// ─────────────────────────────────────────────────────────────────────────────

/// Trait for simplifying composite unit types.
///
/// This allows reducing complex unit expressions to simpler forms,
/// such as `Per<U, U>` to `Unitless` or `Per<N, Per<N, D>>` to `D`.
pub trait Simplify {
    type Out: Unit;
    fn simplify(self) -> Quantity<Self::Out>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Unitless / Dimensionless support
// ─────────────────────────────────────────────────────────────────────────────

/// Dimension for dimensionless quantities.
pub enum Dimensionless {}
impl Dimension for Dimensionless {}

/// Unitless type, used for dimensionless ratios.
pub type Unitless = f64;

impl Unit for Unitless {
    const RATIO: f64 = 1.0;
    type Dim = Dimensionless;
    const SYMBOL: &'static str = "";
}

impl Display for Quantity<Unitless> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value())
    }
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
    /// Arc sine of a unitless ratio.
    #[inline]
    pub fn asin(&self) -> f64 {
        #[cfg(feature = "std")]
        { self.value().asin() }
        #[cfg(not(feature = "std"))]
        { libm::asin(self.value()) }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Serde support (behind feature flag)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl<U: Unit> Serialize for Quantity<U> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, U: Unit> Deserialize<'de> for Quantity<U> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Ok(Quantity::new(value))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Legacy macro for backward compatibility
// ─────────────────────────────────────────────────────────────────────────────

/// Generate a **unit type** and its [`Display`] implementation.
///
/// This macro is provided for backward compatibility. New code should prefer
/// using the `#[derive(Unit)]` procedural macro from `units-derive`.
///
/// # Example
///
/// ```rust,ignore
/// use units_core::{define_unit, Unit, Quantity, Dimension};
///
/// pub enum Length {}
/// impl Dimension for Length {}
///
/// define_unit!("m", Meter, Length, 1.0);
/// pub type Meters = Quantity<Meter>;
///
/// let m = Meters::new(42.0);
/// assert_eq!(format!("{}", m), "42 m");
/// ```
#[macro_export]
macro_rules! define_unit {
    ($symbol:expr, $name:ident, $dim:ty, $ratio:expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub enum $name {}
        impl $crate::Unit for $name {
            const RATIO: f64 = $ratio;
            type Dim = $dim;
            const SYMBOL: &'static str = $symbol;
        }
        impl core::fmt::Display for $crate::Quantity<$name> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{} {}", self.value(), <$name as $crate::Unit>::SYMBOL)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

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
