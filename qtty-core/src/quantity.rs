//! Quantity type and its implementations.

use crate::unit::{Per, Unit};
use core::marker::PhantomData;
use core::ops::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A quantity with a specific unit.
///
/// `Quantity<U>` wraps an `f64` value together with phantom type information
/// about its unit `U`. This enables compile-time dimensional analysis while
/// maintaining zero runtime cost.
///
/// # Examples
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
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Quantity<U: Unit>(f64, PhantomData<U>);

impl<U: Unit + Copy> Quantity<U> {
    /// A constant representing NaN for this quantity type.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    /// assert!(Meters::NAN.value().is_nan());
    /// ```
    pub const NAN: Self = Self::new(f64::NAN);

    /// Creates a new quantity with the given value.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
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
    /// use qtty_core::time::Seconds;
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
    /// use qtty_core::angular::Degrees;
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
    pub const fn to<T: Unit<Dim = U::Dim>>(self) -> Quantity<T> {
        Quantity::<T>::new(self.0 * (U::RATIO / T::RATIO))
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
    pub const fn min(&self, other: Quantity<U>) -> Quantity<U> {
        Quantity::<U>::new(self.value().min(other.value()))
    }

    /// Const addition of two quantities.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
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
    /// use qtty_core::length::Meters;
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
    /// use qtty_core::length::Meters;
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
    /// use qtty_core::length::Meters;
    /// let a = Meters::new(3.0);
    /// let b = Meters::new(4.0);
    /// assert_eq!(a.mul(b).value(), 12.0);
    /// ```
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
// Special methods for Per<U, U> (unitless ratios)
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit> Quantity<Per<U, U>> {
    /// Arc sine of a unitless ratio.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
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

// ─────────────────────────────────────────────────────────────────────────────
// Serde support
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
#[cfg(feature = "serde")]
pub mod serde_with_unit {
    use super::*;
    use serde::de::{self, Deserializer, MapAccess, Visitor};
    use serde::ser::{SerializeStruct, Serializer};

    /// Serializes a `Quantity<U>` as a struct with `value` and `unit` fields.
    ///
    /// # Example JSON Output
    /// ```json
    /// {"value": 42.5, "unit": "m"}
    /// ```
    pub fn serialize<U, S>(quantity: &Quantity<U>, serializer: S) -> Result<S::Ok, S::Error>
    where
        U: Unit,
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Quantity", 2)?;
        state.serialize_field("value", &quantity.value())?;
        state.serialize_field("unit", U::SYMBOL)?;
        state.end()
    }

    /// Deserializes a `Quantity<U>` from a struct with `value` and optionally `unit` fields.
    ///
    /// The `unit` field is validated if present but not required for backwards compatibility.
    /// If provided and doesn't match `U::SYMBOL`, a warning could be logged in the future.
    pub fn deserialize<'de, U, D>(deserializer: D) -> Result<Quantity<U>, D::Error>
    where
        U: Unit,
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Value,
            Unit,
        }

        struct QuantityVisitor<U>(core::marker::PhantomData<U>);

        impl<'de, U: Unit> Visitor<'de> for QuantityVisitor<U> {
            type Value = Quantity<U>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct Quantity with value and unit fields")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Quantity<U>, V::Error>
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

                Ok(Quantity::new(value))
            }
        }

        deserializer.deserialize_struct(
            "Quantity",
            &["value", "unit"],
            QuantityVisitor(core::marker::PhantomData),
        )
    }
}
