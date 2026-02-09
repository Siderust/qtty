//! Quantity type and its implementations.

use crate::dimension::{DimDiv, DimMul, Dimension};
use crate::scalar::{Exact, Real, Scalar, Transcendental};
use crate::unit::{Per, Prod, Unit};
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::*;

/// A quantity with a specific unit and scalar type.
///
/// `Quantity<U, S>` wraps a scalar value of type `S` together with phantom type
/// information about its unit `U`. This enables compile-time dimensional analysis
/// while maintaining zero runtime cost beyond the scalar's size.
///
/// The default scalar type is `f64`, so `Quantity<Meter>` is equivalent to
/// `Quantity<Meter, f64>`.
///
/// # Examples
///
/// Basic usage with default `f64`:
///
/// ```rust
/// use qtty_core::length::{Meter, Meters};
/// use qtty_core::Quantity;
///
/// let x = Meters::new(5.0);
/// let y = Meters::new(3.0);
/// let sum = x + y;
/// assert_eq!(sum.value(), 8.0);
/// ```
///
/// Using `f32` for memory efficiency:
///
/// ```rust
/// use qtty_core::length::Meter;
/// use qtty_core::Quantity;
///
/// let x: Quantity<Meter, f32> = Quantity::new(5.0_f32);
/// assert_eq!(x.value(), 5.0_f32);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quantity<U: Unit, S: Scalar = f64>(S, PhantomData<U>);

// ─────────────────────────────────────────────────────────────────────────────
// Type aliases for common scalar types
// ─────────────────────────────────────────────────────────────────────────────

/// A quantity backed by `f64` (the default).
pub type Quantity64<U> = Quantity<U, f64>;

/// A quantity backed by `f32`.
pub type Quantity32<U> = Quantity<U, f32>;

/// A quantity backed by `rust_decimal::Decimal`.
#[cfg(feature = "scalar-decimal")]
pub type QuantityDecimal<U> = Quantity<U, rust_decimal::Decimal>;

/// A quantity backed by `num_rational::Rational64`.
#[cfg(feature = "scalar-rational")]
pub type QuantityRational<U> = Quantity<U, num_rational::Rational64>;

/// A quantity backed by `i8`.
pub type QuantityI8<U> = Quantity<U, i8>;

/// A quantity backed by `i16`.
pub type QuantityI16<U> = Quantity<U, i16>;

/// A quantity backed by `i32`.
pub type QuantityI32<U> = Quantity<U, i32>;

/// A quantity backed by `i64`.
pub type QuantityI64<U> = Quantity<U, i64>;

/// A quantity backed by `i128`.
pub type QuantityI128<U> = Quantity<U, i128>;

// ─────────────────────────────────────────────────────────────────────────────
// Core implementation for all Scalar types
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit, S: Scalar> Quantity<U, S> {
    /// Creates a new quantity with the given value.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    /// let d = Meters::new(3.0);
    /// assert_eq!(d.value(), 3.0);
    /// ```
    #[inline]
    pub const fn new(value: S) -> Self {
        Self(value, PhantomData)
    }

    /// Returns the raw numeric value.
    ///
    /// ```rust
    /// use qtty_core::time::Seconds;
    /// let t = Seconds::new(2.5);
    /// assert_eq!(t.value(), 2.5);
    /// ```
    #[inline]
    pub const fn value(self) -> S {
        self.0
    }

    /// Returns a reference to the raw numeric value.
    #[inline]
    pub const fn value_ref(&self) -> &S {
        &self.0
    }

    /// Returns the absolute value.
    ///
    /// ```rust
    /// use qtty_core::angular::Degrees;
    /// let a = Degrees::new(-10.0);
    /// assert_eq!(a.abs().value(), 10.0);
    /// ```
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.0.abs())
    }

    /// Returns the minimum of this quantity and another.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    /// let a = Meters::new(3.0);
    /// let b = Meters::new(5.0);
    /// assert_eq!(a.min(b).value(), 3.0);
    /// ```
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self::new(self.0.min(other.0))
    }

    /// Returns the maximum of this quantity and another.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self::new(self.0.max(other.0))
    }

    /// Returns the arithmetic mean (midpoint) of this quantity and another.
    ///
    /// For integer-backed quantities this uses integer division semantics
    /// (truncation toward zero).
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    /// let a = Meters::new(10.0);
    /// let b = Meters::new(14.0);
    /// assert_eq!(a.mean(b).value(), 12.0);
    /// ```
    #[inline]
    pub fn mean(self, other: Self) -> Self {
        Self::new((self.0 + other.0) / (S::ONE + S::ONE))
    }

    /// A constant representing the zero value for this quantity type.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(S::ZERO)
    }

    /// A constant representing the unit value (one) for this quantity type.
    #[inline]
    pub const fn one() -> Self {
        Self::new(S::ONE)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Real-specific implementations (f32, f64, Decimal, etc.)
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit, S: Real> Quantity<U, S> {
    /// A constant representing NaN for this quantity type.
    ///
    /// Note: For types without NaN (like `Decimal`), this may not be a true NaN.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    /// assert!(Meters::NAN.value().is_nan());
    /// ```
    pub const NAN: Self = Self(S::NAN, PhantomData);

    /// A constant representing positive infinity.
    pub const INFINITY: Self = Self(S::INFINITY, PhantomData);

    /// A constant representing negative infinity.
    pub const NEG_INFINITY: Self = Self(S::NEG_INFINITY, PhantomData);

    /// Returns true if the value is NaN.
    #[inline]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Returns true if the value is infinite.
    #[inline]
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    /// Returns true if the value is finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Converts this quantity to another unit of the same dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty_core::length::{Meter, Kilometer, Kilometers};
    /// use qtty_core::Quantity;
    ///
    /// let km = Kilometers::new(1.0);
    /// let m: Quantity<Meter> = km.to();
    /// assert_eq!(m.value(), 1000.0);
    /// ```
    #[inline]
    pub fn to<T: Unit<Dim = U::Dim>>(self) -> Quantity<T, S> {
        let ratio = S::from_f64(U::RATIO / T::RATIO);
        Quantity::<T, S>::new(self.0 * ratio)
    }

    /// Convert the scalar type while preserving the unit.
    ///
    /// This converts via `f64`, so precision may be lost for types with
    /// higher precision than `f64`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty_core::length::{Meter, Meters};
    /// use qtty_core::Quantity;
    ///
    /// let meters_f64 = Meters::new(100.0);
    /// let meters_f32: Quantity<Meter, f32> = meters_f64.cast();
    /// assert_eq!(meters_f32.value(), 100.0_f32);
    /// ```
    #[inline]
    pub fn cast<T: Real>(self) -> Quantity<U, T> {
        Quantity::new(T::from_f64(self.0.to_f64()))
    }

    /// Sign of the value.
    #[inline]
    pub fn signum(self) -> S {
        self.0.signum()
    }

    /// Returns the square root.
    ///
    /// Note: This returns the scalar square root of the value. The resulting
    /// quantity still has the same unit type, which may not be physically
    /// meaningful in all contexts.
    #[inline]
    pub fn sqrt(self) -> Self {
        Self::new(self.0.sqrt())
    }

    /// Returns the smallest integer quantity greater than or equal to this value.
    #[inline]
    pub fn ceil(self) -> Self {
        Self::new(self.0.ceil())
    }

    /// Checks equality with a quantity of a different unit in the same dimension.
    ///
    /// The `other` quantity is converted to unit `U` before comparison.
    /// Note that floating-point conversion may introduce rounding; for exact
    /// equality checks consider converting both to a common unit first and using
    /// an epsilon tolerance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty_core::length::{Kilometers, Meters};
    ///
    /// let km = Kilometers::new(1.0);
    /// let m = Meters::new(1000.0);
    /// assert!(km.eq_unit(&m));
    /// ```
    #[inline]
    pub fn eq_unit<V: Unit<Dim = U::Dim>>(self, other: &Quantity<V, S>) -> bool {
        self.0 == other.to::<U>().value()
    }

    /// Compares with a quantity of a different unit in the same dimension.
    ///
    /// The `other` quantity is converted to unit `U` before comparison.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty_core::length::{Kilometers, Meters};
    /// use core::cmp::Ordering;
    ///
    /// let km = Kilometers::new(2.0);
    /// let m = Meters::new(500.0);
    /// assert_eq!(km.cmp_unit(&m), Some(Ordering::Greater));
    /// ```
    #[inline]
    pub fn cmp_unit<V: Unit<Dim = U::Dim>>(self, other: &Quantity<V, S>) -> Option<Ordering> {
        self.0.partial_cmp(&other.to::<U>().value())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Exact-specific implementations (integers, rationals, etc.)
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit, S: Exact> Quantity<U, S> {
    /// Converts this quantity to another unit of the same dimension (lossy).
    ///
    /// For integer scalars this performs the conversion through `f64` intermediate
    /// arithmetic, then truncates back to the integer type. The result may lose
    /// precision due to truncation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty_core::Quantity;
    /// use qtty_core::length::{Meter, Kilometer};
    ///
    /// let m: Quantity<Meter, i32> = Quantity::new(1500);
    /// let km: Quantity<Kilometer, i32> = m.to_lossy();
    /// assert_eq!(km.value(), 1); // truncated from 1.5
    /// ```
    #[inline]
    pub fn to_lossy<T: Unit<Dim = U::Dim>>(self) -> Quantity<T, S> {
        let value_f64 = self.0.to_f64_approx();
        let ratio = U::RATIO / T::RATIO;
        Quantity::<T, S>::new(S::from_f64_approx(value_f64 * ratio))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Const methods for f64 (backward compatibility)
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit + Copy> Quantity<U, f64> {
    /// Const addition of two quantities.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    /// let a = Meters::new(1.0);
    /// let b = Meters::new(2.0);
    /// assert_eq!(a.const_add(b).value(), 3.0);
    /// ```
    #[inline]
    pub const fn const_add(self, other: Self) -> Self {
        Self(self.0 + other.0, PhantomData)
    }

    /// Const subtraction of two quantities.
    #[inline]
    pub const fn const_sub(self, other: Self) -> Self {
        Self(self.0 - other.0, PhantomData)
    }

    /// Const multiplication by a scalar.
    #[inline]
    pub const fn const_mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs, PhantomData)
    }

    /// Const division by a scalar.
    #[inline]
    pub const fn const_div(self, rhs: f64) -> Self {
        Self(self.0 / rhs, PhantomData)
    }

    /// Const conversion to another unit.
    #[inline]
    pub const fn to_const<T: Unit<Dim = U::Dim> + Copy>(self) -> Quantity<T, f64> {
        Quantity::<T, f64>(self.0 * (U::RATIO / T::RATIO), PhantomData)
    }

    /// Const min of two quantities.
    #[inline]
    pub const fn min_const(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Const max of two quantities.
    #[inline]
    pub const fn max_const(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Const methods for f32
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit + Copy> Quantity<U, f32> {
    /// Const addition of two quantities.
    #[inline]
    pub const fn const_add(self, other: Self) -> Self {
        Self(self.0 + other.0, PhantomData)
    }

    /// Const subtraction of two quantities.
    #[inline]
    pub const fn const_sub(self, other: Self) -> Self {
        Self(self.0 - other.0, PhantomData)
    }

    /// Const multiplication by a scalar.
    #[inline]
    pub const fn const_mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, PhantomData)
    }

    /// Const division by a scalar.
    #[inline]
    pub const fn const_div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, PhantomData)
    }

    /// Const conversion to another unit.
    #[inline]
    pub const fn to_const<T: Unit<Dim = U::Dim> + Copy>(self) -> Quantity<T, f32> {
        Quantity::<T, f32>(self.0 * (U::RATIO as f32 / T::RATIO as f32), PhantomData)
    }

    /// Const min of two quantities.
    #[inline]
    pub const fn min_const(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Const max of two quantities.
    #[inline]
    pub const fn max_const(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Const methods for signed integer types
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! impl_const_for_int {
    ($($t:ty),*) => { $(
        impl<U: Unit + Copy> Quantity<U, $t> {
            /// Const addition of two quantities.
            #[inline]
            pub const fn const_add(self, other: Self) -> Self {
                Self(self.0 + other.0, PhantomData)
            }

            /// Const subtraction of two quantities.
            #[inline]
            pub const fn const_sub(self, other: Self) -> Self {
                Self(self.0 - other.0, PhantomData)
            }

            /// Const multiplication by a scalar.
            #[inline]
            pub const fn const_mul(self, rhs: $t) -> Self {
                Self(self.0 * rhs, PhantomData)
            }

            /// Const division by a scalar.
            #[inline]
            pub const fn const_div(self, rhs: $t) -> Self {
                Self(self.0 / rhs, PhantomData)
            }

            /// Const min of two quantities.
            #[inline]
            pub const fn min_const(self, other: Self) -> Self {
                if self.0 < other.0 {
                    self
                } else {
                    other
                }
            }

            /// Const max of two quantities.
            #[inline]
            pub const fn max_const(self, other: Self) -> Self {
                if self.0 > other.0 {
                    self
                } else {
                    other
                }
            }
        }
    )* };
}

impl_const_for_int!(i8, i16, i32, i64, i128);

// ─────────────────────────────────────────────────────────────────────────────
// Operator implementations
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit, S: Scalar> Add for Quantity<U, S> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

impl<U: Unit, S: Scalar> AddAssign for Quantity<U, S> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<U: Unit, S: Scalar> Sub for Quantity<U, S> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

impl<U: Unit, S: Scalar> SubAssign for Quantity<U, S> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<U: Unit, S: Scalar> Mul<S> for Quantity<U, S> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: S) -> Self {
        Self::new(self.0 * rhs)
    }
}

impl<U: Unit, S: Scalar> Div<S> for Quantity<U, S> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: S) -> Self {
        Self::new(self.0 / rhs)
    }
}

impl<U: Unit, S: Scalar> DivAssign<Self> for Quantity<U, S> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<U: Unit, S: Scalar> Neg for Quantity<U, S> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.0)
    }
}

// Multiplication of f64 * Quantity<U, f64>
impl<U: Unit> Mul<Quantity<U, f64>> for f64 {
    type Output = Quantity<U, f64>;
    #[inline]
    fn mul(self, rhs: Quantity<U, f64>) -> Self::Output {
        rhs * self
    }
}

// Multiplication of f32 * Quantity<U, f32>
impl<U: Unit> Mul<Quantity<U, f32>> for f32 {
    type Output = Quantity<U, f32>;
    #[inline]
    fn mul(self, rhs: Quantity<U, f32>) -> Self::Output {
        rhs * self
    }
}

// Multiplication for Decimal (feature-gated)
#[cfg(feature = "scalar-decimal")]
impl<U: Unit> Mul<Quantity<U, rust_decimal::Decimal>> for rust_decimal::Decimal {
    type Output = Quantity<U, rust_decimal::Decimal>;
    #[inline]
    fn mul(self, rhs: Quantity<U, rust_decimal::Decimal>) -> Self::Output {
        rhs * self
    }
}

// Multiplication for Rational64 (feature-gated)
#[cfg(feature = "scalar-rational")]
impl<U: Unit> Mul<Quantity<U, num_rational::Rational64>> for num_rational::Rational64 {
    type Output = Quantity<U, num_rational::Rational64>;
    #[inline]
    fn mul(self, rhs: Quantity<U, num_rational::Rational64>) -> Self::Output {
        rhs * self
    }
}

// Multiplication for Rational32 (feature-gated)
#[cfg(feature = "scalar-rational")]
impl<U: Unit> Mul<Quantity<U, num_rational::Rational32>> for num_rational::Rational32 {
    type Output = Quantity<U, num_rational::Rational32>;
    #[inline]
    fn mul(self, rhs: Quantity<U, num_rational::Rational32>) -> Self::Output {
        rhs * self
    }
}

// Commutative multiplication for signed integer scalars
macro_rules! impl_int_commutative_mul {
    ($($t:ty),*) => { $(
        impl<U: Unit> Mul<Quantity<U, $t>> for $t {
            type Output = Quantity<U, $t>;
            #[inline]
            fn mul(self, rhs: Quantity<U, $t>) -> Self::Output {
                rhs * self
            }
        }
    )* };
}

impl_int_commutative_mul!(i8, i16, i32, i64, i128);

// Rem for types that implement Rem (floats and integers)
impl<U: Unit, S: Scalar + Rem<Output = S>> Rem<S> for Quantity<U, S> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: S) -> Self {
        Self::new(self.0 % rhs)
    }
}

// PartialEq with scalar
impl<U: Unit, S: Scalar> PartialEq<S> for Quantity<U, S> {
    #[inline]
    fn eq(&self, other: &S) -> bool {
        self.0 == *other
    }
}

// PartialOrd with scalar
impl<U: Unit, S: Scalar> PartialOrd<S> for Quantity<U, S> {
    #[inline]
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

// PartialOrd between quantities of the same unit/scalar.
impl<U: Unit, S: Scalar> PartialOrd for Quantity<U, S> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

// Eq for scalar types that support total equality (integers, rationals, decimals)
impl<U: Unit, S: Scalar + Eq> Eq for Quantity<U, S> {}

// Ord for scalar types that support total ordering (integers, rationals, decimals)
impl<U: Unit, S: Scalar + Ord> Ord for Quantity<U, S> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

// From scalar
impl<U: Unit, S: Scalar> From<S> for Quantity<U, S> {
    #[inline]
    fn from(value: S) -> Self {
        Self::new(value)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Division producing Per<N, D>
// ─────────────────────────────────────────────────────────────────────────────

impl<N: Unit, D: Unit, S: Scalar> Div<Quantity<D, S>> for Quantity<N, S>
where
    N::Dim: DimDiv<D::Dim>,
    <N::Dim as DimDiv<D::Dim>>::Output: Dimension,
{
    type Output = Quantity<Per<N, D>, S>;
    #[inline]
    fn div(self, rhs: Quantity<D, S>) -> Self::Output {
        Quantity::new(self.0 / rhs.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Multiplication producing Prod<A, B>
// ─────────────────────────────────────────────────────────────────────────────

impl<A: Unit, B: Unit, S: Scalar> Mul<Quantity<B, S>> for Quantity<A, S>
where
    A::Dim: DimMul<B::Dim>,
    <A::Dim as DimMul<B::Dim>>::Output: Dimension,
{
    type Output = Quantity<Prod<A, B>, S>;

    #[inline]
    fn mul(self, rhs: Quantity<B, S>) -> Self::Output {
        Quantity::<Prod<A, B>, S>::new(self.0 * rhs.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Special methods for Per<U, U> (unitless ratios)
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit, S: Transcendental> Quantity<Per<U, U>, S>
where
    U::Dim: DimDiv<U::Dim>,
    <U::Dim as DimDiv<U::Dim>>::Output: Dimension,
{
    /// Arc sine of a unitless ratio.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    /// let ratio = Meters::new(1.0) / Meters::new(2.0);
    /// let angle_rad = ratio.asin();
    /// assert!((angle_rad - core::f64::consts::FRAC_PI_6).abs() < 1e-12);
    /// ```
    #[inline]
    pub fn asin(&self) -> S {
        self.0.asin()
    }

    /// Arc cosine of a unitless ratio.
    #[inline]
    pub fn acos(&self) -> S {
        self.0.acos()
    }

    /// Arc tangent of a unitless ratio.
    #[inline]
    pub fn atan(&self) -> S {
        self.0.atan()
    }
}
