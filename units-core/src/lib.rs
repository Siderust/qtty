//! Core type system for strongly typed physical units.
//!
//! `unit-core` provides a minimal, zero-cost units model:
//!
//! - A *unit* is a zero-sized marker type implementing [`Unit`].
//! - A value tagged with a unit is a [`Quantity<U>`], backed by an `f64`.
//! - Conversion is an explicit, type-checked scaling via [`Quantity::to`].
//! - Derived units like velocity are expressed as [`Per<N, D>`] (e.g. `Meter/Second`).
//!
//! Most users should depend on `unit` (the facade crate) unless they need direct access to these primitives.
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
//! use unit_core::length::{Kilometers, Meter};
//!
//! let km = Kilometers::new(1.25);
//! let m = km.to::<Meter>();
//! assert!((m.value() - 1250.0).abs() < 1e-12);
//! ```
//!
//! Compose derived units using `/`:
//!
//! ```rust
//! use unit_core::length::Meters;
//! use unit_core::time::Seconds;
//! use unit_core::velocity::MetersPerSecond;
//!
//! let d = Meters::new(100.0);
//! let t = Seconds::new(20.0);
//! let v: MetersPerSecond = d / t;
//! assert!((v.value() - 5.0).abs() < 1e-12);
//! ```
//!
//! # `no_std`
//!
//! Disable default features to build `unit-core` without `std`:
//!
//! ```toml
//! [dependencies]
//! unit-core = { version = "0.1.0", default-features = false }
//! ```
//!
//! When `std` is disabled, floating-point math that isn’t available in `core` is provided via `libm`.
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

use core::fmt::{Debug, Display, Formatter, Result};
use core::marker::PhantomData;
use core::ops::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Predefined unit modules (grouped by dimension).
///
/// These are defined in `unit-core` so they can implement formatting and helper traits without running into Rust’s
/// orphan rules.
pub mod units;

/// Angular units and helpers.
pub use units::angular;
/// Angular frequency unit aliases (`Angular / Time`).
pub use units::frequency;
/// Length units.
pub use units::length;
/// Mass units.
pub use units::mass;
/// Power units.
pub use units::power;
/// Time units.
pub use units::time;
/// Dimensionless helpers.
pub use units::unitless;
/// Velocity unit aliases (`Length / Time`).
pub use units::velocity;

/// Marker trait for **dimensions** (Length, Time, Mass …).
///
/// A *dimension* is the category that distinguishes a metre from a second.
/// You usually model each dimension as an empty enum:
///
/// ```rust
/// use unit_core::Dimension;
/// #[derive(Debug)]
/// pub enum Length {}
/// impl Dimension for Length {}
/// ```
pub trait Dimension {}

/// Trait implemented by every **unit** type.
///
/// * `RATIO` is the conversion factor from this unit to the *canonical scaling unit* of the same dimension.
///   Example: if metres are canonical (`Meter::RATIO == 1.0`), then kilometres use `Kilometer::RATIO == 1000.0`
///   because `1 km = 1000 m`.
///
/// * `SYMBOL` is the printable string (e.g. `"m"` or `"km"`).
///
/// * `Dim` ties the unit to its underlying [`Dimension`].
///
/// # Invariants
///
/// - Implementations should be zero-sized marker types (this crate’s built-in units are unit structs with no fields).
/// - `RATIO` should be finite and non-zero.
pub trait Unit: Copy + PartialEq + Debug + 'static {
    /// Unit-to-canonical conversion factor.
    const RATIO: f64;

    /// Dimension to which this unit belongs.
    type Dim: Dimension;

    /// Printable symbol, shown by [`core::fmt::Display`].
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
/// use unit_core::{Quantity, Unit, Dimension};
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
    ///
    /// ```rust
    /// use unit_core::length::Meters;
    /// assert!(Meters::NAN.value().is_nan());
    /// ```
    pub const NAN: Self = Self::new(f64::NAN);

    /// Creates a new quantity with the given value.
    ///
    /// ```rust
    /// use unit_core::length::Meters;
    /// let d = Meters::new(3.0);
    /// assert_eq!(d.value(), 3.0);
    /// ```
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    /// Returns the raw numeric value.
    ///
    /// ```rust
    /// use unit_core::time::Seconds;
    /// let t = Seconds::new(2.5);
    /// assert_eq!(t.value(), 2.5);
    /// ```
    #[inline]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns the absolute value.
    ///
    /// ```rust
    /// use unit_core::angular::Degrees;
    /// let a = Degrees::new(-10.0);
    /// assert_eq!(a.abs().value(), 10.0);
    /// ```
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.0.abs())
    }

    /// Converts this quantity to another unit of the same dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use unit_core::{Quantity, Unit, Dimension};
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
    ///
    /// ```rust
    /// use unit_core::length::Meters;
    /// let a = Meters::new(3.0);
    /// let b = Meters::new(5.0);
    /// assert_eq!(a.min(b).value(), 3.0);
    /// ```
    #[inline]
    pub const fn min(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value().min(other.value()))
    }

    /// Const addition of two quantities.
    ///
    /// ```rust
    /// use unit_core::length::Meters;
    /// let a = Meters::new(1.0);
    /// let b = Meters::new(2.0);
    /// assert_eq!(a.add(b).value(), 3.0);
    /// ```
    #[inline]
    pub const fn add(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() + other.value())
    }

    /// Const subtraction of two quantities.
    ///
    /// ```rust
    /// use unit_core::length::Meters;
    /// let a = Meters::new(5.0);
    /// let b = Meters::new(2.0);
    /// assert_eq!(a.sub(b).value(), 3.0);
    /// ```
    #[inline]
    pub const fn sub(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() - other.value())
    }

    /// Const division of two quantities (legacy behavior; returns the same unit).
    ///
    /// For a dimensionless ratio, prefer `/` (which yields a `Per<U, U>`) plus [`Simplify`].
    ///
    /// ```rust
    /// use unit_core::length::Meters;
    /// let a = Meters::new(6.0);
    /// let b = Meters::new(2.0);
    /// assert_eq!(a.div(b).value(), 3.0);
    /// ```
    #[inline]
    pub const fn div(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() / other.value())
    }

    /// Const multiplication of two quantities (returns same unit).
    ///
    /// ```rust
    /// use unit_core::length::Meters;
    /// let a = Meters::new(3.0);
    /// let b = Meters::new(4.0);
    /// assert_eq!(a.mul(b).value(), 12.0);
    /// ```
    #[inline]
    pub const fn mul(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value() * other.value())
    }
}

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

/// Trait for simplifying composite unit types.
///
/// This allows reducing complex unit expressions to simpler forms,
/// such as `Per<U, U>` to `Unitless` or `Per<N, Per<N, D>>` to `D`.
pub trait Simplify {
    /// The simplified unit type.
    type Out: Unit;
    /// Convert this quantity to its simplified unit.
    fn simplify(self) -> Quantity<Self::Out>;
}

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

impl<U: Unit> Simplify for Quantity<Per<U, U>> {
    type Out = Unitless;
    /// ```rust
    /// use unit_core::length::Meters;
    /// use unit_core::{Quantity, Simplify, Unitless};
    ///
    /// let ratio = Meters::new(1.0) / Meters::new(2.0);
    /// let unitless: Quantity<Unitless> = ratio.simplify();
    /// assert!((unitless.value() - 0.5).abs() < 1e-12);
    /// ```
    fn simplify(self) -> Quantity<Unitless> {
        Quantity::new(self.value())
    }
}

impl<N: Unit, D: Unit> Simplify for Quantity<Per<N, Per<N, D>>> {
    type Out = D;
    fn simplify(self) -> Quantity<D> {
        Quantity::new(self.value())
    }
}

impl<U: Unit> Quantity<Per<U, U>> {
    /// Arc sine of a unitless ratio.
    ///
    /// ```rust
    /// use unit_core::length::Meters;
    /// let ratio = Meters::new(1.0) / Meters::new(2.0);
    /// let angle_rad = ratio.asin();
    /// assert!((angle_rad - core::f64::consts::FRAC_PI_6).abs() < 1e-12);
    /// ```
    #[inline]
    pub fn asin(&self) -> f64 {
        #[cfg(feature = "std")]
        {
            self.value().asin()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::asin(self.value())
        }
    }
}

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

/// Generate a **unit type** and its [`Display`] implementation.
///
/// This macro is provided for backward compatibility. New code should prefer
/// using the `#[derive(Unit)]` procedural macro from `unit-derive`.
///
/// Note: This macro is intended for use *inside* `unit-core`. The expansion
/// includes an `impl Display for unit_core::Quantity<...>`, which downstream
/// crates cannot compile due to Rust’s orphan rules.
///
/// # Example
///
/// ```rust,ignore
/// use unit_core::{define_unit, Unit, Quantity, Dimension};
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
