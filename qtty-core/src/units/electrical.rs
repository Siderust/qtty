// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Electrical and magnetic units (feature: `electrical`).
//!
//! This module covers eight SI derived dimensions:
//!
//! | Dimension | SI unit | Symbol |
//! |-----------|---------|--------|
//! | [`Current`] (base) | ampere | A |
//! | [`Charge`] | coulomb | C |
//! | [`Voltage`] | volt | V |
//! | [`Resistance`] | ohm | Ω |
//! | [`Capacitance`] | farad | F |
//! | [`Inductance`] | henry | H |
//! | [`MagneticFlux`] | weber | Wb |
//! | [`MagneticFluxDensity`] | tesla | T |
//!
//! ```rust
//! use qtty_core::electrical::{Volts, Millivolts};
//!
//! let v = Volts::new(1.0);
//! let mv: Millivolts = v.to();
//! assert_eq!(mv.value(), 1_000.0);
//! ```

use crate::Quantity;
use qtty_derive::Unit;

pub use crate::dimension::{
    Capacitance, Charge, Current, Inductance, MagneticFlux, MagneticFluxDensity, Resistance,
    Voltage,
};

// ─────────────────────────────────────────────────────────────────────────────
// Current (base SI dimension I)
// ─────────────────────────────────────────────────────────────────────────────

/// Ampere — SI base unit of electric current (A).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "A", dimension = Current, ratio = 1.0)]
pub struct Ampere;
/// Type alias shorthand for [`Ampere`].
pub type Amp = Ampere;
/// A quantity measured in amperes.
pub type Amperes = Quantity<Amp>;
/// One ampere.
pub const AMPERE: Amperes = Amperes::new(1.0);

/// Microampere — 1 µA = 10⁻⁶ A.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "µA", dimension = Current, ratio = 1e-6)]
pub struct Microampere;
/// Type alias shorthand for [`Microampere`].
pub type UAmps = Microampere;
/// A quantity measured in microamperes.
pub type Microamperes = Quantity<UAmps>;
/// One microampere.
pub const MICROAMPERE: Microamperes = Microamperes::new(1.0);

/// Milliampere — 1 mA = 10⁻³ A.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mA", dimension = Current, ratio = 1e-3)]
pub struct Milliampere;
/// Type alias shorthand for [`Milliampere`].
pub type MAmps = Milliampere;
/// A quantity measured in milliamperes.
pub type Milliamperes = Quantity<MAmps>;
/// One milliampere.
pub const MILLIAMPERE: Milliamperes = Milliamperes::new(1.0);

/// Kiloampere — 1 kA = 10³ A.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kA", dimension = Current, ratio = 1e3)]
pub struct Kiloampere;
/// Type alias shorthand for [`Kiloampere`].
pub type KAmps = Kiloampere;
/// A quantity measured in kiloamperes.
pub type Kiloamperes = Quantity<KAmps>;
/// One kiloampere.
pub const KILOAMPERE: Kiloamperes = Kiloamperes::new(1.0);

/// Canonical list of current units.
#[macro_export]
#[doc(hidden)]
macro_rules! ampere_units {
    ($cb:path) => {
        $cb!(Ampere, Microampere, Milliampere, Kiloampere);
    };
}

ampere_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
ampere_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
ampere_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Charge (I¹ · T¹)
// ─────────────────────────────────────────────────────────────────────────────

/// Coulomb — SI coherent derived unit of electric charge (A·s).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "C", dimension = Charge, ratio = 1.0)]
pub struct Coulomb;
/// Type alias shorthand for [`Coulomb`].
pub type Coul = Coulomb;
/// A quantity measured in coulombs.
pub type Coulombs = Quantity<Coul>;
/// One coulomb.
pub const COULOMB: Coulombs = Coulombs::new(1.0);

/// Millicoulomb — 1 mC = 10⁻³ C.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mC", dimension = Charge, ratio = 1e-3)]
pub struct Millicoulomb;
/// A quantity measured in millicoulombs.
pub type Millicoulombs = Quantity<Millicoulomb>;
/// One millicoulomb.
pub const MILLICOULOMB: Millicoulombs = Millicoulombs::new(1.0);

/// Microcoulomb — 1 µC = 10⁻⁶ C.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "µC", dimension = Charge, ratio = 1e-6)]
pub struct Microcoulomb;
/// A quantity measured in microcoulombs.
pub type Microcoulombs = Quantity<Microcoulomb>;
/// One microcoulomb.
pub const MICROCOULOMB: Microcoulombs = Microcoulombs::new(1.0);

/// Kilocoulomb — 1 kC = 10³ C.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kC", dimension = Charge, ratio = 1e3)]
pub struct Kilocoulomb;
/// A quantity measured in kilocoulombs.
pub type Kilocoulombs = Quantity<Kilocoulomb>;
/// One kilocoulomb.
pub const KILOCOULOMB: Kilocoulombs = Kilocoulombs::new(1.0);

/// Canonical list of charge units.
#[macro_export]
#[doc(hidden)]
macro_rules! coulomb_units {
    ($cb:path) => {
        $cb!(Coulomb, Millicoulomb, Microcoulomb, Kilocoulomb);
    };
}

coulomb_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
coulomb_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
coulomb_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Voltage (M¹ · L² · T⁻³ · I⁻¹)
// ─────────────────────────────────────────────────────────────────────────────

/// Volt — SI coherent derived unit of voltage (kg·m²·s⁻³·A⁻¹).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "V", dimension = Voltage, ratio = 1.0)]
pub struct Volt;
/// A quantity measured in volts.
pub type Volts = Quantity<Volt>;
/// One volt.
pub const VOLT: Volts = Volts::new(1.0);

/// Microvolt — 1 µV = 10⁻⁶ V.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "µV", dimension = Voltage, ratio = 1e-6)]
pub struct Microvolt;
/// A quantity measured in microvolts.
pub type Microvolts = Quantity<Microvolt>;
/// One microvolt.
pub const MICROVOLT: Microvolts = Microvolts::new(1.0);

/// Millivolt — 1 mV = 10⁻³ V.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mV", dimension = Voltage, ratio = 1e-3)]
pub struct Millivolt;
/// A quantity measured in millivolts.
pub type Millivolts = Quantity<Millivolt>;
/// One millivolt.
pub const MILLIVOLT: Millivolts = Millivolts::new(1.0);

/// Kilovolt — 1 kV = 10³ V.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kV", dimension = Voltage, ratio = 1e3)]
pub struct Kilovolt;
/// A quantity measured in kilovolts.
pub type Kilovolts = Quantity<Kilovolt>;
/// One kilovolt.
pub const KILOVOLT: Kilovolts = Kilovolts::new(1.0);

/// Megavolt — 1 MV = 10⁶ V.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "MV", dimension = Voltage, ratio = 1e6)]
pub struct Megavolt;
/// A quantity measured in megavolts.
pub type Megavolts = Quantity<Megavolt>;
/// One megavolt.
pub const MEGAVOLT: Megavolts = Megavolts::new(1.0);

/// Canonical list of voltage units.
#[macro_export]
#[doc(hidden)]
macro_rules! volt_units {
    ($cb:path) => {
        $cb!(Volt, Microvolt, Millivolt, Kilovolt, Megavolt);
    };
}

volt_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
volt_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
volt_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Resistance (M¹ · L² · T⁻³ · I⁻²)
// ─────────────────────────────────────────────────────────────────────────────

/// Ohm — SI coherent derived unit of electrical resistance (V/A).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Ω", dimension = Resistance, ratio = 1.0)]
pub struct Ohm;
/// A quantity measured in ohms.
pub type Ohms = Quantity<Ohm>;
/// One ohm.
pub const OHM: Ohms = Ohms::new(1.0);

/// Milliohm — 1 mΩ = 10⁻³ Ω.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mΩ", dimension = Resistance, ratio = 1e-3)]
pub struct Milliohm;
/// A quantity measured in milliohms.
pub type Milliohms = Quantity<Milliohm>;
/// One milliohm.
pub const MILLIOHM: Milliohms = Milliohms::new(1.0);

/// Kilohm — 1 kΩ = 10³ Ω.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "kΩ", dimension = Resistance, ratio = 1e3)]
pub struct Kilohm;
/// A quantity measured in kilohms.
pub type Kilohms = Quantity<Kilohm>;
/// One kilohm.
pub const KILOHM: Kilohms = Kilohms::new(1.0);

/// Megaohm — 1 MΩ = 10⁶ Ω.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "MΩ", dimension = Resistance, ratio = 1e6)]
pub struct Megaohm;
/// A quantity measured in megaohms.
pub type Megaohms = Quantity<Megaohm>;
/// One megaohm.
pub const MEGAOHM: Megaohms = Megaohms::new(1.0);

/// Canonical list of resistance units.
#[macro_export]
#[doc(hidden)]
macro_rules! ohm_units {
    ($cb:path) => {
        $cb!(Ohm, Milliohm, Kilohm, Megaohm);
    };
}

ohm_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
ohm_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
ohm_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Capacitance (M⁻¹ · L⁻² · T⁴ · I²)
// ─────────────────────────────────────────────────────────────────────────────

/// Farad — SI coherent derived unit of electrical capacitance (C/V).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "F", dimension = Capacitance, ratio = 1.0)]
pub struct Farad;
/// A quantity measured in farads.
pub type Farads = Quantity<Farad>;
/// One farad.
pub const FARAD: Farads = Farads::new(1.0);

/// Picofarad — 1 pF = 10⁻¹² F.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "pF", dimension = Capacitance, ratio = 1e-12)]
pub struct Picofarad;
/// A quantity measured in picofarads.
pub type Picofarads = Quantity<Picofarad>;
/// One picofarad.
pub const PICOFARAD: Picofarads = Picofarads::new(1.0);

/// Nanofarad — 1 nF = 10⁻⁹ F.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "nF", dimension = Capacitance, ratio = 1e-9)]
pub struct Nanofarad;
/// A quantity measured in nanofarads.
pub type Nanofarads = Quantity<Nanofarad>;
/// One nanofarad.
pub const NANOFARAD: Nanofarads = Nanofarads::new(1.0);

/// Microfarad — 1 µF = 10⁻⁶ F.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "µF", dimension = Capacitance, ratio = 1e-6)]
pub struct Microfarad;
/// A quantity measured in microfarads.
pub type Microfarads = Quantity<Microfarad>;
/// One microfarad.
pub const MICROFARAD: Microfarads = Microfarads::new(1.0);

/// Millifarad — 1 mF = 10⁻³ F.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mF", dimension = Capacitance, ratio = 1e-3)]
pub struct Millifarad;
/// A quantity measured in millifarads.
pub type Millifarads = Quantity<Millifarad>;
/// One millifarad.
pub const MILLIFARAD: Millifarads = Millifarads::new(1.0);

/// Canonical list of capacitance units.
#[macro_export]
#[doc(hidden)]
macro_rules! farad_units {
    ($cb:path) => {
        $cb!(Farad, Picofarad, Nanofarad, Microfarad, Millifarad);
    };
}

farad_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
farad_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
farad_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Inductance (M¹ · L² · T⁻² · I⁻²)
// ─────────────────────────────────────────────────────────────────────────────

/// Henry — SI coherent derived unit of electrical inductance (V·s/A).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "H", dimension = Inductance, ratio = 1.0)]
pub struct Henry;
/// A quantity measured in henries.
pub type Henries = Quantity<Henry>;
/// One henry.
pub const HENRY: Henries = Henries::new(1.0);

/// Microhenry — 1 µH = 10⁻⁶ H.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "µH", dimension = Inductance, ratio = 1e-6)]
pub struct Microhenry;
/// A quantity measured in microhenries.
pub type Microhenries = Quantity<Microhenry>;
/// One microhenry.
pub const MICROHENRY: Microhenries = Microhenries::new(1.0);

/// Millihenry — 1 mH = 10⁻³ H.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mH", dimension = Inductance, ratio = 1e-3)]
pub struct Millihenry;
/// A quantity measured in millihenries.
pub type Millihenries = Quantity<Millihenry>;
/// One millihenry.
pub const MILLIHENRY: Millihenries = Millihenries::new(1.0);

/// Canonical list of inductance units.
#[macro_export]
#[doc(hidden)]
macro_rules! henry_units {
    ($cb:path) => {
        $cb!(Henry, Microhenry, Millihenry);
    };
}

henry_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
henry_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
henry_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Magnetic flux (M¹ · L² · T⁻² · I⁻¹)
// ─────────────────────────────────────────────────────────────────────────────

/// Weber — SI coherent derived unit of magnetic flux (V·s).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Wb", dimension = MagneticFlux, ratio = 1.0)]
pub struct Weber;
/// A quantity measured in webers.
pub type Webers = Quantity<Weber>;
/// One weber.
pub const WEBER: Webers = Webers::new(1.0);

/// Milliweber — 1 mWb = 10⁻³ Wb.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mWb", dimension = MagneticFlux, ratio = 1e-3)]
pub struct Milliweber;
/// A quantity measured in milliwebers.
pub type Milliwebers = Quantity<Milliweber>;
/// One milliweber.
pub const MILLIWEBER: Milliwebers = Milliwebers::new(1.0);

/// Canonical list of magnetic flux units.
#[macro_export]
#[doc(hidden)]
macro_rules! weber_units {
    ($cb:path) => {
        $cb!(Weber, Milliweber);
    };
}

weber_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
weber_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
weber_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Magnetic flux density (M¹ · T⁻² · I⁻¹)
// ─────────────────────────────────────────────────────────────────────────────

/// Tesla — SI coherent derived unit of magnetic flux density (Wb/m²).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "T", dimension = MagneticFluxDensity, ratio = 1.0)]
pub struct Tesla;
/// A quantity measured in teslas.
pub type Teslas = Quantity<Tesla>;
/// One tesla.
pub const TESLA: Teslas = Teslas::new(1.0);

/// Millitesla — 1 mT = 10⁻³ T.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mT", dimension = MagneticFluxDensity, ratio = 1e-3)]
pub struct Millitesla;
/// A quantity measured in milliteslas.
pub type Milliteslas = Quantity<Millitesla>;
/// One millitesla.
pub const MILLITESLA: Milliteslas = Milliteslas::new(1.0);

/// Microtesla — 1 µT = 10⁻⁶ T.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "µT", dimension = MagneticFluxDensity, ratio = 1e-6)]
pub struct Microtesla;
/// A quantity measured in microteslas.
pub type Microteslas = Quantity<Microtesla>;
/// One microtesla.
pub const MICROTESLA: Microteslas = Microteslas::new(1.0);

/// Canonical list of magnetic flux density units.
#[macro_export]
#[doc(hidden)]
macro_rules! tesla_units {
    ($cb:path) => {
        $cb!(Tesla, Millitesla, Microtesla);
    };
}

tesla_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
tesla_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
tesla_units!(crate::assert_units_are_builtin);

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    // Current
    #[test]
    fn milliampere_to_ampere() {
        let ma = Milliamperes::new(1_000.0);
        let a: Amperes = ma.to();
        assert_abs_diff_eq!(a.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn microampere_to_milliampere() {
        let ua = Microamperes::new(1_000.0);
        let ma: Milliamperes = ua.to();
        assert_abs_diff_eq!(ma.value(), 1.0, epsilon = 1e-12);
    }

    // Charge
    #[test]
    fn millicoulomb_to_coulomb() {
        let mc = Millicoulombs::new(1_000.0);
        let c: Coulombs = mc.to();
        assert_abs_diff_eq!(c.value(), 1.0, epsilon = 1e-12);
    }

    // Voltage
    #[test]
    fn millivolt_to_volt() {
        let mv = Millivolts::new(1_000.0);
        let v: Volts = mv.to();
        assert_abs_diff_eq!(v.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn kilovolt_to_volt() {
        let kv = Kilovolts::new(1.0);
        let v: Volts = kv.to();
        assert_abs_diff_eq!(v.value(), 1_000.0, epsilon = 1e-9);
    }

    // Resistance
    #[test]
    fn kilohm_to_ohm() {
        let ko = Kilohms::new(1.0);
        let o: Ohms = ko.to();
        assert_abs_diff_eq!(o.value(), 1_000.0, epsilon = 1e-9);
    }

    // Capacitance
    #[test]
    fn microfarad_to_farad() {
        let uf = Microfarads::new(1.0);
        let f: Farads = uf.to();
        assert_abs_diff_eq!(f.value(), 1e-6, epsilon = 1e-18);
    }

    // Inductance
    #[test]
    fn millihenry_to_henry() {
        let mh = Millihenries::new(1_000.0);
        let h: Henries = mh.to();
        assert_abs_diff_eq!(h.value(), 1.0, epsilon = 1e-12);
    }

    // Magnetic flux
    #[test]
    fn milliweber_to_weber() {
        let mwb = Milliwebers::new(1_000.0);
        let wb: Webers = mwb.to();
        assert_abs_diff_eq!(wb.value(), 1.0, epsilon = 1e-12);
    }

    // Magnetic flux density
    #[test]
    fn millitesla_to_tesla() {
        let mt = Milliteslas::new(1_000.0);
        let t: Teslas = mt.to();
        assert_abs_diff_eq!(t.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn microtesla_earth_field() {
        // Earth's magnetic field ≈ 25–65 µT
        let ut = Microteslas::new(50.0);
        let t: Teslas = ut.to();
        assert_abs_diff_eq!(t.value(), 50e-6, epsilon = 1e-18);
    }
}
