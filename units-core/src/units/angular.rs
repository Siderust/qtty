//! Angular quantities and utilities.
//!
//! This module defines the **`Angular` dimension**, a blanket [`AngularUnit`] trait that extends
//! [`Unit`] for all angular units, common angular units (degrees, radians, arcseconds, etc.), and a set of
//! convenience methods on [`Quantity<U>`] where `U: AngularUnit`.
//!
//! # Design overview
//!
//! * **Canonical unit:** Degrees are taken as the canonical *scaling* unit for this dimension. That is,
//!   `Degree::RATIO == 1.0`, and all other angular units express how many *degrees* correspond to one of that unit.
//!   For example, `Radian::RATIO == 180.0 / PI` because 1 radian = 180/π degrees.
//! * **Associated constants:** The `AngularUnit` trait exposes precomputed constants (`FULL_TURN`, `HALF_TURN`,
//!   `QUARTED_TURN`) expressed *in the receiving unit* for ergonomic range‑wrapping.
//! * **Trigonometry:** `sin`, `cos`, `tan`, and `sin_cos` methods are provided on angular quantities; they convert to
//!   radians internally and then call the corresponding `f64` intrinsic.
//! * **Wrapping helpers:** Utility methods to wrap any angle into common ranges — `[0, 360)` (or unit equivalent),
//!   `(-180, 180]`, and the latitude‑style quarter fold `[-90, 90]`.
//!
//! ## Examples
//!
//! Convert between degrees and radians and evaluate a trig function:
//!
//! ```rust,ignore
//! use crate::angular::{Degrees, Radians};
//! use crate::Quantity;
//!
//! let angle: Degrees = Degrees::new(90.0);
//! let r: Radians = angle.to();
//! assert!((r.value() - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
//! assert!((angle.sin() - 1.0).abs() < 1e-12);
//! ```
//!
//! Wrap into the conventional signed range:
//!
//! ```rust,ignore
//! use crate::angular::Degrees;
//! let a = Degrees::new(370.0).wrap_signed();
//! assert_eq!(a.value(), 10.0);
//! ```

use crate::{Dimension, Quantity, Unit};
use units_derive::Unit;
use std::f64::consts::TAU;

/// Dimension tag for angular measures (e.g., degrees, radians, arcseconds).
pub enum Angular {}
impl Dimension for Angular {}

/// Blanket extension trait for any [`Unit`] whose dimension is [`Angular`].
///
/// These associated constants provide the size of key turn fractions *expressed in the implementing unit*.
/// They are computed using a compiletime conversion from `TAU` radians (i.e., a full revolution).
///
/// > **Naming note:** The historical spelling `QUARTED_TURN` is retained for backward compatibility. It represents a
/// > quarter turn (90°).
pub trait AngularUnit: Unit<Dim = Angular> {
    const FULL_TURN: f64;
    const HALF_TURN: f64;
    const QUARTED_TURN: f64;
}
impl<T: Unit<Dim = Angular>> AngularUnit for T {
    /// One full revolution (360°) expressed in T unit.
    const FULL_TURN: f64 = Radians::new(TAU).to::<T>().value();
    /// Half a revolution (180°) expressed in T unit.
    const HALF_TURN: f64 = Radians::new(TAU).to::<T>().value() * 0.5;
    /// Quarter revolution (90°) expressed in T unit.
    const QUARTED_TURN: f64 = Radians::new(TAU).to::<T>().value() * 0.25;
}

impl<U: AngularUnit + Copy> Quantity<U> {
    /// Constant representing τ radians (2π rad == 360°).
    pub const TAU: Quantity<U> = Quantity::<U>::new(U::FULL_TURN);
    /// One full revolution (360°) expressed in Quantity<T> unit.
    pub const FULL_TURN: Quantity<U> = Quantity::<U>::new(U::FULL_TURN);
    /// Half a revolution (180°) expressed in Quantity<T> unit.
    pub const HALF_TURN: Quantity<U> = Quantity::<U>::new(U::HALF_TURN);
    /// Quarter revolution (90°) expressed in Quantity<T> unit.
    pub const QUARTED_TURN: Quantity<U> = Quantity::<U>::new(U::QUARTED_TURN);

    /// Sine of the angle.
    #[inline]
    pub fn sin(&self) -> f64 {
        self.to::<Radian>().value().sin()
    }

    /// Cosine of the angle.
    #[inline]
    pub fn cos(&self) -> f64 {
        self.to::<Radian>().value().cos()
    }

    /// Tangent of the angle.
    #[inline]
    pub fn tan(&self) -> f64 {
        self.to::<Radian>().value().tan()
    }

    /// Simultaneously compute sine and cosine.
    #[inline]
    pub fn sin_cos(&self) -> (f64, f64) {
        self.to::<Radian>().value().sin_cos()
    }

    /// Sign of the *raw numeric* in this unit (same semantics as `f64::signum()`).
    #[inline]
    pub const fn signum(self) -> f64 {
        self.value().signum()
    }

    /// Normalize into the canonical positive range `[0, FULL_TURN)`.
    ///
    /// Shorthand for [`Self::wrap_pos`].
    #[inline]
    pub fn normalize(self) -> Self {
        self.wrap_pos()
    }

    /// Wrap into the positive range `[0, FULL_TURN)` using Euclidean remainder.
    #[inline]
    pub fn wrap_pos(self) -> Self {
        Self::new(self.value().rem_euclid(U::FULL_TURN))
    }

    /// Wrap into the signed range `(-HALF_TURN, HALF_TURN]`.
    ///
    /// *Upper bound is inclusive*; lower bound is exclusive. Useful for computing minimal signed angular differences.
    #[inline]
    pub fn wrap_signed(self) -> Self {
        let full = U::FULL_TURN;
        let half = 0.5 * full;
        let x = self.value();
        let y = (x + half).rem_euclid(full) - half;
        let norm = if y <= -half { y + full } else { y };
        Self::new(norm)
    }

    /// Wrap into the alternate signed range `[-HALF_TURN, HALF_TURN)`.
    ///
    /// Lower bound inclusive; upper bound exclusive. Equivalent to `self.wrap_signed()` with the boundary flipped.
    #[inline]
    pub fn wrap_signed_lo(self) -> Self {
        let mut y = self.wrap_signed().value(); // now in (-half, half]
        let half = 0.5 * U::FULL_TURN;
        if y > half {
            // move +half to -half
            y -= U::FULL_TURN;
        }
        Self::new(y)
    }

    /// "Latitude fold": map into `[-QUARTER_TURN, +QUARTER_TURN]`.
    ///
    /// Useful for folding polar coordinates (e.g., converting declination‑like angles to a limited range).
    #[inline]
    pub fn wrap_quarter_fold(self) -> Self {
        let full = U::FULL_TURN;
        let half = 0.5 * full;
        let quarter = 0.25 * full;
        let y = (self.value() + quarter).rem_euclid(full);
        // quarter - |y - half| yields [-quarter, quarter]
        Self::new(quarter - (y - half).abs())
    }

    /// Signed smallest angular separation in `(-HALF_TURN, HALF_TURN]`.
    #[inline]
    pub fn signed_separation(self, other: Self) -> Self {
        (self - other).wrap_signed()
    }

    /// Absolute smallest angular separation (magnitude only).
    #[inline]
    pub fn abs_separation(self, other: Self) -> Self {
        let sep = self.signed_separation(other);
        Self::new(sep.value().abs())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Deg", dimension = Angular, ratio = 1.0)]
pub struct Degree;
/// Type alias shorthand for [`Degree`].
pub type Deg = Degree;
/// Convenience alias for a degree quantity.
pub type Degrees = Quantity<Deg>;
/// One degree.
pub const DEG: Degrees = Degrees::new(1.0);

// NOTE: 1 rad = 180/π degrees.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Rad", dimension = Angular, ratio = 180.0 / 3.141592653589793)]
pub struct Radian;
/// Type alias shorthand for [`Radian`].
pub type Rad = Radian;
/// Convenience alias for a radian quantity.
pub type Radians = Quantity<Rad>;
/// One radian.
pub const RAD: Radians = Radians::new(1.0);

// NOTE: 1 arcsecond = 1/3600 degree.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Arcs", dimension = Angular, ratio = 1.0 / 3600.0)]
pub struct Arcsecond;
/// Type alias shorthand for [`Arcsecond`].
pub type Arcs = Arcsecond;
/// Convenience alias for an arcsecond quantity.
pub type Arcseconds = Quantity<Arcs>;
/// One arcsecond.
pub const ARCS: Arcseconds = Arcseconds::new(1.0);

// NOTE: 1 milliarcsecond = 1/3_600_000 degree.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Mas", dimension = Angular, ratio = 1.0 / 3_600_000.0)]
pub struct MilliArcsecond;
/// Type alias shorthand for [`MilliArcsecond`].
pub type Mas = MilliArcsecond;
/// Convenience alias for a milliarcsecond quantity.
pub type MilliArcseconds = Quantity<Mas>;
/// One milliarcsecond.
pub const MAS: MilliArcseconds = MilliArcseconds::new(1.0);

// NOTE: 1 hour angle = 15 degrees.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Hms", dimension = Angular, ratio = 15.0)]
pub struct HourAngle;
/// Type alias shorthand for [`HourAngle`].
pub type Hms = HourAngle;
/// Convenience alias for an hour-angle quantity.
pub type HourAngles = Quantity<Hms>;
/// One hour angle hour (==15°).
pub const HOUR_ANGLE: HourAngles = HourAngles::new(1.0);

impl HourAngles {
    /// Construct from **HMS** components (`hours`, `minutes`, `seconds`).
    ///
    /// Sign is taken from `hours`; the `minutes` and `seconds` parameters are treated as magnitudes.
    ///
    /// ```rust,ignore
    /// use crate::angular::HourAngles;
    /// let ra = HourAngles::from_hms(5, 30, 0.0); // 5h30m == 5.5h
    /// assert_eq!(ra.value(), 5.5);
    /// ```
    pub const fn from_hms(hours: i32, minutes: u32, seconds: f64) -> Self {
        let sign = if hours < 0 { -1.0 } else { 1.0 };
        let h_abs = if hours < 0 { -hours } else { hours } as f64;
        let m = minutes as f64 / 60.0;
        let s = seconds / 3600.0;
        let total_hours = sign * (h_abs + m + s);
        Self::new(total_hours)
    }
}

impl Degrees {
    /// Construct from **DMS** components (`deg`, `min`, `sec`).
    ///
    /// Sign is taken from `deg`; the magnitude of `min` and `sec` is always added.
    /// No range checking is performed. Use one of the wrapping helpers if you need a canonical range.
    ///
    /// ```rust,ignore
    /// use crate::angular::Degrees;
    /// let lat = Degrees::from_dms(-33, 52, 0.0); // −33°52′00″
    /// assert!(lat.value() < 0.0);
    /// ```
    pub const fn from_dms(deg: i32, min: u32, sec: f64) -> Self {
        let sign = if deg < 0 { -1.0 } else { 1.0 };
        let d_abs = if deg < 0 { -deg } else { deg } as f64;
        let m = min as f64 / 60.0;
        let s = sec / 3600.0;
        let total = sign * (d_abs + m + s);
        Self::new(total)
    }

    /// Construct from explicit sign and magnitude components.
    ///
    /// `sign` should be −1, 0, or +1 (0 treated as +1 unless all components are zero).
    pub const fn from_dms_sign(sign: i8, deg: u32, min: u32, sec: f64) -> Self {
        let s = if sign < 0 { -1.0 } else { 1.0 };
        let total = (deg as f64) + (min as f64) / 60.0 + (sec / 3600.0);
        Self::new(s * total)
    }
}

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        deg.to::<Radian>()
    }
}
impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        rad.to::<Degree>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{PI, TAU};
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Angular unit constants
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_full_turn() {
        assert_abs_diff_eq!(Radian::FULL_TURN, TAU, epsilon = 1e-12);
        assert_eq!(Degree::FULL_TURN, 360.0);
        assert_eq!(Arcsecond::FULL_TURN, 1_296_000.0);
    }

    #[test]
    fn test_half_turn() {
        assert_abs_diff_eq!(Radian::HALF_TURN, PI, epsilon = 1e-12);
        assert_eq!(Degree::HALF_TURN, 180.0);
        assert_eq!(Arcsecond::HALF_TURN, 648_000.0);
    }

    #[test]
    fn test_quarter_turn() {
        assert_abs_diff_eq!(Radian::QUARTED_TURN, PI / 2.0, epsilon = 1e-12);
        assert_eq!(Degree::QUARTED_TURN, 90.0);
        assert_eq!(Arcsecond::QUARTED_TURN, 324_000.0);
    }

    #[test]
    fn test_quantity_constants() {
        assert_eq!(Degrees::FULL_TURN.value(), 360.0);
        assert_eq!(Degrees::HALF_TURN.value(), 180.0);
        assert_eq!(Degrees::QUARTED_TURN.value(), 90.0);
        assert_eq!(Degrees::TAU.value(), 360.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn conversion_degrees_to_radians() {
        let deg = Degrees::new(180.0);
        let rad = deg.to::<Radian>();
        assert_abs_diff_eq!(rad.value(), PI, epsilon = 1e-12);
    }

    #[test]
    fn conversion_radians_to_degrees() {
        let rad = Radians::new(PI);
        let deg = rad.to::<Degree>();
        assert_abs_diff_eq!(deg.value(), 180.0, epsilon = 1e-12);
    }

    #[test]
    fn conversion_degrees_to_arcseconds() {
        let deg = Degrees::new(1.0);
        let arcs = deg.to::<Arcsecond>();
        assert_abs_diff_eq!(arcs.value(), 3600.0, epsilon = 1e-9);
    }

    #[test]
    fn conversion_arcseconds_to_degrees() {
        let arcs = Arcseconds::new(3600.0);
        let deg = arcs.to::<Degree>();
        assert_abs_diff_eq!(deg.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn conversion_degrees_to_milliarcseconds() {
        let deg = Degrees::new(1.0);
        let mas = deg.to::<MilliArcsecond>();
        assert_abs_diff_eq!(mas.value(), 3_600_000.0, epsilon = 1e-6);
    }

    #[test]
    fn conversion_hour_angles_to_degrees() {
        let ha = HourAngles::new(1.0);
        let deg = ha.to::<Degree>();
        assert_abs_diff_eq!(deg.value(), 15.0, epsilon = 1e-12);
    }

    #[test]
    fn conversion_roundtrip() {
        let original = Degrees::new(123.456);
        let rad = original.to::<Radian>();
        let back = rad.to::<Degree>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-12);
    }

    #[test]
    fn from_impl_degrees_radians() {
        let deg = Degrees::new(90.0);
        let rad: Radians = deg.into();
        assert_abs_diff_eq!(rad.value(), PI / 2.0, epsilon = 1e-12);

        let rad2 = Radians::new(PI);
        let deg2: Degrees = rad2.into();
        assert_abs_diff_eq!(deg2.value(), 180.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Trig functions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_trig() {
        let a = Degrees::new(90.0);
        assert!((a.sin() - 1.0).abs() < 1e-12);
        assert!(a.cos().abs() < 1e-12);
    }

    #[test]
    fn trig_sin_known_values() {
        assert_abs_diff_eq!(Degrees::new(0.0).sin(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(30.0).sin(), 0.5, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(90.0).sin(), 1.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(180.0).sin(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(270.0).sin(), -1.0, epsilon = 1e-12);
    }

    #[test]
    fn trig_cos_known_values() {
        assert_abs_diff_eq!(Degrees::new(0.0).cos(), 1.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(60.0).cos(), 0.5, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(90.0).cos(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(180.0).cos(), -1.0, epsilon = 1e-12);
    }

    #[test]
    fn trig_tan_known_values() {
        assert_abs_diff_eq!(Degrees::new(0.0).tan(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(45.0).tan(), 1.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(180.0).tan(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn trig_sin_cos_consistency() {
        let angle = Degrees::new(37.5);
        let (sin, cos) = angle.sin_cos();
        assert_abs_diff_eq!(sin, angle.sin(), epsilon = 1e-15);
        assert_abs_diff_eq!(cos, angle.cos(), epsilon = 1e-15);
    }

    #[test]
    fn trig_pythagorean_identity() {
        let angle = Degrees::new(123.456);
        let sin = angle.sin();
        let cos = angle.cos();
        assert_abs_diff_eq!(sin * sin + cos * cos, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn trig_radians() {
        assert_abs_diff_eq!(Radians::new(0.0).sin(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Radians::new(PI / 2.0).sin(), 1.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Radians::new(PI).cos(), -1.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // signum
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn signum_positive() {
        assert_eq!(Degrees::new(45.0).signum(), 1.0);
    }

    #[test]
    fn signum_negative() {
        assert_eq!(Degrees::new(-45.0).signum(), -1.0);
    }

    #[test]
    fn signum_zero() {
        assert_eq!(Degrees::new(0.0).signum(), 1.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // wrap_pos (normalize)
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn wrap_pos_basic() {
        assert_abs_diff_eq!(Degrees::new(370.0).wrap_pos().value(), 10.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(720.0).wrap_pos().value(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(0.0).wrap_pos().value(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn wrap_pos_negative() {
        assert_abs_diff_eq!(Degrees::new(-10.0).wrap_pos().value(), 350.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(-370.0).wrap_pos().value(), 350.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(-720.0).wrap_pos().value(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn wrap_pos_boundary() {
        assert_abs_diff_eq!(Degrees::new(360.0).wrap_pos().value(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(-360.0).wrap_pos().value(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn normalize_is_wrap_pos() {
        let angle = Degrees::new(450.0);
        assert_eq!(angle.normalize().value(), angle.wrap_pos().value());
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // wrap_signed: (-180, 180]
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_wrap_signed() {
        let a = Degrees::new(370.0).wrap_signed();
        assert_eq!(a.value(), 10.0);
        let b = Degrees::new(-190.0).wrap_signed();
        assert_eq!(b.value(), 170.0);
    }

    #[test]
    fn wrap_signed_basic() {
        assert_abs_diff_eq!(Degrees::new(10.0).wrap_signed().value(), 10.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(-10.0).wrap_signed().value(), -10.0, epsilon = 1e-12);
    }

    #[test]
    fn wrap_signed_over_180() {
        assert_abs_diff_eq!(Degrees::new(190.0).wrap_signed().value(), -170.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(270.0).wrap_signed().value(), -90.0, epsilon = 1e-12);
    }

    #[test]
    fn wrap_signed_boundary_180() {
        assert_abs_diff_eq!(Degrees::new(180.0).wrap_signed().value(), 180.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(-180.0).wrap_signed().value(), 180.0, epsilon = 1e-12);
    }

    #[test]
    fn wrap_signed_large_values() {
        assert_abs_diff_eq!(Degrees::new(540.0).wrap_signed().value(), 180.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(-540.0).wrap_signed().value(), 180.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // wrap_quarter_fold: [-90, 90]
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn wrap_quarter_fold_basic() {
        assert_abs_diff_eq!(Degrees::new(0.0).wrap_quarter_fold().value(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(45.0).wrap_quarter_fold().value(), 45.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(-45.0).wrap_quarter_fold().value(), -45.0, epsilon = 1e-12);
    }

    #[test]
    fn wrap_quarter_fold_boundary() {
        assert_abs_diff_eq!(Degrees::new(90.0).wrap_quarter_fold().value(), 90.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(-90.0).wrap_quarter_fold().value(), -90.0, epsilon = 1e-12);
    }

    #[test]
    fn wrap_quarter_fold_over_90() {
        assert_abs_diff_eq!(Degrees::new(100.0).wrap_quarter_fold().value(), 80.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(135.0).wrap_quarter_fold().value(), 45.0, epsilon = 1e-12);
        assert_abs_diff_eq!(Degrees::new(180.0).wrap_quarter_fold().value(), 0.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Separation helpers
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn signed_separation_basic() {
        let a = Degrees::new(30.0);
        let b = Degrees::new(50.0);
        assert_abs_diff_eq!(a.signed_separation(b).value(), -20.0, epsilon = 1e-12);
        assert_abs_diff_eq!(b.signed_separation(a).value(), 20.0, epsilon = 1e-12);
    }

    #[test]
    fn signed_separation_wrap() {
        let a = Degrees::new(10.0);
        let b = Degrees::new(350.0);
        assert_abs_diff_eq!(a.signed_separation(b).value(), 20.0, epsilon = 1e-12);
        assert_abs_diff_eq!(b.signed_separation(a).value(), -20.0, epsilon = 1e-12);
    }

    #[test]
    fn abs_separation() {
        let a = Degrees::new(30.0);
        let b = Degrees::new(50.0);
        assert_abs_diff_eq!(a.abs_separation(b).value(), 20.0, epsilon = 1e-12);
        assert_abs_diff_eq!(b.abs_separation(a).value(), 20.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // DMS / HMS construction
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn degrees_from_dms_positive() {
        let d = Degrees::from_dms(12, 30, 0.0);
        assert_abs_diff_eq!(d.value(), 12.5, epsilon = 1e-12);
    }

    #[test]
    fn degrees_from_dms_negative() {
        let d = Degrees::from_dms(-33, 52, 0.0);
        assert!(d.value() < 0.0);
        assert_abs_diff_eq!(d.value(), -(33.0 + 52.0 / 60.0), epsilon = 1e-12);
    }

    #[test]
    fn degrees_from_dms_with_seconds() {
        let d = Degrees::from_dms(10, 20, 30.0);
        assert_abs_diff_eq!(d.value(), 10.0 + 20.0 / 60.0 + 30.0 / 3600.0, epsilon = 1e-12);
    }

    #[test]
    fn degrees_from_dms_sign() {
        let pos = Degrees::from_dms_sign(1, 45, 30, 0.0);
        let neg = Degrees::from_dms_sign(-1, 45, 30, 0.0);
        assert_abs_diff_eq!(pos.value(), 45.5, epsilon = 1e-12);
        assert_abs_diff_eq!(neg.value(), -45.5, epsilon = 1e-12);
    }

    #[test]
    fn hour_angles_from_hms() {
        let ha = HourAngles::from_hms(5, 30, 0.0);
        assert_abs_diff_eq!(ha.value(), 5.5, epsilon = 1e-12);
    }

    #[test]
    fn hour_angles_from_hms_negative() {
        let ha = HourAngles::from_hms(-3, 15, 0.0);
        assert_abs_diff_eq!(ha.value(), -3.25, epsilon = 1e-12);
    }

    #[test]
    fn hour_angles_to_degrees() {
        let ha = HourAngles::new(6.0);
        let deg = ha.to::<Degree>();
        assert_abs_diff_eq!(deg.value(), 90.0, epsilon = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display formatting
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_degrees() {
        let d = Degrees::new(45.5);
        assert_eq!(format!("{}", d), "45.5 Deg");
    }

    #[test]
    fn display_radians() {
        let r = Radians::new(1.0);
        assert_eq!(format!("{}", r), "1 Rad");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Unit constants
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unit_constants() {
        assert_eq!(DEG.value(), 1.0);
        assert_eq!(RAD.value(), 1.0);
        assert_eq!(ARCS.value(), 1.0);
        assert_eq!(MAS.value(), 1.0);
        assert_eq!(HOUR_ANGLE.value(), 1.0);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_wrap_pos_range(angle in -1e6..1e6f64) {
            let wrapped = Degrees::new(angle).wrap_pos();
            prop_assert!(wrapped.value() >= 0.0);
            prop_assert!(wrapped.value() < 360.0);
        }

        #[test]
        fn prop_wrap_signed_range(angle in -1e6..1e6f64) {
            let wrapped = Degrees::new(angle).wrap_signed();
            prop_assert!(wrapped.value() > -180.0);
            prop_assert!(wrapped.value() <= 180.0);
        }

        #[test]
        fn prop_wrap_quarter_fold_range(angle in -1e6..1e6f64) {
            let wrapped = Degrees::new(angle).wrap_quarter_fold();
            prop_assert!(wrapped.value() >= -90.0);
            prop_assert!(wrapped.value() <= 90.0);
        }

        #[test]
        fn prop_pythagorean_identity(angle in -360.0..360.0f64) {
            let a = Degrees::new(angle);
            let sin = a.sin();
            let cos = a.cos();
            assert_abs_diff_eq!(sin * sin + cos * cos, 1.0, epsilon = 1e-12);
        }

        #[test]
        fn prop_conversion_roundtrip(angle in -1e6..1e6f64) {
            let deg = Degrees::new(angle);
            let rad = deg.to::<Radian>();
            let back = rad.to::<Degree>();
            assert_relative_eq!(back.value(), deg.value(), max_relative = 1e-12);
        }

        #[test]
        fn prop_abs_separation_symmetric(a in -360.0..360.0f64, b in -360.0..360.0f64) {
            let da = Degrees::new(a);
            let db = Degrees::new(b);
            assert_abs_diff_eq!(
                da.abs_separation(db).value(),
                db.abs_separation(da).value(),
                epsilon = 1e-12
            );
        }
    }
}
