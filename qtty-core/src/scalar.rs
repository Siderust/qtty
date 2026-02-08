//! Scalar traits for quantity values.
//!
//! This module defines the trait hierarchy for numeric types that can be used as the
//! underlying storage for [`Quantity`](crate::Quantity). The traits are sealed to prevent
//! external implementations, ensuring consistent behavior across the crate.
//!
//! # Trait Hierarchy
//!
//! ```text
//! Scalar (basic arithmetic, copy, partial ordering)
//!    │
//!    ├── Real (floating-point-like: from_f64, to_f64, constants like PI)
//!    │      │
//!    │      └── Transcendental (sin, cos, sqrt, etc. - requires std or libm)
//!    │
//!    └── Exact (rational/decimal types - no floating-only ops)
//! ```
//!
//! # Supported Scalar Types
//!
//! **Always available:**
//! - `f64` (default) - implements `Scalar`, `Real`, `Transcendental`
//! - `f32` - implements `Scalar`, `Real`, `Transcendental`
//!
//! **Feature-gated:**
//! - `rust_decimal::Decimal` (`scalar-decimal`) - implements `Scalar`, `Real`
//! - `num_rational::Rational64` (`scalar-rational`) - implements `Scalar`, `Exact`
//! - `num_rational::Rational32` (`scalar-rational`) - implements `Scalar`, `Exact`
//!
//! Note: `BigRational` is NOT supported because `BigInt` does not implement `Copy`,
//! which is required by the `Scalar` trait for performance and ergonomics.
//!
//! # Example
//!
//! ```rust
//! use qtty_core::scalar::{Scalar, Real};
//!
//! fn print_half<S: Real>(value: S) {
//!     let half = S::from_f64(0.5);
//!     println!("Half of {:?} is {:?}", value.to_f64(), (value * half).to_f64());
//! }
//! ```

use core::fmt::{Debug, Display};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

// ─────────────────────────────────────────────────────────────────────────────
// Sealed trait pattern
// ─────────────────────────────────────────────────────────────────────────────

mod private {
    pub trait Sealed {}

    impl Sealed for f64 {}
    impl Sealed for f32 {}
    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for i128 {}

    #[cfg(feature = "scalar-decimal")]
    impl Sealed for rust_decimal::Decimal {}

    #[cfg(feature = "scalar-rational")]
    impl Sealed for num_rational::Rational64 {}

    #[cfg(feature = "scalar-rational")]
    impl Sealed for num_rational::Rational32 {}
}

// ─────────────────────────────────────────────────────────────────────────────
// Core Scalar trait
// ─────────────────────────────────────────────────────────────────────────────

/// The base trait for all scalar types usable in [`Quantity`](crate::Quantity).
///
/// This trait provides the minimal requirements for a numeric type to be used
/// as the underlying storage for quantities: basic arithmetic operations,
/// copy semantics, and partial ordering.
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait Scalar:
    private::Sealed
    + Copy
    + Clone
    + Debug
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Neg<Output = Self>
    + Sized
    + 'static
{
    /// The zero value for this scalar type.
    const ZERO: Self;

    /// The one value for this scalar type.
    const ONE: Self;

    /// Returns the absolute value.
    fn abs(self) -> Self;

    /// Returns the minimum of two values.
    fn min(self, other: Self) -> Self;

    /// Returns the maximum of two values.
    fn max(self, other: Self) -> Self;

    /// Returns the remainder after division.
    fn rem_euclid(self, rhs: Self) -> Self;
}

// ─────────────────────────────────────────────────────────────────────────────
// Real trait (floating-point-like operations)
// ─────────────────────────────────────────────────────────────────────────────

/// Trait for scalar types that support real-number operations.
///
/// This extends [`Scalar`] with the ability to convert to/from `f64` and access
/// common mathematical constants. Types implementing this trait can be used in
/// operations that require floating-point-like behavior.
///
/// # Conversion Guarantees
///
/// - `from_f64` should produce a reasonable approximation when exact representation
///   is not possible (e.g., for `Decimal` or `f32`).
/// - `to_f64` should produce the closest `f64` representation.
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait Real: Scalar + Display + Rem<Output = Self> {
    /// The mathematical constant π (pi).
    const PI: Self;

    /// The mathematical constant τ (tau = 2π).
    const TAU: Self;

    /// The mathematical constant e (Euler's number).
    const E: Self;

    /// Positive infinity, if supported by the type.
    const INFINITY: Self;

    /// Negative infinity, if supported by the type.
    const NEG_INFINITY: Self;

    /// Not-a-Number value, if supported by the type.
    const NAN: Self;

    /// Convert from `f64`.
    ///
    /// Returns the closest representable value in this type.
    fn from_f64(value: f64) -> Self;

    /// Convert to `f64`.
    ///
    /// Returns the closest `f64` representation.
    fn to_f64(self) -> f64;

    /// Returns the sign of the number.
    ///
    /// - `1.0` if positive
    /// - `-1.0` if negative
    /// - `NaN` if NaN
    fn signum(self) -> Self;

    /// Returns true if this value is NaN.
    fn is_nan(self) -> bool;

    /// Returns true if this value is infinite.
    fn is_infinite(self) -> bool;

    /// Returns true if this value is finite (not infinite and not NaN).
    fn is_finite(self) -> bool;

    /// Fused multiply-add: `self * a + b` with only one rounding error.
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Returns the largest integer less than or equal to self.
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to self.
    fn ceil(self) -> Self;

    /// Returns the nearest integer to self.
    fn round(self) -> Self;

    /// Returns the integer part of self.
    fn trunc(self) -> Self;

    /// Returns the fractional part of self.
    fn fract(self) -> Self;

    /// Raises self to the power of exp.
    fn powf(self, exp: Self) -> Self;

    /// Raises self to an integer power.
    fn powi(self, exp: i32) -> Self;

    /// Returns the square root.
    fn sqrt(self) -> Self;

    /// Returns the cube root.
    fn cbrt(self) -> Self;

    /// Returns the natural logarithm.
    fn ln(self) -> Self;

    /// Returns the base-10 logarithm.
    fn log10(self) -> Self;

    /// Returns the base-2 logarithm.
    fn log2(self) -> Self;

    /// Returns the logarithm with arbitrary base.
    fn log(self, base: Self) -> Self;

    /// Returns e^self.
    fn exp(self) -> Self;

    /// Returns 2^self.
    fn exp2(self) -> Self;

    /// Computes the length of the hypotenuse: sqrt(self² + other²).
    fn hypot(self, other: Self) -> Self;
}

// ─────────────────────────────────────────────────────────────────────────────
// Transcendental trait (trigonometric functions)
// ─────────────────────────────────────────────────────────────────────────────

/// Trait for scalar types that support transcendental (trigonometric) functions.
///
/// This extends [`Real`] with trigonometric and hyperbolic functions. When `std`
/// is not available, these functions are provided via `libm`.
///
/// # Note
///
/// Exact numeric types like `Decimal` or `Rational` typically do not implement
/// this trait because trigonometric functions produce irrational results.
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait Transcendental: Real {
    /// Sine function.
    fn sin(self) -> Self;

    /// Cosine function.
    fn cos(self) -> Self;

    /// Tangent function.
    fn tan(self) -> Self;

    /// Sine and cosine computed together for efficiency.
    fn sin_cos(self) -> (Self, Self);

    /// Arc sine (inverse sine).
    fn asin(self) -> Self;

    /// Arc cosine (inverse cosine).
    fn acos(self) -> Self;

    /// Arc tangent (inverse tangent).
    fn atan(self) -> Self;

    /// Arc tangent of y/x, with correct quadrant.
    fn atan2(self, other: Self) -> Self;

    /// Hyperbolic sine.
    fn sinh(self) -> Self;

    /// Hyperbolic cosine.
    fn cosh(self) -> Self;

    /// Hyperbolic tangent.
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine.
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine.
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent.
    fn atanh(self) -> Self;
}

// ─────────────────────────────────────────────────────────────────────────────
// Exact trait (for rational/decimal types)
// ─────────────────────────────────────────────────────────────────────────────

/// Trait for exact numeric types that avoid floating-point rounding.
///
/// Types implementing this trait (e.g., `Rational64`, `Decimal`, signed integers)
/// provide exact arithmetic but typically do not support transcendental functions.
///
/// The `to_f64_approx` and `from_f64_approx` methods enable lossy unit conversion
/// for types that cannot implement [`Real`]. For integers, `from_f64_approx`
/// truncates toward zero.
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait Exact: Scalar {
    /// Convert to `f64`, potentially losing precision.
    ///
    /// For integers larger than 2^53, this will lose least-significant bits.
    fn to_f64_approx(self) -> f64;

    /// Convert from `f64`, truncating toward zero.
    ///
    /// For integers, this is equivalent to `value as Self` (truncation + saturation).
    fn from_f64_approx(value: f64) -> Self;
}

/// Marker trait for integer scalar types.
///
/// This is implemented only by signed integer types (`i8`, `i16`, `i32`, `i64`, `i128`).
/// It is used to provide non-overlapping `Display` implementations for integer quantities,
/// since `Decimal` implements both [`Real`] and [`Exact`].
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait IntegerScalar: Exact + Display {}

// ─────────────────────────────────────────────────────────────────────────────
// f64 implementations
// ─────────────────────────────────────────────────────────────────────────────

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    #[inline]
    fn abs(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::abs(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fabs(self)
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::min(self, other)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fmin(self, other)
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::max(self, other)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fmax(self, other)
        }
    }

    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::rem_euclid(self, rhs)
        }
        #[cfg(not(feature = "std"))]
        {
            let r = libm::fmod(self, rhs);
            if r < 0.0 {
                r + rhs
            } else {
                r
            }
        }
    }
}

impl Real for f64 {
    const PI: Self = core::f64::consts::PI;
    const TAU: Self = core::f64::consts::TAU;
    const E: Self = core::f64::consts::E;
    const INFINITY: Self = f64::INFINITY;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
    const NAN: Self = f64::NAN;

    #[inline]
    fn from_f64(value: f64) -> Self {
        value
    }

    #[inline]
    fn to_f64(self) -> f64 {
        self
    }

    #[inline]
    fn signum(self) -> Self {
        f64::signum(self)
    }

    #[inline]
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }

    #[inline]
    fn is_infinite(self) -> bool {
        f64::is_infinite(self)
    }

    #[inline]
    fn is_finite(self) -> bool {
        f64::is_finite(self)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::mul_add(self, a, b)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fma(self, a, b)
        }
    }

    #[inline]
    fn floor(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::floor(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::floor(self)
        }
    }

    #[inline]
    fn ceil(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::ceil(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::ceil(self)
        }
    }

    #[inline]
    fn round(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::round(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::round(self)
        }
    }

    #[inline]
    fn trunc(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::trunc(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::trunc(self)
        }
    }

    #[inline]
    fn fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn powf(self, exp: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::powf(self, exp)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::pow(self, exp)
        }
    }

    #[inline]
    fn powi(self, exp: i32) -> Self {
        #[cfg(feature = "std")]
        {
            f64::powi(self, exp)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::pow(self, exp as f64)
        }
    }

    #[inline]
    fn sqrt(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::sqrt(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sqrt(self)
        }
    }

    #[inline]
    fn cbrt(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::cbrt(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::cbrt(self)
        }
    }

    #[inline]
    fn ln(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::ln(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::log(self)
        }
    }

    #[inline]
    fn log10(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::log10(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::log10(self)
        }
    }

    #[inline]
    fn log2(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::log2(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::log2(self)
        }
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::log(self, base)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::log(self) / libm::log(base)
        }
    }

    #[inline]
    fn exp(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::exp(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::exp(self)
        }
    }

    #[inline]
    fn exp2(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::exp2(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::exp2(self)
        }
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::hypot(self, other)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::hypot(self, other)
        }
    }
}

impl Transcendental for f64 {
    #[inline]
    fn sin(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::sin(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sin(self)
        }
    }

    #[inline]
    fn cos(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::cos(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::cos(self)
        }
    }

    #[inline]
    fn tan(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::tan(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::tan(self)
        }
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        #[cfg(feature = "std")]
        {
            f64::sin_cos(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sincos(self)
        }
    }

    #[inline]
    fn asin(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::asin(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::asin(self)
        }
    }

    #[inline]
    fn acos(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::acos(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::acos(self)
        }
    }

    #[inline]
    fn atan(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::atan(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::atan(self)
        }
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::atan2(self, other)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::atan2(self, other)
        }
    }

    #[inline]
    fn sinh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::sinh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sinh(self)
        }
    }

    #[inline]
    fn cosh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::cosh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::cosh(self)
        }
    }

    #[inline]
    fn tanh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::tanh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::tanh(self)
        }
    }

    #[inline]
    fn asinh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::asinh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::asinh(self)
        }
    }

    #[inline]
    fn acosh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::acosh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::acosh(self)
        }
    }

    #[inline]
    fn atanh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::atanh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::atanh(self)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// f32 implementations
// ─────────────────────────────────────────────────────────────────────────────

impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    #[inline]
    fn abs(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::abs(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fabsf(self)
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::min(self, other)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fminf(self, other)
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::max(self, other)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fmaxf(self, other)
        }
    }

    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::rem_euclid(self, rhs)
        }
        #[cfg(not(feature = "std"))]
        {
            let r = libm::fmodf(self, rhs);
            if r < 0.0 {
                r + rhs
            } else {
                r
            }
        }
    }
}

impl Real for f32 {
    const PI: Self = core::f32::consts::PI;
    const TAU: Self = core::f32::consts::TAU;
    const E: Self = core::f32::consts::E;
    const INFINITY: Self = f32::INFINITY;
    const NEG_INFINITY: Self = f32::NEG_INFINITY;
    const NAN: Self = f32::NAN;

    #[inline]
    fn from_f64(value: f64) -> Self {
        value as f32
    }

    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }

    #[inline]
    fn signum(self) -> Self {
        f32::signum(self)
    }

    #[inline]
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }

    #[inline]
    fn is_infinite(self) -> bool {
        f32::is_infinite(self)
    }

    #[inline]
    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::mul_add(self, a, b)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fmaf(self, a, b)
        }
    }

    #[inline]
    fn floor(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::floor(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::floorf(self)
        }
    }

    #[inline]
    fn ceil(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::ceil(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::ceilf(self)
        }
    }

    #[inline]
    fn round(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::round(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::roundf(self)
        }
    }

    #[inline]
    fn trunc(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::trunc(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::truncf(self)
        }
    }

    #[inline]
    fn fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn powf(self, exp: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::powf(self, exp)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::powf(self, exp)
        }
    }

    #[inline]
    fn powi(self, exp: i32) -> Self {
        #[cfg(feature = "std")]
        {
            f32::powi(self, exp)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::powf(self, exp as f32)
        }
    }

    #[inline]
    fn sqrt(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::sqrt(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sqrtf(self)
        }
    }

    #[inline]
    fn cbrt(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::cbrt(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::cbrtf(self)
        }
    }

    #[inline]
    fn ln(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::ln(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::logf(self)
        }
    }

    #[inline]
    fn log10(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::log10(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::log10f(self)
        }
    }

    #[inline]
    fn log2(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::log2(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::log2f(self)
        }
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::log(self, base)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::logf(self) / libm::logf(base)
        }
    }

    #[inline]
    fn exp(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::exp(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::expf(self)
        }
    }

    #[inline]
    fn exp2(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::exp2(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::exp2f(self)
        }
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::hypot(self, other)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::hypotf(self, other)
        }
    }
}

impl Transcendental for f32 {
    #[inline]
    fn sin(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::sin(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sinf(self)
        }
    }

    #[inline]
    fn cos(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::cos(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::cosf(self)
        }
    }

    #[inline]
    fn tan(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::tan(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::tanf(self)
        }
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        #[cfg(feature = "std")]
        {
            f32::sin_cos(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sincosf(self)
        }
    }

    #[inline]
    fn asin(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::asin(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::asinf(self)
        }
    }

    #[inline]
    fn acos(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::acos(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::acosf(self)
        }
    }

    #[inline]
    fn atan(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::atan(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::atanf(self)
        }
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::atan2(self, other)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::atan2f(self, other)
        }
    }

    #[inline]
    fn sinh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::sinh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sinhf(self)
        }
    }

    #[inline]
    fn cosh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::cosh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::coshf(self)
        }
    }

    #[inline]
    fn tanh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::tanh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::tanhf(self)
        }
    }

    #[inline]
    fn asinh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::asinh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::asinhf(self)
        }
    }

    #[inline]
    fn acosh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::acosh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::acoshf(self)
        }
    }

    #[inline]
    fn atanh(self) -> Self {
        #[cfg(feature = "std")]
        {
            f32::atanh(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::atanhf(self)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Decimal implementation (feature-gated)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "scalar-decimal")]
mod decimal_impl {
    use super::*;
    use rust_decimal::Decimal;

    impl Scalar for Decimal {
        const ZERO: Self = Decimal::ZERO;
        const ONE: Self = Decimal::ONE;

        #[inline]
        fn abs(self) -> Self {
            Decimal::abs(&self)
        }

        #[inline]
        fn min(self, other: Self) -> Self {
            Decimal::min(self, other)
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            Decimal::max(self, other)
        }

        #[inline]
        fn rem_euclid(self, rhs: Self) -> Self {
            let r = self % rhs;
            if r < Decimal::ZERO {
                r + rhs.abs()
            } else {
                r
            }
        }
    }

    impl Exact for Decimal {
        #[inline]
        fn to_f64_approx(self) -> f64 {
            use rust_decimal::prelude::ToPrimitive;
            ToPrimitive::to_f64(&self).unwrap_or(0.0)
        }

        #[inline]
        fn from_f64_approx(value: f64) -> Self {
            Decimal::try_from(value).unwrap_or(Decimal::ZERO)
        }
    }

    // Note: Decimal implements a limited Real interface.
    // Transcendental functions are not available.
    impl Real for Decimal {
        const PI: Self = Decimal::PI;
        const TAU: Self = Decimal::TWO_PI;
        const E: Self = Decimal::E;
        // Decimal doesn't have infinity/NaN, use MAX/MIN as approximations
        const INFINITY: Self = Decimal::MAX;
        const NEG_INFINITY: Self = Decimal::MIN;
        const NAN: Self = Decimal::ZERO; // No NaN representation

        #[inline]
        fn from_f64(value: f64) -> Self {
            Decimal::try_from(value).unwrap_or(Decimal::ZERO)
        }

        #[inline]
        fn to_f64(self) -> f64 {
            use rust_decimal::prelude::ToPrimitive;
            ToPrimitive::to_f64(&self).unwrap_or(0.0)
        }

        #[inline]
        fn signum(self) -> Self {
            if self > Decimal::ZERO {
                Decimal::ONE
            } else if self < Decimal::ZERO {
                Decimal::NEGATIVE_ONE
            } else {
                Decimal::ZERO
            }
        }

        #[inline]
        fn is_nan(self) -> bool {
            false // Decimal has no NaN
        }

        #[inline]
        fn is_infinite(self) -> bool {
            false // Decimal has no infinity
        }

        #[inline]
        fn is_finite(self) -> bool {
            true // Decimal is always finite
        }

        #[inline]
        fn mul_add(self, a: Self, b: Self) -> Self {
            self * a + b
        }

        #[inline]
        fn floor(self) -> Self {
            Decimal::floor(&self)
        }

        #[inline]
        fn ceil(self) -> Self {
            Decimal::ceil(&self)
        }

        #[inline]
        fn round(self) -> Self {
            Decimal::round(&self)
        }

        #[inline]
        fn trunc(self) -> Self {
            Decimal::trunc(&self)
        }

        #[inline]
        fn fract(self) -> Self {
            Decimal::fract(&self)
        }

        #[inline]
        fn powf(self, exp: Self) -> Self {
            // Decimal::powd may panic for non-integer exponents
            // Fall back to conversion through f64
            Self::from_f64(self.to_f64().powf(exp.to_f64()))
        }

        #[inline]
        fn powi(self, exp: i32) -> Self {
            use rust_decimal::MathematicalOps;
            MathematicalOps::powi(&self, exp as i64)
        }

        #[inline]
        fn sqrt(self) -> Self {
            use rust_decimal::MathematicalOps;
            MathematicalOps::sqrt(&self).unwrap_or(Decimal::ZERO)
        }

        #[inline]
        fn cbrt(self) -> Self {
            // No native cbrt, use powf
            Self::from_f64(self.to_f64().cbrt())
        }

        #[inline]
        fn ln(self) -> Self {
            use rust_decimal::MathematicalOps;
            MathematicalOps::ln(&self)
        }

        #[inline]
        fn log10(self) -> Self {
            use rust_decimal::MathematicalOps;
            MathematicalOps::log10(&self)
        }

        #[inline]
        fn log2(self) -> Self {
            use rust_decimal::MathematicalOps;
            // No native log2, compute as ln(self) / ln(2)
            MathematicalOps::ln(&self) / MathematicalOps::ln(&Decimal::TWO)
        }

        #[inline]
        fn log(self, base: Self) -> Self {
            use rust_decimal::MathematicalOps;
            MathematicalOps::ln(&self) / MathematicalOps::ln(&base)
        }

        #[inline]
        fn exp(self) -> Self {
            use rust_decimal::MathematicalOps;
            MathematicalOps::exp(&self)
        }

        #[inline]
        fn exp2(self) -> Self {
            use rust_decimal::MathematicalOps;
            // 2^self = exp(self * ln(2))
            MathematicalOps::exp(&(self * MathematicalOps::ln(&Decimal::TWO)))
        }

        #[inline]
        fn hypot(self, other: Self) -> Self {
            (self * self + other * other).sqrt()
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Rational implementations (feature-gated)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "scalar-rational")]
mod rational_impl {
    use super::*;
    use num_rational::{Rational32, Rational64};

    impl Scalar for Rational64 {
        const ZERO: Self = Rational64::new_raw(0, 1);
        const ONE: Self = Rational64::new_raw(1, 1);

        #[inline]
        fn abs(self) -> Self {
            if self < Self::ZERO {
                -self
            } else {
                self
            }
        }

        #[inline]
        fn min(self, other: Self) -> Self {
            if self < other {
                self
            } else {
                other
            }
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            if self > other {
                self
            } else {
                other
            }
        }

        #[inline]
        fn rem_euclid(self, rhs: Self) -> Self {
            let r = self - (self / rhs).trunc() * rhs;
            if r < Self::ZERO {
                r + rhs.abs()
            } else {
                r
            }
        }
    }

    impl Exact for Rational64 {
        #[inline]
        fn to_f64_approx(self) -> f64 {
            *self.numer() as f64 / *self.denom() as f64
        }

        #[inline]
        fn from_f64_approx(value: f64) -> Self {
            Rational64::approximate_float(value).unwrap_or(Rational64::new_raw(0, 1))
        }
    }

    impl Scalar for Rational32 {
        const ZERO: Self = Rational32::new_raw(0, 1);
        const ONE: Self = Rational32::new_raw(1, 1);

        #[inline]
        fn abs(self) -> Self {
            if self < Self::ZERO {
                -self
            } else {
                self
            }
        }

        #[inline]
        fn min(self, other: Self) -> Self {
            if self < other {
                self
            } else {
                other
            }
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            if self > other {
                self
            } else {
                other
            }
        }

        #[inline]
        fn rem_euclid(self, rhs: Self) -> Self {
            let r = self - (self / rhs).trunc() * rhs;
            if r < Self::ZERO {
                r + rhs.abs()
            } else {
                r
            }
        }
    }

    impl Exact for Rational32 {
        #[inline]
        fn to_f64_approx(self) -> f64 {
            *self.numer() as f64 / *self.denom() as f64
        }

        #[inline]
        fn from_f64_approx(value: f64) -> Self {
            Rational32::approximate_float(value).unwrap_or(Rational32::new_raw(0, 1))
        }
    }
}

// NOTE: BigRational (Ratio<BigInt>) is NOT supported because BigInt does not implement Copy,
// which is required by the Scalar trait. Supporting arbitrary-precision rationals would require
// a different design using Clone instead of Copy.

// ─────────────────────────────────────────────────────────────────────────────
// Signed integer implementations
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! impl_scalar_for_signed_int {
    ($($t:ty),*) => { $(
        impl Scalar for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline]
            fn min(self, other: Self) -> Self {
                Ord::min(self, other)
            }

            #[inline]
            fn max(self, other: Self) -> Self {
                Ord::max(self, other)
            }

            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
        }

        impl Exact for $t {
            #[inline]
            fn to_f64_approx(self) -> f64 {
                self as f64
            }

            #[inline]
            fn from_f64_approx(value: f64) -> Self {
                value as Self
            }
        }

        impl IntegerScalar for $t {}
    )* };
}

impl_scalar_for_signed_int!(i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_scalar_basic() {
        assert_eq!(f64::ZERO, 0.0);
        assert_eq!(f64::ONE, 1.0);
        assert_eq!((-5.0_f64).abs(), 5.0);
        assert_eq!(3.0_f64.min(5.0), 3.0);
        assert_eq!(3.0_f64.max(5.0), 5.0);
    }

    #[test]
    fn test_f64_real() {
        assert!((f64::PI - core::f64::consts::PI).abs() < 1e-15);
        assert_eq!(f64::from_f64(42.5), 42.5);
        assert_eq!(42.5_f64.to_f64(), 42.5);
        assert!(f64::NAN.is_nan());
        assert!(f64::INFINITY.is_infinite());
    }

    #[test]
    fn test_f64_transcendental() {
        let angle = core::f64::consts::FRAC_PI_2;
        assert!((angle.sin() - 1.0).abs() < 1e-15);
        assert!(angle.cos().abs() < 1e-15);
    }

    #[test]
    fn test_f32_scalar_basic() {
        assert_eq!(f32::ZERO, 0.0);
        assert_eq!(f32::ONE, 1.0);
        assert_eq!((-5.0_f32).abs(), 5.0);
    }

    #[test]
    fn test_f32_real() {
        assert!((f32::PI - core::f32::consts::PI).abs() < 1e-6);
        assert_eq!(f32::from_f64(42.5), 42.5);
    }

    #[test]
    fn test_f32_transcendental() {
        let angle = core::f32::consts::FRAC_PI_2;
        assert!((angle.sin() - 1.0).abs() < 1e-6);
    }

    // ── Integer scalar tests ──────────────────────────────────────────────

    #[test]
    fn test_i32_scalar_basic() {
        assert_eq!(i32::ZERO, 0);
        assert_eq!(i32::ONE, 1);
        assert_eq!((-5_i32).abs(), 5);
        assert_eq!(Scalar::min(3_i32, 5), 3);
        assert_eq!(Scalar::max(3_i32, 5), 5);
        assert_eq!(7_i32.rem_euclid(4), 3);
        assert_eq!((-7_i32).rem_euclid(4), 1);
    }

    #[test]
    fn test_i64_scalar_basic() {
        assert_eq!(i64::ZERO, 0);
        assert_eq!(i64::ONE, 1);
        assert_eq!((-100_i64).abs(), 100);
        assert_eq!(Scalar::min(10_i64, 20), 10);
        assert_eq!(Scalar::max(10_i64, 20), 20);
    }

    #[test]
    fn test_i8_scalar_basic() {
        assert_eq!(i8::ZERO, 0);
        assert_eq!(i8::ONE, 1);
        assert_eq!((-5_i8).abs(), 5);
        assert_eq!(Scalar::max(127_i8, -128), 127);
    }

    #[test]
    fn test_i16_scalar_basic() {
        assert_eq!(i16::ZERO, 0);
        assert_eq!(i16::ONE, 1);
        assert_eq!((-1000_i16).abs(), 1000);
    }

    #[test]
    fn test_i128_scalar_basic() {
        assert_eq!(i128::ZERO, 0);
        assert_eq!(i128::ONE, 1);
        assert_eq!((-42_i128).abs(), 42);
    }

    #[test]
    fn test_integer_exact_conversions() {
        // i32
        assert_eq!(42_i32.to_f64_approx(), 42.0);
        assert_eq!(i32::from_f64_approx(42.9), 42); // truncates toward zero
        assert_eq!(i32::from_f64_approx(-3.7), -3); // truncates toward zero

        // i64
        assert_eq!(1000_i64.to_f64_approx(), 1000.0);
        assert_eq!(i64::from_f64_approx(1500.0), 1500);

        // i8
        assert_eq!(i8::from_f64_approx(100.0), 100);
    }
}
