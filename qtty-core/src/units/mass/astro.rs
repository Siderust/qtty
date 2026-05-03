// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Nominal solar mass (IAU 2015 Resolution B3; grams per M☉).
///
/// This is a **conversion constant** (nominal), not a “best estimate” of the Sun’s true mass.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "M☉", dimension = Mass, ratio = 1.988_416e33)]
pub struct SolarMass;
/// A quantity measured in solar masses.
pub type SolarMasses = Quantity<SolarMass>;
/// One nominal solar mass.
pub const MSUN: SolarMasses = SolarMasses::new(1.0);

crate::impl_unit_from_conversions_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    SolarMass
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    SolarMass
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! mass_astro_units {
    ($cb:path) => {
        $cb!(SolarMass,);
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn solar_mass_to_grams() {
        let solar = SolarMasses::new(1.0);
        let grams: Grams = solar.to();
        assert_abs_diff_eq!(grams.value(), 1.988_416e33, epsilon = 1e20);
    }

    #[test]
    fn kilograms_to_solar_mass() {
        let kg = Kilograms::new(1.988_416e30);
        let solar: SolarMasses = kg.to();
        assert_abs_diff_eq!(solar.value(), 1.0, epsilon = 1e-12);
    }

    proptest! {
        #[test]
        fn solar_mass_gram_roundtrip(v in -1.0e6_f64..1.0e6_f64) {
            let solar = SolarMasses::new(v);
            let roundtrip: SolarMasses = solar.to::<Gram>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
