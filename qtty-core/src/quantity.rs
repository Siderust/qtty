//! Quantity type and its implementations.

use crate::scalar::{Real, Scalar, Transcendental};
use crate::unit::{Per, Unit};
use core::marker::PhantomData;
use core::ops::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
/// use qtty_core::{Quantity, Unit, Dimension};
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
///
/// Using `f32` for memory efficiency:
///
/// ```rust
/// use qtty_core::{Quantity, Unit, Dimension};
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
/// let x: Quantity<Meter, f32> = Quantity::new(5.0_f32);
/// assert_eq!(x.value(), 5.0_f32);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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
    /// use qtty_core::{Quantity, Unit, Dimension};
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

// Rem for Real types
impl<U: Unit, S: Real> Rem<S> for Quantity<U, S> {
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

impl<N: Unit, D: Unit, S: Scalar> Div<Quantity<D, S>> for Quantity<N, S> {
    type Output = Quantity<Per<N, D>, S>;
    #[inline]
    fn div(self, rhs: Quantity<D, S>) -> Self::Output {
        Quantity::new(self.0 / rhs.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Multiplication: Per<N,D> * D = N
// ─────────────────────────────────────────────────────────────────────────────

impl<N: Unit, D: Unit, S: Scalar> Mul<Quantity<D, S>> for Quantity<Per<N, D>, S> {
    type Output = Quantity<N, S>;

    #[inline]
    fn mul(self, rhs: Quantity<D, S>) -> Self::Output {
        Quantity::<N, S>::new(self.0 * rhs.0)
    }
}

impl<N: Unit, D: Unit, S: Scalar> Mul<Quantity<Per<N, D>, S>> for Quantity<D, S> {
    type Output = Quantity<N, S>;

    #[inline]
    fn mul(self, rhs: Quantity<Per<N, D>, S>) -> Self::Output {
        rhs * self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Special methods for Per<U, U> (unitless ratios)
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit, S: Transcendental> Quantity<Per<U, U>, S> {
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

// ─────────────────────────────────────────────────────────────────────────────
// Serde support
// ─────────────────────────────────────────────────────────────────────────────

// Default serde: serialize as f64 (backward compatible)
#[cfg(feature = "serde")]
impl<U: Unit, S: Real> Serialize for Quantity<U, S> {
    fn serialize<Ser>(&self, serializer: Ser) -> core::result::Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        // Strategy A: Always serialize as f64 for backward compatibility
        self.0.to_f64().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, U: Unit, S: Real> Deserialize<'de> for Quantity<U, S> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Strategy A: Deserialize from f64 and convert
        let value = f64::deserialize(deserializer)?;
        Ok(Quantity::new(S::from_f64(value)))
    }
}

/// Serde helper module for serializing the raw scalar value.
///
/// This module serializes the scalar type directly instead of converting to f64.
/// Use this when you need to preserve the exact scalar representation.
///
/// # Example
///
/// ```rust,ignore
/// use qtty_core::length::Meters;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     #[serde(with = "qtty_core::serde_scalar")]
///     distance: Meters,  // Serializes the scalar directly
/// }
/// ```
#[cfg(feature = "serde")]
pub mod serde_scalar {
    use super::*;

    /// Serializes the quantity's scalar value directly.
    #[allow(dead_code)]
    pub fn serialize<U, S, Ser>(quantity: &Quantity<U, S>, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        U: Unit,
        S: Scalar + Serialize,
        Ser: Serializer,
    {
        quantity.value_ref().serialize(serializer)
    }

    /// Deserializes the quantity from a scalar value directly.
    #[allow(dead_code)]
    pub fn deserialize<'de, U, S, D>(deserializer: D) -> Result<Quantity<U, S>, D::Error>
    where
        U: Unit,
        S: Scalar + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let value = S::deserialize(deserializer)?;
        Ok(Quantity::new(value))
    }
}

/// Serde helper module for serializing quantities with unit information.
///
/// Use this with the `#[serde(with = "...")]` attribute to preserve unit symbols
/// in serialized data. This is useful for external APIs, configuration files, or
/// self-documenting data formats.
///
/// # Examples
///
/// ```rust
/// use qtty_core::length::Meters;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     #[serde(with = "qtty_core::serde_with_unit")]
///     max_distance: Meters,  // Serializes as {"value": 100.0, "unit": "m"}
///     
///     min_distance: Meters,  // Serializes as 50.0 (default, compact)
/// }
/// ```
#[cfg(all(feature = "serde", feature = "std"))]
pub mod serde_with_unit {
    extern crate alloc;
    use alloc::format;
    use alloc::string::String;

    use super::*;
    use serde::de::{self, Deserializer, MapAccess, Visitor};
    use serde::ser::{SerializeStruct, Serializer};

    /// Serializes a `Quantity<U, S>` as a struct with `value` and `unit` fields.
    ///
    /// # Example JSON Output
    /// ```json
    /// {"value": 42.5, "unit": "m"}
    /// ```
    pub fn serialize<U, S, Ser>(quantity: &Quantity<U, S>, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        U: Unit,
        S: Real,
        Ser: Serializer,
    {
        let mut state = serializer.serialize_struct("Quantity", 2)?;
        state.serialize_field("value", &quantity.value().to_f64())?;
        state.serialize_field("unit", U::SYMBOL)?;
        state.end()
    }

    /// Deserializes a `Quantity<U, S>` from a struct with `value` and optionally `unit` fields.
    ///
    /// The `unit` field is validated if present but not required for backwards compatibility.
    /// If provided and doesn't match `U::SYMBOL`, an error is returned.
    pub fn deserialize<'de, U, S, D>(deserializer: D) -> Result<Quantity<U, S>, D::Error>
    where
        U: Unit,
        S: Real,
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Value,
            Unit,
        }

        struct QuantityVisitor<U, S>(core::marker::PhantomData<(U, S)>);

        impl<'de, U: Unit, S: Real> Visitor<'de> for QuantityVisitor<U, S> {
            type Value = Quantity<U, S>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct Quantity with value and unit fields")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Quantity<U, S>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value: Option<f64> = None;
                let mut unit: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Unit => {
                            if unit.is_some() {
                                return Err(de::Error::duplicate_field("unit"));
                            }
                            unit = Some(map.next_value()?);
                        }
                    }
                }

                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;

                // Validate unit if provided (optional for backwards compatibility)
                if let Some(ref unit_str) = unit {
                    if unit_str != U::SYMBOL {
                        return Err(de::Error::custom(format!(
                            "unit mismatch: expected '{}', found '{}'",
                            U::SYMBOL,
                            unit_str
                        )));
                    }
                }

                Ok(Quantity::new(S::from_f64(value)))
            }
        }

        deserializer.deserialize_struct(
            "Quantity",
            &["value", "unit"],
            QuantityVisitor(core::marker::PhantomData),
        )
    }
}
