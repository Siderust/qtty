// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Solid-angle units.
//!
//! Solid angle is plane angle squared (`A²`). The canonical scaling unit for
//! the [`Angular`](crate::dimension::Angular) dimension is the **degree**
//! (`Degree::RATIO == 1.0`), so the canonical scaling unit for the
//! [`SolidAngle`](crate::dimension::SolidAngle) dimension is the
//! **square degree** (`SquareDegree::RATIO == 1.0`).
//!
//! All other solid-angle units are expressed as exact ratios to square
//! degrees through the `Prod`-based composition. In particular,
//! `Steradian::RATIO == (180/π)² ≈ 3282.806…`, since `1 sr ≈ 3282.806 deg²`.
//!
//! Solid-angle units arise *directly* from multiplying two angular
//! quantities — no conversion needed:
//!
//! ```rust
//! use qtty_core::angular::Degrees;
//! use qtty_core::solid_angle::SquareDegrees;
//!
//! let theta = Degrees::new(2.0);
//! let omega: SquareDegrees = theta * theta;
//! assert!((omega.value() - 4.0).abs() < 1e-12);
//! ```
//!
//! ```rust
//! use qtty_core::angular::Radians;
//! use qtty_core::solid_angle::Steradians;
//!
//! let r = Radians::new(1.0);
//! let omega: Steradians = r * r;
//! assert!((omega.value() - 1.0).abs() < 1e-12);
//! ```

use crate::units::angular::{Degree, Milliradian, Radian};
use crate::{Prod, Quantity, Unit};

#[cfg(feature = "astro")]
use crate::units::angular::{Arcminute, Arcsecond};

/// Re-export the solid-angle dimension from the dimension module.
pub use crate::dimension::SolidAngle;

/// Marker trait for any [`Unit`] whose dimension is [`SolidAngle`].
pub trait SolidAngleUnit: Unit<Dim = SolidAngle> {}
impl<T: Unit<Dim = SolidAngle>> SolidAngleUnit for T {}

/// A composed solid-angle quantity from squaring an angular unit.
///
/// `SolidAngleOf<A>` is `Quantity<Prod<A, A>>`. Since the named solid-angle
/// types are themselves `Prod` aliases, `SolidAngleOf<Radian>` and
/// [`Steradians`] are the **same type**.
///
/// # Examples
///
/// ```rust
/// use qtty_core::angular::{Degree, Degrees};
/// use qtty_core::solid_angle::{SolidAngleOf, SquareDegrees};
///
/// let side = Degrees::new(5.0);
/// let omega: SolidAngleOf<Degree> = side * side;
/// assert!((omega.value() - 25.0).abs() < 1e-12);
///
/// // SolidAngleOf<Degree> IS SquareDegrees — same type:
/// let named: SquareDegrees = omega;
/// assert!((named.value() - 25.0).abs() < 1e-12);
/// ```
pub type SolidAngleOf<A> = Quantity<Prod<A, A>>;

// ─────────────────────────────────────────────────────────────────────────────
// Always-available solid-angle units (built from base angular units)
// ─────────────────────────────────────────────────────────────────────────────

/// Square degree — product of two [`Degree`] units (canonical, `RATIO == 1.0`).
pub type SquareDegree = Prod<Degree, Degree>;
/// A quantity measured in square degrees (= [`SolidAngleOf<Degree>`]).
pub type SquareDegrees = Quantity<SquareDegree>;

/// Steradian — product of two [`Radian`] units (`(180/π)² deg²`).
pub type Steradian = Prod<Radian, Radian>;
/// A quantity measured in steradians (= [`SolidAngleOf<Radian>`]).
pub type Steradians = Quantity<Steradian>;

/// Square milliradian — product of two [`Milliradian`] units.
pub type SquareMilliradian = Prod<Milliradian, Milliradian>;
/// A quantity measured in square milliradians.
pub type SquareMilliradians = Quantity<SquareMilliradian>;

// ─────────────────────────────────────────────────────────────────────────────
// Astro-feature solid-angle units
// ─────────────────────────────────────────────────────────────────────────────

/// Square arcminute — product of two [`Arcminute`] units.
#[cfg(feature = "astro")]
pub type SquareArcminute = Prod<Arcminute, Arcminute>;
/// A quantity measured in square arcminutes.
#[cfg(feature = "astro")]
pub type SquareArcminutes = Quantity<SquareArcminute>;

/// Square arcsecond — product of two [`Arcsecond`] units.
#[cfg(feature = "astro")]
pub type SquareArcsecond = Prod<Arcsecond, Arcsecond>;
/// A quantity measured in square arcseconds.
#[cfg(feature = "astro")]
pub type SquareArcseconds = Quantity<SquareArcsecond>;

/// Canonical list of always-available solid-angle units.
///
/// Exported (`#[doc(hidden)]`) for use by the cross-dimension registry,
/// `From` impls, and compile-time consistency checks.
#[macro_export]
#[doc(hidden)]
macro_rules! solid_angle_units {
    ($cb:path) => {
        $cb!(SquareDegree, Steradian, SquareMilliradian);
    };
}

// Generate bidirectional From impls between always-available solid-angle units.
solid_angle_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
solid_angle_units!(crate::impl_unit_cross_unit_ops);

// ── Astro-feature cross conversions ──────────────────────────────────────────
#[cfg(feature = "astro")]
crate::impl_unit_from_conversions_between!(
    SquareDegree, Steradian, SquareMilliradian;
    SquareArcminute, SquareArcsecond
);

#[cfg(all(feature = "astro", feature = "cross-unit-ops"))]
crate::impl_unit_cross_unit_ops_between!(
    SquareDegree, Steradian, SquareMilliradian;
    SquareArcminute, SquareArcsecond
);

/// Canonical list of solid-angle units exposed under the `astro` feature.
#[cfg(feature = "astro")]
#[macro_export]
#[doc(hidden)]
macro_rules! solid_angle_astro_units {
    ($cb:path) => {
        $cb!(SquareArcminute, SquareArcsecond);
    };
}

#[cfg(all(test, feature = "astro"))]
solid_angle_astro_units!(crate::assert_units_are_builtin);

// Compile-time check: every base solid-angle unit is registered as BuiltinUnit.
#[cfg(test)]
solid_angle_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::angular::{Degrees, Radians};
    use approx::assert_relative_eq;

    const STERADIAN_IN_SQDEG: f64 = (180.0 / core::f64::consts::PI)
        * (180.0 / core::f64::consts::PI);

    #[test]
    fn square_degree_is_canonical() {
        assert_eq!(SquareDegree::RATIO, 1.0);
    }

    #[test]
    fn steradian_to_square_degree_ratio() {
        assert_relative_eq!(Steradian::RATIO, STERADIAN_IN_SQDEG, max_relative = 1e-12);
    }

    #[test]
    fn radian_squared_is_steradian() {
        let r = Radians::new(1.0);
        let omega: Steradians = r * r;
        assert_relative_eq!(omega.value(), 1.0, max_relative = 1e-12);
    }

    #[test]
    fn degree_squared_is_square_degree() {
        let d = Degrees::new(3.0);
        let omega: SquareDegrees = d * d;
        assert_relative_eq!(omega.value(), 9.0, max_relative = 1e-12);
    }

    #[test]
    fn steradian_to_square_degree_conversion() {
        let sr = Steradians::new(1.0);
        let sqd: SquareDegrees = sr.to();
        assert_relative_eq!(sqd.value(), STERADIAN_IN_SQDEG, max_relative = 1e-12);
    }

    #[test]
    fn full_sphere_in_square_degrees() {
        // Full sphere = 4π sr = 4π · (180/π)² = 41252.961… deg²
        let full = Steradians::new(4.0 * core::f64::consts::PI);
        let sqd: SquareDegrees = full.to();
        assert_relative_eq!(sqd.value(), 41_252.961_249_419_3, max_relative = 1e-9);
    }

    #[test]
    #[cfg(feature = "astro")]
    fn square_arcsecond_to_steradian() {
        // 1 arcsec = π/(180·3600) rad → 1 arcsec² = (π/648000)² sr
        let one = Quantity::<SquareArcsecond>::new(1.0);
        let sr: Steradians = one.to();
        let expected = (core::f64::consts::PI / (180.0 * 3600.0)).powi(2);
        assert_relative_eq!(sr.value(), expected, max_relative = 1e-12);
    }

    #[test]
    #[cfg(feature = "astro")]
    fn square_arcminute_in_square_degrees() {
        let one = Quantity::<SquareArcminute>::new(3600.0);
        let sqd: SquareDegrees = one.to();
        // 3600 arcmin² = 1 deg² (60 arcmin = 1 deg ⇒ 3600 arcmin² = 1 deg²)
        assert_relative_eq!(sqd.value(), 1.0, max_relative = 1e-12);
    }

    #[test]
    fn square_milliradian_to_steradian() {
        let one = Quantity::<SquareMilliradian>::new(1.0);
        let sr: Steradians = one.to();
        // 1 mrad² = 1e-6 sr
        assert_relative_eq!(sr.value(), 1e-6, max_relative = 1e-12);
    }
}
