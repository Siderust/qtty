// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Amount-of-substance units (feature: `chemistry`).
//!
//! The canonical scaling unit for this dimension is the **mole**
//! (`Mole::RATIO == 1.0`). All other amount units are expressed as
//! exact ratios to moles.
//!
//! ```rust
//! use qtty_core::amount::{Millimoles, Mole};
//!
//! let mmol = Millimoles::new(1_000.0);
//! let mol = mmol.to::<Mole>();
//! assert_eq!(mol.value(), 1.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export the amount-of-substance dimension from the dimension module.
pub use crate::dimension::AmountOfSubstance;

/// Marker trait for any [`Unit`] whose dimension is [`AmountOfSubstance`].
pub trait AmountUnit: Unit<Dim = AmountOfSubstance> {}
impl<T: Unit<Dim = AmountOfSubstance>> AmountUnit for T {}

// ─────────────────────────────────────────────────────────────────────────────
// SI mole
// ─────────────────────────────────────────────────────────────────────────────

/// Mole — SI base unit of amount of substance (mol).
///
/// Redefined in the 2019 SI revision as exactly 6.022 140 76 × 10²³ elementary
/// entities (Avogadro number).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mol", dimension = AmountOfSubstance, ratio = 1.0)]
pub struct Mole;
/// Type alias shorthand for [`Mole`].
pub type Mol = Mole;
/// A quantity measured in moles.
pub type Moles = Quantity<Mol>;
/// One mole.
pub const MOLE: Moles = Moles::new(1.0);

macro_rules! si_mole {
    ($name:ident, $sym:literal, $ratio:expr, $alias:ident, $qty:ident, $one:ident) => {
        #[doc = concat!("SI-prefixed mole unit (", stringify!($ratio), " mol).")]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
        #[unit(symbol = $sym, dimension = AmountOfSubstance, ratio = $ratio)]
        pub struct $name;
        #[doc = concat!("Type alias shorthand for [`", stringify!($name), "`].")]
        pub type $alias = $name;
        #[doc = concat!("A quantity measured in ", stringify!($name), ".")]
        pub type $qty = Quantity<$alias>;
        #[doc = concat!("One ", stringify!($name), ".")]
        pub const $one: $qty = $qty::new(1.0);
    };
}

si_mole!(Nanomole, "nmol", 1e-9, Nmol, Nanomoles, NANOMOLE);
si_mole!(Micromole, "µmol", 1e-6, Umol, Micromoles, MICROMOLE);
si_mole!(Millimole, "mmol", 1e-3, Mmol, Millimoles, MILLIMOLE);
si_mole!(Kilomole, "kmol", 1e3, Kmol, Kilomoles, KILOMOLE);

// ─────────────────────────────────────────────────────────────────────────────
// Unit inventory macro
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical list of amount-of-substance units.
#[macro_export]
#[doc(hidden)]
macro_rules! amount_units {
    ($cb:path) => {
        $cb!(Mole, Nanomole, Micromole, Millimole, Kilomole);
    };
}

amount_units!(crate::impl_unit_from_conversions);

#[cfg(feature = "cross-unit-ops")]
amount_units!(crate::impl_unit_cross_unit_ops);

#[cfg(test)]
amount_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn millimole_to_mole() {
        let mmol = Millimoles::new(1_000.0);
        let mol: Moles = mmol.to();
        assert_abs_diff_eq!(mol.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn kilomole_to_mole() {
        let kmol = Kilomoles::new(1.0);
        let mol: Moles = kmol.to();
        assert_abs_diff_eq!(mol.value(), 1_000.0, epsilon = 1e-9);
    }

    #[test]
    fn micromole_to_nanomole() {
        let umol = Micromoles::new(1.0);
        let nmol: Nanomoles = umol.to();
        assert_abs_diff_eq!(nmol.value(), 1_000.0, epsilon = 1e-9);
    }
}
