// SPDX-License-Identifier: AGPL-3.0-or-later
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

use crate::Quantity;
use qtty_derive::Unit;

pub use crate::dimension::{
    InverseSolidAngle, PhotonRadiance, Radiance, SpectralPhotonRadiance, SpectralRadiance,
};

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
// Conversion helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Photon energy `E = h · c` in **erg · ångström** units (CODATA 2018).
///
/// `h · c` ≈ `1.986_445_857 × 10⁻⁸ erg·Å`. The reciprocal,
/// `5.034_116_5 × 10⁷ ph / (erg · Å)`, was historically rounded to
/// `5.03e7` in `NSB_Utils.py`.
const HC_ERG_ANGSTROM: f64 = 1.986_445_857_148_968e-8;

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
        assert!(rel < 1.0e-3, "exact = {exact}, legacy = {legacy}, rel = {rel}");
    }

    #[test]
    fn s10_value_passthrough() {
        let s = S10s::new(123.0);
        assert_eq!(s.value(), 123.0);
    }
}
