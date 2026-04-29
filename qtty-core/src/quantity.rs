// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Quantity type and its implementations.

use crate::scalar::{Exact, Real, Scalar, Transcendental};
use crate::unit::Unit;
use crate::unit_arithmetic::{QuantityDivOutput, UnitDiv, UnitMul, UnitSqrt};
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::iter::Sum;
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quantity<U: Unit, S: Scalar = f64>(S, PhantomData<U>);

// ─────────────────────────────────────────────────────────────────────────────
// Type aliases for common scalar types
// ─────────────────────────────────────────────────────────────────────────────

/// A quantity backed by `f64` (the default).
pub type Quantity64<U> = Quantity<U, f64>;

/// A quantity backed by `f32`.
pub type Quantity32<U> = Quantity<U, f32>;

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

    /// Clamps this quantity to `[min_val, max_val]`.
    #[inline]
    pub fn clamp(self, min_val: Self, max_val: Self) -> Self {
        debug_assert!(
            min_val.0 <= max_val.0,
            "Quantity::clamp requires min_val <= max_val"
        );
        self.max(min_val).min(max_val)
    }

    /// Returns the arithmetic mean (midpoint) of this quantity and another.
    ///
    /// For integer-backed quantities this uses integer division semantics
    /// (truncation toward zero). The computation is overflow-safe for all
    /// scalar types, including integers at their extremes.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    /// let a = Meters::new(10.0);
    /// let b = Meters::new(14.0);
    /// assert_eq!(a.mean(b).value(), 12.0);
    /// ```
    #[inline]
    pub fn mean(self, other: Self) -> Self {
        let two = S::ONE + S::ONE;
        let a = self.0;
        let b = other.0;
        // Equal operands: the midpoint is the operand itself.
        // This also short-circuits same-sign infinities (e.g. +∞.mean(+∞)):
        // the split-half path below computes ∞ − ∞ = NaN for those cases,
        // which would violate the IEEE-754 expectation that the midpoint of
        // two identical infinities stays infinite.
        if a == b {
            return Self::new(a);
        }
        // When both values have the same sign, their sum may overflow.
        // Use a split-half formula that is safe for same-sign operands.
        // When signs differ, the direct sum never overflows (for integers
        // it stays within the type's range; for floats it is always fine).
        if (a >= S::ZERO) == (b >= S::ZERO) {
            let ha = a / two;
            let hb = b / two;
            // For ±∞: a / 2 == a (infinity halved is still infinity), so
            // `a - ha * two` would compute ∞ − ∞ = NaN.  Treat the
            // remainder as zero in that case; the half already carries the
            // full infinite magnitude.
            let ra = if ha == a { S::ZERO } else { a - ha * two };
            let rb = if hb == b { S::ZERO } else { b - hb * two };
            Self::new(ha + hb + (ra + rb) / two)
        } else {
            Self::new((a + b) / two)
        }
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

    /// Erases the unit tag, returning the raw stored scalar `S`.
    ///
    /// **This is a lossy operation**: the raw stored number is returned as-is,
    /// without any normalization to the canonical (SI) unit. Use this only
    /// when you explicitly intend to discard dimensional information, e.g.
    /// for adapter layers or debugging.
    ///
    /// For a true dimensionless ratio, divide two quantities of the same unit
    /// instead (`a / b` where both are `Quantity<U, S>`).
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty_core::length::Kilometers;
    ///
    /// let km = Kilometers::new(1.0);
    /// let raw: f64 = km.erase_unit_raw();
    /// assert_eq!(raw, 1.0); // raw stored number, NOT 1000.0
    /// ```
    #[inline]
    pub fn erase_unit_raw(self) -> S {
        self.0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Real-specific implementations (f32, f64, etc.)
// ─────────────────────────────────────────────────────────────────────────────

impl<U: Unit, S: Real> Quantity<U, S> {
    /// A constant representing NaN for this quantity type.
    ///
    /// Note: For scalar types without a NaN representation, this may be an
    /// approximation rather than a true IEEE-754 NaN.
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
    /// The conversion multiplies the scalar value by `U::RATIO / T::RATIO`.
    /// Because [`Unit::RATIO`] is always an `f64`, this ratio is computed in
    /// `f64` and then cast back to `S` via [`Scalar::from_f64`].  For
    /// floating-point scalars (`f64`, `f32`) the precision loss is negligible.
    /// For **exact scalar types** (`Rational64`, integers) the cast is lossy:
    /// the numeric value will be an `f64`-rounded approximation of the true
    /// rational ratio.  See [`Exact`] for the explicit trade-off documentation.
    ///
    /// If you need unit conversion without the `f64` round-trip, store values
    /// in the target unit from the start rather than converting after the fact.
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

    /// Returns the square root of the underlying scalar value.
    ///
    /// This returns the raw scalar `S` rather than `Quantity<U, S>`, because
    /// the square root of a dimensional quantity does not in general carry the
    /// same dimension (e.g. √(m²) = m, not m²).  If you need a quantity
    /// result, wrap it explicitly with the correct unit type.
    #[inline]
    pub fn scalar_sqrt(self) -> S {
        self.0.sqrt()
    }

    /// Returns the largest integer quantity less than or equal to this value.
    #[inline]
    pub fn floor(self) -> Self {
        Self::new(self.0.floor())
    }

    /// Returns the smallest integer quantity greater than or equal to this value.
    #[inline]
    pub fn ceil(self) -> Self {
        Self::new(self.0.ceil())
    }

    /// Returns the nearest integer quantity to this value.
    #[inline]
    pub fn round(self) -> Self {
        Self::new(self.0.round())
    }

    /// Returns the integer part of this quantity.
    #[inline]
    pub fn trunc(self) -> Self {
        Self::new(self.0.trunc())
    }

    /// Returns the fractional part of this quantity.
    #[inline]
    pub fn fract(self) -> Self {
        Self::new(self.0.fract())
    }

    /// Checks equality with a quantity of a different unit in the same dimension.
    ///
    /// Both operands are converted to the reference (SI) unit before comparison,
    /// ensuring that `a.eq_unit(&b)` and `b.eq_unit(&a)` always agree.
    ///
    /// Note that floating-point conversion may introduce rounding; for exact
    /// equality checks consider using an epsilon tolerance.
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
        // Always multiply the value in the smaller-RATIO unit by the ratio
        // (smaller/larger) ≤ 1, preventing overflow for near-MAX values while
        // keeping both `a.eq_unit(&b)` and `b.eq_unit(&a)` numerically
        // identical (preserving symmetry).
        if U::RATIO >= V::RATIO {
            self.0 == other.value() * S::from_f64(V::RATIO / U::RATIO)
        } else {
            self.0 * S::from_f64(U::RATIO / V::RATIO) == other.value()
        }
    }

    /// Compares with a quantity of a different unit in the same dimension.
    ///
    /// Both operands are converted to the reference (SI) unit before comparison,
    /// ensuring order-consistency regardless of operand direction.
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
        if U::RATIO >= V::RATIO {
            self.0
                .partial_cmp(&(other.value() * S::from_f64(V::RATIO / U::RATIO)))
        } else {
            (self.0 * S::from_f64(U::RATIO / V::RATIO)).partial_cmp(&other.value())
        }
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
    /// precision due to:
    ///
    /// - **Truncation toward zero** for fractional results (e.g. `1500 m → 1 km`).
    /// - **Saturation at integer bounds** when the converted value exceeds the
    ///   target type's range (e.g. `1 km → 127 m` for `i8`).
    ///
    /// Use [`checked_to_lossy`](Self::checked_to_lossy) if you need to detect
    /// range overflow.
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
        let ratio = U::RATIO / T::RATIO;
        // Same-ratio fast path: skip the f64 round-trip entirely.
        // Without this, large integer values (e.g. near i64::MAX) would be
        // corrupted even for identity conversions because f64 cannot represent
        // them exactly.
        if ratio == 1.0 {
            return Quantity::<T, S>::new(self.0);
        }
        let value_f64 = self.0.to_f64_approx();
        Quantity::<T, S>::new(S::from_f64_approx(value_f64 * ratio))
    }

    /// Checked lossy unit conversion.
    ///
    /// Like [`to_lossy`](Self::to_lossy), but returns `None` when the converted
    /// value would overflow the scalar type (i.e. saturation/clipping would
    /// occur). Fractional truncation toward zero is still permitted.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty_core::Quantity;
    /// use qtty_core::length::{Meter, Kilometer};
    ///
    /// let km: Quantity<Kilometer, i8> = Quantity::new(1);
    /// // 1 km = 1000 m, which doesn't fit in i8
    /// assert_eq!(km.checked_to_lossy::<Meter>(), None);
    ///
    /// let m: Quantity<Meter, i32> = Quantity::new(1500);
    /// let km: Option<Quantity<Kilometer, i32>> = m.checked_to_lossy();
    /// assert_eq!(km.unwrap().value(), 1); // truncated, but within range
    /// ```
    #[inline]
    pub fn checked_to_lossy<T: Unit<Dim = U::Dim>>(self) -> Option<Quantity<T, S>> {
        let ratio = U::RATIO / T::RATIO;
        // Same-ratio fast path: the value is unchanged, so always in range.
        // Without this, large integers near i64::MAX would round-trip through
        // f64 and either return a mutated value or a false None.
        if ratio == 1.0 {
            return Some(Quantity::<T, S>::new(self.0));
        }
        let value_f64 = self.0.to_f64_approx();
        S::checked_from_f64(value_f64 * ratio).map(Quantity::<T, S>::new)
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

    /// Returns the least non-negative remainder of `self.value() % rhs`.
    ///
    /// Equivalent to `f64::rem_euclid`: always returns a value in `[0, rhs)` for `rhs > 0`.
    /// Useful for canonicalising angular quantities to `[0°, 360°)` without extracting the inner value.
    #[inline]
    pub fn rem_euclid(self, rhs: f64) -> Self {
        Self::new(self.0.rem_euclid(rhs))
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
        Quantity::<T, f32>(self.0 * ((U::RATIO / T::RATIO) as f32), PhantomData)
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

impl<U: Unit, S: Scalar> MulAssign<S> for Quantity<U, S> {
    /// In-place scalar multiplication.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    ///
    /// let mut d = Meters::new(3.0);
    /// d *= 4.0;
    /// assert_eq!(d.value(), 12.0);
    /// ```
    ///
    /// ```compile_fail
    /// use qtty_core::length::Meters;
    ///
    /// let mut d = Meters::new(3.0);
    /// d *= Meters::new(4.0);
    /// ```
    #[inline]
    fn mul_assign(&mut self, rhs: S) {
        self.0 *= rhs;
    }
}

impl<U: Unit, S: Scalar> Div<S> for Quantity<U, S> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: S) -> Self {
        Self::new(self.0 / rhs)
    }
}

impl<U: Unit, S: Scalar> DivAssign<S> for Quantity<U, S> {
    /// In-place scalar division.
    ///
    /// ```rust
    /// use qtty_core::length::Meters;
    ///
    /// let mut d = Meters::new(120.0);
    /// d /= 60.0;
    /// assert_eq!(d.value(), 2.0);
    /// ```
    ///
    /// ```compile_fail
    /// use qtty_core::length::Meters;
    ///
    /// let mut d = Meters::new(120.0);
    /// d /= Meters::new(60.0);
    /// ```
    #[inline]
    fn div_assign(&mut self, rhs: S) {
        self.0 /= rhs;
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

// Same-unit remainder: `5 m % 3 m == 2 m`.
impl<U: Unit, S: Scalar + Rem<Output = S>> Rem for Quantity<U, S> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::new(self.0 % rhs.0)
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

// Hash for scalar types that support hashing, enabling Quantity in HashMap/HashSet.
impl<U: Unit, S: Scalar + Hash> Hash for Quantity<U, S> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

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

// Sum quantities into a quantity of the same unit/scalar.
impl<U: Unit, S: Scalar> Sum for Quantity<U, S> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, q| acc + q)
    }
}

impl<'a, U: Unit, S: Scalar> Sum<&'a Quantity<U, S>> for Quantity<U, S> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Quantity<U, S>>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, q| acc + *q)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Division delegating to UnitDiv + QuantityDivOutput
// ─────────────────────────────────────────────────────────────────────────────

// When the two units are the same (`N = D`), `UnitDiv` returns `SameDivOutput`
// and `QuantityDivOutput` maps that to `S` — the raw scalar.
// For different units, `UnitDiv` returns a composite unit and `QuantityDivOutput`
// wraps it in `Quantity<..., S>`.
impl<N: Unit, D: Unit, S: Scalar> Div<Quantity<D, S>> for Quantity<N, S>
where
    N: UnitDiv<D>,
    <N as UnitDiv<D>>::Output: QuantityDivOutput<S>,
{
    type Output = <<N as UnitDiv<D>>::Output as QuantityDivOutput<S>>::Output;
    #[inline]
    fn div(self, rhs: Quantity<D, S>) -> Self::Output {
        <<N as UnitDiv<D>>::Output as QuantityDivOutput<S>>::wrap(self.0 / rhs.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Multiplication delegating to UnitMul
// ─────────────────────────────────────────────────────────────────────────────

impl<A: Unit, B: Unit, S: Scalar> Mul<Quantity<B, S>> for Quantity<A, S>
where
    A: UnitMul<B>,
{
    type Output = Quantity<<A as UnitMul<B>>::Output, S>;

    #[inline]
    fn mul(self, rhs: Quantity<B, S>) -> Self::Output {
        Quantity::new(self.0 * rhs.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Trig helpers for dimensionless quantities
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// Dimensionally-typed square root
// ─────────────────────────────────────────────────────────────────────────────

impl<U, S> Quantity<U, S>
where
    U: UnitSqrt,
    S: Real,
{
    /// Dimensionally-typed square root.
    ///
    /// For any squared unit `U = Prod<R, R>` (e.g. `SquareMeter ≡ Prod<Meter, Meter>`),
    /// this returns a `Quantity<R, S>` whose value is the scalar square root.
    ///
    /// This is the inverse of the `Quantity<R> * Quantity<R> -> Quantity<Prod<R, R>>`
    /// multiplication produced by [`UnitMul`].
    ///
    /// Only nonneg values make dimensional sense; for negative scalars the
    /// returned quantity carries `S::sqrt(neg)` (typically NaN for real
    /// scalars). Callers that need explicit signed-safety should branch on
    /// [`signum`](Self::signum) first.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qtty_core::area::SquareMeters;
    /// use qtty_core::length::{Meter, Meters};
    /// use qtty_core::Quantity;
    ///
    /// let area = SquareMeters::new(25.0);
    /// let side: Quantity<Meter> = area.sqrt();
    /// assert!((side.value() - 5.0).abs() < 1e-12);
    /// ```
    #[inline]
    pub fn sqrt(self) -> Quantity<U::Root, S> {
        Quantity::new(self.0.sqrt())
    }
}

impl<U, S> Quantity<U, S>
where
    U: Unit<Dim = crate::dimension::Dimensionless>,
    S: Transcendental,
{
    /// Arc sine returning a typed angle in radians.
    ///
    /// Applicable to any quantity whose dimension is `Dimensionless`, such as
    /// `Quantity<Per<Meter, Meter>, f64>` — the result of dividing different
    /// length units that share the same dimension.
    ///
    /// ```rust
    /// use qtty_core::angular::{Degree, Radian};
    /// use qtty_core::length::{Kilometer, Meter, Kilometers, Meters};
    /// use qtty_core::{Per, Quantity};
    ///
    /// // Cross-unit ratio: 1 km / 2000 m  =  0.5  (dimensionless)
    /// let km_per_m: Quantity<Per<Kilometer, Meter>> = Quantity::new(0.5);
    /// let angle: Quantity<Radian> = km_per_m.asin_angle();
    /// assert!((angle.value() - 0.5_f64.asin()).abs() < 1e-12);
    ///
    /// // Convert to degrees:
    /// let deg: Quantity<Degree> = angle.to();
    /// assert!((deg.value() - 30.0).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn asin_angle(&self) -> Quantity<crate::units::angular::Radian, S> {
        Quantity::new(self.0.asin())
    }

    /// Arc cosine returning a typed angle in radians.
    #[inline]
    pub fn acos_angle(&self) -> Quantity<crate::units::angular::Radian, S> {
        Quantity::new(self.0.acos())
    }

    /// Arc tangent returning a typed angle in radians.
    #[inline]
    pub fn atan_angle(&self) -> Quantity<crate::units::angular::Radian, S> {
        Quantity::new(self.0.atan())
    }
}
