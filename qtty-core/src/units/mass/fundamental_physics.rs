// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Unified atomic mass unit (u), a.k.a. dalton (Da).
///
/// Stored in grams using the CODATA 2022 recommended value for `m_u` in kilograms, converted by `1 kg = 1000 g`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "u", dimension = Mass, ratio = 1.660_539_068_92e-24)]
pub struct AtomicMassUnit;
/// Type alias shorthand for [`AtomicMassUnit`].
pub type Dalton = AtomicMassUnit;
/// Quantity measured in atomic mass units.
pub type AtomicMassUnits = Quantity<AtomicMassUnit>;
/// One atomic mass unit.
pub const U: AtomicMassUnits = AtomicMassUnits::new(1.0);

crate::impl_unit_from_conversions_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    AtomicMassUnit
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    AtomicMassUnit
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! mass_fundamental_physics_units {
    ($cb:path) => {
        $cb!(AtomicMassUnit,);
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn atomic_mass_unit_to_grams() {
        let u = AtomicMassUnits::new(1.0);
        let grams: Grams = u.to();
        assert_abs_diff_eq!(grams.value(), 1.660_539_068_92e-24, epsilon = 1e-36);
    }

    #[test]
    fn grams_to_atomic_mass_units() {
        let grams = Grams::new(1.660_539_068_92e-24);
        let u: AtomicMassUnits = grams.to();
        assert_abs_diff_eq!(u.value(), 1.0, epsilon = 1e-12);
    }

    proptest! {
        #[test]
        fn atomic_mass_unit_gram_roundtrip(v in -1.0e12_f64..1.0e12_f64) {
            let u = AtomicMassUnits::new(v);
            let roundtrip: AtomicMassUnits = u.to::<Gram>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
