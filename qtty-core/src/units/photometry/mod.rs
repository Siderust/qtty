// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Logarithmic photometric quantities: magnitudes and surface brightness;
//! and SI photometric units: candela, lumen, lux.
//!
//! Available behind the `photometry` feature.
//!
//! ## Design choice — Option A newtypes
//!
//! Magnitudes are *logarithmic* and dimensionally distinct from linear
//! physical dimensions: adding two magnitudes (`mag1 + mag2`) is
//! physically meaningless, while subtracting them (`mag1 − mag2`) yields a
//! magnitude *difference*. This module therefore uses **newtypes** (Option A)
//! rather than `Quantity<U>`, mirroring the `affn::Position` / displacement
//! pattern:
//!
//! - `Magnitude − Magnitude` → `Magnitude` (a difference, in mag).
//! - `Magnitude + Magnitude` → **compile error** (no `Add<Magnitude>` impl).
//!
//! ## Photometric system
//!
//! This module is **system-agnostic**: no specific zero-point (Vega, AB, ST,
//! or any survey-specific value) is hard-coded. The zero-point is always
//! supplied by the caller, keeping domain-specific constants in the domain
//! crate (e.g. the `27.78` S10-equivalent zero-point lives in `nsb`).
//!
//! ```rust
//! use qtty_core::photometry::{flux_to_magnitude, magnitude_to_flux};
//!
//! // 100× flux ratio = 5 mag (Pogson's definition)
//! let m = flux_to_magnitude(100.0, 0.0);
//! assert!((m.value() - (-5.0)).abs() < 1e-12);
//!
//! // Round-trip
//! let f = 42.0_f64;
//! let zp = 20.0_f64;
//! let recovered = magnitude_to_flux(flux_to_magnitude(f, zp), zp);
//! assert!((recovered - f).abs() < 1e-10);
//! ```

use core::ops::Sub;

use crate::Quantity;
use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Magnitude
// ─────────────────────────────────────────────────────────────────────────────

/// A logarithmic magnitude value (dimensionless, but system-agnostic).
///
/// The photometric system (Vega / AB / ST / survey-specific) is **implicit** in
/// the zero-point used when constructing or converting this value. No specific
/// system is assumed here.
///
/// ## Arithmetic
///
/// Only subtraction is defined: `Magnitude − Magnitude` returns the
/// magnitude *difference*, which is a dimensionally meaningful quantity.
/// Addition of two magnitudes is **not** defined (it is physically
/// meaningless); attempting it will produce a compile error.
///
/// ## NaN / infinity
///
/// Arithmetic follows IEEE-754 semantics. Values constructed from non-finite
/// or negative-flux inputs propagate `NaN` or `±∞` as expected.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Magnitude(pub f64);

impl Magnitude {
    /// Construct a [`Magnitude`] from a raw value.
    #[inline]
    pub const fn new(v: f64) -> Self {
        Self(v)
    }

    /// Return the underlying scalar (in mag).
    #[inline]
    pub fn value(self) -> f64 {
        self.0
    }
}

/// `Magnitude − Magnitude` → magnitude difference (in mag).
impl Sub for Magnitude {
    type Output = Magnitude;

    #[inline]
    fn sub(self, rhs: Magnitude) -> Magnitude {
        Magnitude(self.0 - rhs.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SurfaceBrightness
// ─────────────────────────────────────────────────────────────────────────────

/// Surface brightness in magnitudes per square arcsecond (`mag arcsec⁻²`).
///
/// Like [`Magnitude`], this type is system-agnostic: the photometric system
/// (Vega / AB / ST / survey-specific) is implicit in the zero-point used when
/// constructing the value.
///
/// ## Arithmetic
///
/// Subtraction is defined and returns a surface-brightness *difference*
/// (still in mag arcsec⁻²). Addition is **not** defined (compile error).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct SurfaceBrightness(pub f64);

impl SurfaceBrightness {
    /// Construct a [`SurfaceBrightness`] from a raw value in mag arcsec⁻².
    #[inline]
    pub const fn new(v: f64) -> Self {
        Self(v)
    }

    /// Return the underlying scalar in mag arcsec⁻².
    #[inline]
    pub fn value(self) -> f64 {
        self.0
    }
}

/// `SurfaceBrightness − SurfaceBrightness` → surface brightness difference.
impl Sub for SurfaceBrightness {
    type Output = SurfaceBrightness;

    #[inline]
    fn sub(self, rhs: SurfaceBrightness) -> SurfaceBrightness {
        SurfaceBrightness(self.0 - rhs.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Conversion functions
// ─────────────────────────────────────────────────────────────────────────────

/// Convert a flux to a [`Magnitude`] using the classical Pogson formula:
///
/// ```text
/// mag = zero_point − 2.5 · log10(flux)
/// ```
///
/// `zero_point` is the magnitude assigned to `flux = 1.0` in whatever
/// units `flux` is measured. Callers are responsible for unit consistency.
///
/// If `flux ≤ 0`, the result is `NaN` (from `log10`) or `+∞`; this mirrors
/// IEEE-754 natural behaviour and is documented as intentional — no
/// validation is performed.
///
/// # Example
/// ```rust
/// use qtty_core::photometry::flux_to_magnitude;
///
/// // flux = 1, zero_point = 0  →  mag = 0
/// let m = flux_to_magnitude(1.0, 0.0);
/// assert!((m.value() - 0.0).abs() < 1e-12);
///
/// // 100× brighter = 5 mag (Pogson)
/// let m = flux_to_magnitude(100.0, 0.0);
/// assert!((m.value() - (-5.0)).abs() < 1e-12);
/// ```
#[inline]
pub fn flux_to_magnitude(flux: f64, zero_point: f64) -> Magnitude {
    Magnitude(zero_point - 2.5 * flux.log10())
}

/// Convert a [`Magnitude`] back to a linear flux using the inverse Pogson
/// formula:
///
/// ```text
/// flux = 10^((zero_point − mag) / 2.5)
/// ```
///
/// The `zero_point` must be the same value used when constructing `mag`.
///
/// # Example
/// ```rust
/// use qtty_core::photometry::{flux_to_magnitude, magnitude_to_flux};
///
/// let f = 42.0_f64;
/// let zp = 20.0_f64;
/// let m = flux_to_magnitude(f, zp);
/// let recovered = magnitude_to_flux(m, zp);
/// assert!((recovered - f).abs() < 1e-10);
/// ```
#[inline]
pub fn magnitude_to_flux(mag: Magnitude, zero_point: f64) -> f64 {
    10_f64.powf((zero_point - mag.0) / 2.5)
}

/// Convert a per-area-per-solid-angle flux to a [`SurfaceBrightness`] using
/// the classical Pogson formula:
///
/// ```text
/// sb = zero_point − 2.5 · log10(flux)
/// ```
///
/// This is identical to [`flux_to_magnitude`] in arithmetic but returns
/// [`SurfaceBrightness`] (mag arcsec⁻²) as a convenience for callers whose
/// flux is already normalised to a per-solid-angle quantity.
/// The same IEEE-754 NaN/±∞ propagation rules apply for `flux ≤ 0`.
///
/// # Example
/// ```rust
/// use qtty_core::photometry::band_flux_to_surface_brightness;
///
/// // NSB zero-point: 27.78 (get_NSB.py, B-band-equivalent S10 units)
/// let sb = band_flux_to_surface_brightness(1.0, 27.78);
/// assert!((sb.value() - 27.78).abs() < 1e-12);
/// ```
#[inline]
pub fn band_flux_to_surface_brightness(flux: f64, zero_point: f64) -> SurfaceBrightness {
    SurfaceBrightness(zero_point - 2.5 * flux.log10())
}

/// Typed variant of [`band_flux_to_surface_brightness`] that accepts an
/// [`S10s`] flux quantity directly, avoiding the need to call `.value()`.
///
/// Requires both the `photometry` and `radiometry` features.
///
/// # Example
/// ```rust
/// use qtty_core::photometry::s10_to_surface_brightness;
/// use qtty_core::radiometry::S10s;
///
/// let flux = S10s::new(1.0);
/// let sb = s10_to_surface_brightness(flux, 27.78);
/// assert!((sb.value() - 27.78).abs() < 1e-12);
/// ```
#[cfg(feature = "radiometry")]
#[inline]
pub fn s10_to_surface_brightness(
    flux: crate::units::radiometry::S10s,
    zero_point: f64,
) -> SurfaceBrightness {
    band_flux_to_surface_brightness(flux.value(), zero_point)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    /// `flux = 1, zp = 0` → `mag = 0`
    #[test]
    fn flux_to_mag_unit_flux() {
        let m = flux_to_magnitude(1.0, 0.0);
        assert!(
            (m.value() - 0.0).abs() < 1e-12,
            "expected 0, got {}",
            m.value()
        );
    }

    /// 100× flux ratio = 5 mag (Pogson's definition)
    #[test]
    fn flux_to_mag_hundred_to_one() {
        let m = flux_to_magnitude(100.0, 0.0);
        assert!(
            (m.value() - (-5.0)).abs() < 1e-12,
            "expected -5, got {}",
            m.value()
        );
    }

    /// Round-trip `flux → mag → flux` is bit-equivalent for representative inputs.
    #[test]
    fn round_trip_bit_equivalence() {
        for &(f, zp) in &[(1.0_f64, 0.0), (42.0, 20.0), (1e-4, 30.0), (1e6, -5.0)] {
            let recovered = magnitude_to_flux(flux_to_magnitude(f, zp), zp);
            assert!(
                (recovered - f).abs() / f < 1e-12,
                "round-trip failed: f={f}, zp={zp}, recovered={recovered}"
            );
        }
    }

    /// NSB zero-point: `flux = 1` with `zp = 27.78` → `mag = 27.78`
    #[test]
    fn nsb_zero_point() {
        let m = flux_to_magnitude(1.0, 27.78);
        assert!(
            (m.value() - 27.78).abs() < 1e-12,
            "expected 27.78, got {}",
            m.value()
        );
    }

    /// Magnitude subtraction yields a difference.
    #[test]
    fn magnitude_subtraction() {
        let a = Magnitude::new(15.0);
        let b = Magnitude::new(10.0);
        let diff = a - b;
        assert!((diff.value() - 5.0).abs() < 1e-12);
    }

    /// `band_flux_to_surface_brightness` with NSB zero-point at `flux = 1`.
    #[test]
    fn surface_brightness_nsb_zero_point() {
        let sb = band_flux_to_surface_brightness(1.0, 27.78);
        assert!((sb.value() - 27.78).abs() < 1e-12);
    }

    /// `SurfaceBrightness` subtraction.
    #[test]
    fn surface_brightness_subtraction() {
        let a = SurfaceBrightness::new(22.0);
        let b = SurfaceBrightness::new(20.0);
        let diff = a - b;
        assert!((diff.value() - 2.0).abs() < 1e-12);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SI photometric units
// ─────────────────────────────────────────────────────────────────────────────

pub use crate::dimension::{Illuminance, LuminousFlux, LuminousIntensity};

/// Candela — SI base unit of luminous intensity (cd).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "cd", dimension = LuminousIntensity, ratio = 1.0)]
pub struct Candela;
/// Type alias shorthand for [`Candela`].
pub type Cd = Candela;
/// A quantity measured in candelas.
pub type Candelas = Quantity<Cd>;
/// One candela.
pub const CANDELA: Candelas = Candelas::new(1.0);

/// Lumen — SI coherent derived unit of luminous flux (cd·sr).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lm", dimension = LuminousFlux, ratio = 1.0)]
pub struct Lumen;
/// Type alias shorthand for [`Lumen`].
pub type Lm = Lumen;
/// A quantity measured in lumens.
pub type Lumens = Quantity<Lm>;
/// One lumen.
pub const LUMEN: Lumens = Lumens::new(1.0);

/// Millilumen — 1 mlm = 10⁻³ lm.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mlm", dimension = LuminousFlux, ratio = 1e-3)]
pub struct Millilumen;
/// A quantity measured in millilumens.
pub type Millilumens = Quantity<Millilumen>;
/// One millilumen.
pub const MILLILUMEN: Millilumens = Millilumens::new(1.0);

/// Kilolumen — 1 klm = 10³ lm.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "klm", dimension = LuminousFlux, ratio = 1e3)]
pub struct Kilolumen;
/// A quantity measured in kilolumens.
pub type Kilolumens = Quantity<Kilolumen>;
/// One kilolumen.
pub const KILOLUMEN: Kilolumens = Kilolumens::new(1.0);

/// Lux — SI coherent derived unit of illuminance (lm·m⁻²).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lx", dimension = Illuminance, ratio = 1.0)]
pub struct Lux;
/// Type alias shorthand for [`Lux`].
pub type Lx = Lux;
/// A quantity measured in lux.
pub type Luxs = Quantity<Lx>;
/// One lux.
pub const LUX: Luxs = Luxs::new(1.0);

/// Millilux — 1 mlx = 10⁻³ lx.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mlx", dimension = Illuminance, ratio = 1e-3)]
pub struct Millilux;
/// A quantity measured in millilux.
pub type Milliluxs = Quantity<Millilux>;
/// One millilux.
pub const MILLILUX: Milliluxs = Milliluxs::new(1.0);

/// Kilolux — 1 klx = 10³ lx.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "klx", dimension = Illuminance, ratio = 1e3)]
pub struct Kilolux;
/// A quantity measured in kilolux.
pub type Kiloluxs = Quantity<Kilolux>;
/// One kilolux.
pub const KILOLUX: Kiloluxs = Kiloluxs::new(1.0);

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macros
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of luminous-intensity (candela) units.
#[macro_export]
#[doc(hidden)]
macro_rules! candela_units {
    ($cb:path) => {
        $cb!(Candela);
    };
}

/// Canonical list of luminous-flux (lumen) units.
#[macro_export]
#[doc(hidden)]
macro_rules! lumen_units {
    ($cb:path) => {
        $cb!(Lumen, Millilumen, Kilolumen);
    };
}

/// Canonical list of illuminance (lux) units.
#[macro_export]
#[doc(hidden)]
macro_rules! lux_units {
    ($cb:path) => {
        $cb!(Lux, Millilux, Kilolux);
    };
}

candela_units!(crate::impl_unit_from_conversions);
lumen_units!(crate::impl_unit_from_conversions);
lux_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
candela_units!(crate::impl_unit_cross_unit_ops);
#[cfg(feature = "cross-unit-ops")]
lumen_units!(crate::impl_unit_cross_unit_ops);
#[cfg(feature = "cross-unit-ops")]
lux_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
candela_units!(crate::assert_units_are_builtin);
#[cfg(test)]
lumen_units!(crate::assert_units_are_builtin);
#[cfg(test)]
lux_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod photometric_unit_tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn kilolumen_to_lumen() {
        let klm = Kilolumens::new(1.0);
        let lm: Lumens = klm.to();
        assert_abs_diff_eq!(lm.value(), 1_000.0, epsilon = 1e-9);
    }

    #[test]
    fn millilux_to_lux() {
        let mlx = Milliluxs::new(1_000.0);
        let lx: Luxs = mlx.to();
        assert_abs_diff_eq!(lx.value(), 1.0, epsilon = 1e-12);
    }
}
