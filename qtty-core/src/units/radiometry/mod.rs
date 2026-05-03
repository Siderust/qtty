// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Radiometric and photon-radiometric units.
//!
//! Available behind the `radiometry` feature. This module collects unit
//! markers for the four most common radiometric quantities used in
//! astronomy and atmospheric optics:
//!
//! - [`Radiance`](crate::dimension::Radiance) —
//!   power per area per solid angle (e.g. W·m⁻²·sr⁻¹).
//! - [`SpectralRadiance`](crate::dimension::SpectralRadiance) —
//!   radiance per unit wavelength (e.g. W·m⁻²·sr⁻¹·nm⁻¹).
//! - [`PhotonRadiance`](crate::dimension::PhotonRadiance) —
//!   photon count per area per time per solid angle (e.g. ph·cm⁻²·s⁻¹·sr⁻¹).
//! - [`SpectralPhotonRadiance`](crate::dimension::SpectralPhotonRadiance) —
//!   spectral photon radiance per unit wavelength
//!   (e.g. ph·cm⁻²·s⁻¹·sr⁻¹·Å⁻¹).
//!
//! It also provides:
//!
//! - [`S10`] — the "10th-magnitude stars per square degree" surface
//!   brightness unit used by Leinert et al.'s zodiacal-light tables.
//! - [`erg_to_photon`] — typed conversion from spectral *energy* radiance in
//!   CGS (erg·s⁻¹·cm⁻²·sr⁻¹·Å⁻¹) to spectral *photon* radiance
//!   (ph·s⁻¹·cm⁻²·sr⁻¹·Å⁻¹) at a given wavelength using `1 / (h · c)`.
//!
//! ```rust
//! use qtty_core::radiometry::WattsPerSquareMeterSteradian;
//!
//! let l = WattsPerSquareMeterSteradian::new(1.0);
//! assert_eq!(l.value(), 1.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

pub use crate::dimension::{
    InverseSolidAngle, PhotonRadiance, Radiance, SpectralPhotonRadiance, SpectralRadiance,
};

// ─────────────────────────────────────────────────────────────────────────────
// Marker traits
// ─────────────────────────────────────────────────────────────────────────────

/// Marker trait for any [`Unit`] whose dimension is [`Radiance`].
pub trait RadianceUnit: Unit<Dim = Radiance> {}
impl<T: Unit<Dim = Radiance>> RadianceUnit for T {}

/// Marker trait for any [`Unit`] whose dimension is [`SpectralRadiance`].
pub trait SpectralRadianceUnit: Unit<Dim = SpectralRadiance> {}
impl<T: Unit<Dim = SpectralRadiance>> SpectralRadianceUnit for T {}

/// Marker trait for any [`Unit`] whose dimension is [`PhotonRadiance`].
pub trait PhotonRadianceUnit: Unit<Dim = PhotonRadiance> {}
impl<T: Unit<Dim = PhotonRadiance>> PhotonRadianceUnit for T {}

/// Marker trait for any [`Unit`] whose dimension is [`SpectralPhotonRadiance`].
pub trait SpectralPhotonRadianceUnit: Unit<Dim = SpectralPhotonRadiance> {}
impl<T: Unit<Dim = SpectralPhotonRadiance>> SpectralPhotonRadianceUnit for T {}

/// Marker trait for any [`Unit`] whose dimension is [`InverseSolidAngle`].
pub trait InverseSolidAngleUnit: Unit<Dim = InverseSolidAngle> {}
impl<T: Unit<Dim = InverseSolidAngle>> InverseSolidAngleUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// Radiance — W · m⁻² · sr⁻¹
// ─────────────────────────────────────────────────────────────────────────────

/// Radiance — watt per square metre per steradian (`W·m⁻²·sr⁻¹`).
///
/// Canonical SI unit of radiance. RATIO is `1.0`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "W·m⁻²·sr⁻¹", dimension = Radiance, ratio = 1.0)]
pub struct WattPerSquareMeterSteradian;
/// A quantity measured in watts per square metre per steradian.
pub type WattsPerSquareMeterSteradian = Quantity<WattPerSquareMeterSteradian>;

/// Radiance — erg per second per square centimetre per steradian
/// (CGS, `erg·s⁻¹·cm⁻²·sr⁻¹`).
///
/// 1 erg·s⁻¹·cm⁻²·sr⁻¹ = 1×10⁻³ W·m⁻²·sr⁻¹.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "erg·s⁻¹·cm⁻²·sr⁻¹", dimension = Radiance, ratio = 1.0e-3)]
pub struct ErgPerSecondSquareCentimeterSteradian;
/// A quantity measured in erg·s⁻¹·cm⁻²·sr⁻¹.
pub type ErgsPerSecondSquareCentimeterSteradian = Quantity<ErgPerSecondSquareCentimeterSteradian>;

// ─────────────────────────────────────────────────────────────────────────────
// Spectral radiance (per wavelength) — W · m⁻² · sr⁻¹ · m⁻¹
// ─────────────────────────────────────────────────────────────────────────────

/// Spectral radiance — watt per cubic metre per steradian
/// (`W·m⁻³·sr⁻¹` ≡ `W·m⁻²·sr⁻¹·m⁻¹`).
///
/// Canonical SI unit. RATIO is `1.0`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "W·m⁻²·sr⁻¹·m⁻¹", dimension = SpectralRadiance, ratio = 1.0)]
pub struct WattPerSquareMeterSteradianMeter;
/// A quantity measured in W·m⁻²·sr⁻¹·m⁻¹.
pub type WattsPerSquareMeterSteradianMeter = Quantity<WattPerSquareMeterSteradianMeter>;

/// Spectral radiance — watt per square metre per steradian per **nanometre**.
///
/// 1 W·m⁻²·sr⁻¹·nm⁻¹ = 1×10⁹ W·m⁻²·sr⁻¹·m⁻¹.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "W·m⁻²·sr⁻¹·nm⁻¹", dimension = SpectralRadiance, ratio = 1.0e9)]
pub struct WattPerSquareMeterSteradianNanometer;
/// A quantity measured in W·m⁻²·sr⁻¹·nm⁻¹.
pub type WattsPerSquareMeterSteradianNanometer = Quantity<WattPerSquareMeterSteradianNanometer>;

/// Spectral radiance — erg per second per square centimetre per steradian per
/// **ångström** (`erg·s⁻¹·cm⁻²·sr⁻¹·Å⁻¹`), the standard spectroscopy CGS
/// unit used by `NSB_Utils.py`.
///
/// 1 erg·s⁻¹·cm⁻²·sr⁻¹·Å⁻¹ = 1×10⁷ W·m⁻²·sr⁻¹·m⁻¹.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "erg·s⁻¹·cm⁻²·sr⁻¹·Å⁻¹", dimension = SpectralRadiance, ratio = 1.0e7)]
pub struct ErgPerSecondSquareCentimeterSteradianAngstrom;
/// A quantity measured in erg·s⁻¹·cm⁻²·sr⁻¹·Å⁻¹.
pub type ErgsPerSecondSquareCentimeterSteradianAngstrom =
    Quantity<ErgPerSecondSquareCentimeterSteradianAngstrom>;

// ─────────────────────────────────────────────────────────────────────────────
// Photon radiance — ph · cm⁻² · s⁻¹ · sr⁻¹
// ─────────────────────────────────────────────────────────────────────────────

/// Photon radiance — photons per square metre per second per steradian
/// (`ph·m⁻²·s⁻¹·sr⁻¹`).
///
/// Canonical SI form. RATIO is `1.0`. Photons are treated as dimensionless
/// counts.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ph·m⁻²·s⁻¹·sr⁻¹", dimension = PhotonRadiance, ratio = 1.0)]
pub struct PhotonPerSquareMeterSecondSteradian;
/// A quantity measured in ph·m⁻²·s⁻¹·sr⁻¹.
pub type PhotonsPerSquareMeterSecondSteradian = Quantity<PhotonPerSquareMeterSecondSteradian>;

/// Photon radiance — photons per square centimetre per second per steradian
/// (`ph·cm⁻²·s⁻¹·sr⁻¹`).
///
/// 1 ph·cm⁻²·s⁻¹·sr⁻¹ = 1×10⁴ ph·m⁻²·s⁻¹·sr⁻¹.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ph·cm⁻²·s⁻¹·sr⁻¹", dimension = PhotonRadiance, ratio = 1.0e4)]
pub struct PhotonPerSquareCentimeterSecondSteradian;
/// A quantity measured in ph·cm⁻²·s⁻¹·sr⁻¹.
pub type PhotonsPerSquareCentimeterSecondSteradian =
    Quantity<PhotonPerSquareCentimeterSecondSteradian>;

/// Photon radiance — photons per square centimetre per **nanosecond** per
/// steradian (`ph·cm⁻²·ns⁻¹·sr⁻¹`).
///
/// This is the unit reported by the `darknsb` Python pipeline for the
/// integrated NSB. 1 ph·cm⁻²·ns⁻¹·sr⁻¹ = 1×10¹³ ph·m⁻²·s⁻¹·sr⁻¹.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ph·cm⁻²·ns⁻¹·sr⁻¹", dimension = PhotonRadiance, ratio = 1.0e13)]
pub struct PhotonPerSquareCentimeterNanosecondSteradian;
/// A quantity measured in ph·cm⁻²·ns⁻¹·sr⁻¹.
pub type PhotonsPerSquareCentimeterNanosecondSteradian =
    Quantity<PhotonPerSquareCentimeterNanosecondSteradian>;

// ─────────────────────────────────────────────────────────────────────────────
// Spectral photon radiance — ph · cm⁻² · s⁻¹ · sr⁻¹ · Å⁻¹
// ─────────────────────────────────────────────────────────────────────────────

/// Spectral photon radiance — photons per square metre per second per
/// steradian per **metre** (`ph·m⁻³·s⁻¹·sr⁻¹`).
///
/// Canonical SI form. RATIO is `1.0`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ph·m⁻²·s⁻¹·sr⁻¹·m⁻¹", dimension = SpectralPhotonRadiance, ratio = 1.0)]
pub struct PhotonPerSquareMeterSecondSteradianMeter;
/// A quantity measured in ph·m⁻²·s⁻¹·sr⁻¹·m⁻¹.
pub type PhotonsPerSquareMeterSecondSteradianMeter =
    Quantity<PhotonPerSquareMeterSecondSteradianMeter>;

/// Spectral photon radiance — `ph·cm⁻²·s⁻¹·sr⁻¹·Å⁻¹`, the standard
/// spectroscopy unit (matches `NSB_Utils.py`).
///
/// 1 ph·cm⁻²·s⁻¹·sr⁻¹·Å⁻¹ = 1×10¹⁴ ph·m⁻²·s⁻¹·sr⁻¹·m⁻¹
/// (10⁴ for cm⁻²→m⁻² and 10¹⁰ for Å⁻¹→m⁻¹).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ph·cm⁻²·s⁻¹·sr⁻¹·Å⁻¹", dimension = SpectralPhotonRadiance, ratio = 1.0e14)]
pub struct PhotonPerSquareCentimeterSecondSteradianAngstrom;
/// A quantity measured in ph·cm⁻²·s⁻¹·sr⁻¹·Å⁻¹.
pub type PhotonsPerSquareCentimeterSecondSteradianAngstrom =
    Quantity<PhotonPerSquareCentimeterSecondSteradianAngstrom>;

/// Spectral photon radiance — `ph·cm⁻²·s⁻¹·sr⁻¹·nm⁻¹`.
///
/// 1 ph·cm⁻²·s⁻¹·sr⁻¹·nm⁻¹ = 1×10¹³ ph·m⁻²·s⁻¹·sr⁻¹·m⁻¹.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ph·cm⁻²·s⁻¹·sr⁻¹·nm⁻¹", dimension = SpectralPhotonRadiance, ratio = 1.0e13)]
pub struct PhotonPerSquareCentimeterSecondSteradianNanometer;
/// A quantity measured in ph·cm⁻²·s⁻¹·sr⁻¹·nm⁻¹.
pub type PhotonsPerSquareCentimeterSecondSteradianNanometer =
    Quantity<PhotonPerSquareCentimeterSecondSteradianNanometer>;

/// Spectral photon radiance — `ph·cm⁻²·ns⁻¹·sr⁻¹·nm⁻¹`.
///
/// 1 ph·cm⁻²·ns⁻¹·sr⁻¹·nm⁻¹ = 1×10²² ph·m⁻²·s⁻¹·sr⁻¹·m⁻¹
/// (10⁴ for cm⁻²→m⁻², 10⁹ for ns⁻¹→s⁻¹, and 10⁹ for nm⁻¹→m⁻¹).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ph·cm⁻²·ns⁻¹·sr⁻¹·nm⁻¹", dimension = SpectralPhotonRadiance, ratio = 1.0e22)]
pub struct PhotonPerSquareCentimeterNanosecondSteradianNanometer;
/// A quantity measured in ph·cm⁻²·ns⁻¹·sr⁻¹·nm⁻¹.
pub type PhotonsPerSquareCentimeterNanosecondSteradianNanometer =
    Quantity<PhotonPerSquareCentimeterNanosecondSteradianNanometer>;

// ─────────────────────────────────────────────────────────────────────────────
// S10 — 10th-magnitude stars per square degree
// ─────────────────────────────────────────────────────────────────────────────

/// `S10` — surface brightness in units of one 10th-visual-magnitude star per
/// square degree.
///
/// Used by Leinert et al. (1998) zodiacal-light tables and by classical
/// background-light catalogues. Dimensionally this is an inverse solid
/// angle; the name `S10` is preserved for traceability.
///
/// Conversion to/from a true spectral radiance depends on the photometric
/// band and reference spectrum chosen, so this crate exposes `S10` only as
/// an inverse-solid-angle marker. Domain crates should layer their own
/// (band-dependent) transform on top.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "S10", dimension = InverseSolidAngle, ratio = 1.0)]
pub struct S10;
/// A quantity measured in S10 units.
pub type S10s = Quantity<S10>;

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macros — one per dimension group
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of radiance units (power per area per solid angle).
#[macro_export]
#[doc(hidden)]
macro_rules! radiance_units {
    ($cb:path) => {
        $cb!(
            WattPerSquareMeterSteradian,
            ErgPerSecondSquareCentimeterSteradian
        );
    };
}

/// Canonical list of spectral-radiance units (radiance per unit wavelength).
#[macro_export]
#[doc(hidden)]
macro_rules! spectral_radiance_units {
    ($cb:path) => {
        $cb!(
            WattPerSquareMeterSteradianMeter,
            WattPerSquareMeterSteradianNanometer,
            ErgPerSecondSquareCentimeterSteradianAngstrom
        );
    };
}

/// Canonical list of photon-radiance units (photon count per area per time per solid angle).
#[macro_export]
#[doc(hidden)]
macro_rules! photon_radiance_units {
    ($cb:path) => {
        $cb!(
            PhotonPerSquareMeterSecondSteradian,
            PhotonPerSquareCentimeterSecondSteradian,
            PhotonPerSquareCentimeterNanosecondSteradian
        );
    };
}

/// Canonical list of spectral-photon-radiance units.
#[macro_export]
#[doc(hidden)]
macro_rules! spectral_photon_radiance_units {
    ($cb:path) => {
        $cb!(
            PhotonPerSquareMeterSecondSteradianMeter,
            PhotonPerSquareCentimeterSecondSteradianAngstrom,
            PhotonPerSquareCentimeterSecondSteradianNanometer,
            PhotonPerSquareCentimeterNanosecondSteradianNanometer
        );
    };
}

/// Canonical list of inverse-solid-angle units.
#[macro_export]
#[doc(hidden)]
macro_rules! inverse_solid_angle_units {
    ($cb:path) => {
        $cb!(S10);
    };
}

// Generate bidirectional From impls between units within each dimension group.
radiance_units!(crate::impl_unit_from_conversions);
spectral_radiance_units!(crate::impl_unit_from_conversions);
photon_radiance_units!(crate::impl_unit_from_conversions);
spectral_photon_radiance_units!(crate::impl_unit_from_conversions);
// inverse_solid_angle_units! has only one unit; no From conversions to generate.

#[cfg(feature = "cross-unit-ops")]
radiance_units!(crate::impl_unit_cross_unit_ops);
#[cfg(feature = "cross-unit-ops")]
spectral_radiance_units!(crate::impl_unit_cross_unit_ops);
#[cfg(feature = "cross-unit-ops")]
photon_radiance_units!(crate::impl_unit_cross_unit_ops);
#[cfg(feature = "cross-unit-ops")]
spectral_photon_radiance_units!(crate::impl_unit_cross_unit_ops);

// Compile-time check: every radiometry unit is registered as BuiltinUnit.
#[cfg(test)]
radiance_units!(crate::assert_units_are_builtin);
#[cfg(test)]
spectral_radiance_units!(crate::assert_units_are_builtin);
#[cfg(test)]
photon_radiance_units!(crate::assert_units_are_builtin);
#[cfg(test)]
spectral_photon_radiance_units!(crate::assert_units_are_builtin);
#[cfg(test)]
inverse_solid_angle_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Conversion helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Photon energy `E = h · c` in **erg · ångström** units (CODATA 2018).
///
/// `h · c` ≈ `1.986_445_857 × 10⁻⁸ erg·Å`. The reciprocal,
/// `5.034_116_5 × 10⁷ ph / (erg · Å)`, was historically rounded to
/// `5.03e7` in `NSB_Utils.py`.
const HC_ERG_ANGSTROM: f64 = 1.986_445_857_148_968e-8;

/// Photon energy `E = h · c` in **joule metre** units (CODATA 2018).
const HC_JOULE_METER: f64 = 1.986_445_857_148_968e-25;

/// Convert spectral *energy* radiance (CGS, erg·s⁻¹·cm⁻²·sr⁻¹·Å⁻¹) to
/// spectral *photon* radiance (ph·s⁻¹·cm⁻²·sr⁻¹·Å⁻¹) at wavelength
/// `lambda` (in metres), using `1 / (h · c)`.
///
/// Both inputs and outputs are typed [`Quantity`] values, so the call site
/// keeps the dimensional bookkeeping that the historical `5.03e7 · λ`
/// scalar form lost.
///
/// ```rust
/// use qtty_core::length::Meters;
/// use qtty_core::radiometry::{
///     erg_to_photon, ErgsPerSecondSquareCentimeterSteradianAngstrom,
/// };
///
/// // λ = 5500 Å = 5.5e-7 m
/// let lambda = Meters::new(5.5e-7);
/// let e_rad = ErgsPerSecondSquareCentimeterSteradianAngstrom::new(1.0);
/// let p_rad = erg_to_photon(e_rad, lambda);
/// // 1 erg at λ = 5500 Å ≈ 2.77e11 photons / (s·cm²·sr·Å)
/// assert!((p_rad.value() - 2.77e11).abs() < 1.0e10);
/// ```
#[inline]
pub fn erg_to_photon(
    energy_radiance: ErgsPerSecondSquareCentimeterSteradianAngstrom,
    lambda: crate::length::Meters,
) -> PhotonsPerSquareCentimeterSecondSteradianAngstrom {
    let lambda_angstrom = lambda.value() * 1.0e10;
    PhotonsPerSquareCentimeterSecondSteradianAngstrom::new(
        energy_radiance.value() * lambda_angstrom / HC_ERG_ANGSTROM,
    )
}

/// Convert spectral *energy* radiance in SI wavelength units
/// (`W·m⁻²·sr⁻¹·nm⁻¹`) to spectral *photon* radiance in the NSB/SkyCalc
/// optical convention (`ph·cm⁻²·ns⁻¹·sr⁻¹·nm⁻¹`).
///
/// This is the typed equivalent of the legacy `erg * wavelength_A * 5.03e7`
/// pipeline followed by `s⁻¹ → ns⁻¹`; it keeps both wavelength and radiance
/// units explicit at the call site.
#[inline]
pub fn spectral_radiance_to_photon_radiance_ns_nm(
    energy_radiance: WattsPerSquareMeterSteradianNanometer,
    lambda: crate::length::Nanometers,
) -> PhotonsPerSquareCentimeterNanosecondSteradianNanometer {
    let lambda_m = lambda.value() * 1.0e-9;
    let photons_per_s_m2_nm = energy_radiance.value() * lambda_m / HC_JOULE_METER;
    PhotonsPerSquareCentimeterNanosecondSteradianNanometer::new(photons_per_s_m2_nm * 1.0e-13)
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::length::Meters;
    use approx::assert_relative_eq;

    #[test]
    fn radiance_cgs_to_si_conversion() {
        let cgs = ErgsPerSecondSquareCentimeterSteradian::new(1.0);
        let si = cgs.to::<WattPerSquareMeterSteradian>();
        assert_relative_eq!(si.value(), 1.0e-3, max_relative = 1e-12);
    }

    #[test]
    fn spectral_radiance_cgs_to_si_conversion() {
        let cgs = ErgsPerSecondSquareCentimeterSteradianAngstrom::new(1.0);
        let si = cgs.to::<WattPerSquareMeterSteradianMeter>();
        assert_relative_eq!(si.value(), 1.0e7, max_relative = 1e-12);
        let nm = cgs.to::<WattPerSquareMeterSteradianNanometer>();
        assert_relative_eq!(nm.value(), 1.0e-2, max_relative = 1e-12);
    }

    #[test]
    fn photon_radiance_unit_conversions() {
        let cgs = PhotonsPerSquareCentimeterSecondSteradian::new(1.0);
        let si = cgs.to::<PhotonPerSquareMeterSecondSteradian>();
        assert_relative_eq!(si.value(), 1.0e4, max_relative = 1e-12);

        let ns = PhotonsPerSquareCentimeterNanosecondSteradian::new(1.0);
        let s = ns.to::<PhotonPerSquareCentimeterSecondSteradian>();
        assert_relative_eq!(s.value(), 1.0e9, max_relative = 1e-12);
    }

    #[test]
    fn erg_to_photon_at_5500_angstrom() {
        let lambda = Meters::new(5.5e-7);
        let e_rad = ErgsPerSecondSquareCentimeterSteradianAngstrom::new(1.0);
        let p_rad = erg_to_photon(e_rad, lambda);
        assert_relative_eq!(p_rad.value(), 2.768_764e11, max_relative = 1e-4);
    }

    #[test]
    fn erg_to_photon_matches_legacy_constant_within_tolerance() {
        let lambda = Meters::new(1.0e-10);
        let e_rad = ErgsPerSecondSquareCentimeterSteradianAngstrom::new(1.0);
        let p_rad = erg_to_photon(e_rad, lambda);
        let legacy = 5.03e7_f64;
        let exact = p_rad.value();
        let rel = (exact - legacy).abs() / exact;
        assert!(
            rel < 1.0e-3,
            "exact = {exact}, legacy = {legacy}, rel = {rel}"
        );
    }

    #[test]
    fn si_spectral_radiance_to_photon_radiance_matches_legacy_path() {
        let lambda = crate::length::Nanometers::new(550.0);
        let e_rad = WattsPerSquareMeterSteradianNanometer::new(1.0);
        let got = spectral_radiance_to_photon_radiance_ns_nm(e_rad, lambda);

        // Legacy NSB path:
        // 1 W m^-2 sr^-1 nm^-1 = 100 erg s^-1 cm^-2 sr^-1 A^-1
        // photons/A/s = erg * 5.03e7 * lambda_A
        // A^-1 -> nm^-1: *10; s^-1 -> ns^-1: *1e-9
        let legacy = 100.0 * 5.03e7 * 5500.0 * 10.0 * 1.0e-9;
        let rel = (got.value() - legacy).abs() / got.value();
        assert!(
            rel < 1.0e-3,
            "typed = {}, legacy = {legacy}, rel = {rel}",
            got.value()
        );
    }

    #[test]
    fn s10_value_passthrough() {
        let s = S10s::new(123.0);
        assert_eq!(s.value(), 123.0);
    }
}
