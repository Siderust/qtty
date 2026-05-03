// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

// ─────────────────────────────────────────────────────────────────────────────
// Fundamental physics lengths (CODATA 2022 recommended values)
// ─────────────────────────────────────────────────────────────────────────────

/// Bohr radius (`a0`). CODATA 2022 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "a0", dimension = Length, ratio = 5.291_772_105_44e-11)]
pub struct BohrRadius;
/// A quantity measured in Bohr radii.
pub type BohrRadii = Quantity<BohrRadius>;
/// One Bohr radius.
pub const A0: BohrRadii = BohrRadii::new(1.0);

/// Classical electron radius (`re`). CODATA 2022 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "re", dimension = Length, ratio = 2.817_940_320_5e-15)]
pub struct ClassicalElectronRadius;
/// A quantity measured in classical electron radii.
pub type ClassicalElectronRadii = Quantity<ClassicalElectronRadius>;
/// One classical electron radius.
pub const RE: ClassicalElectronRadii = ClassicalElectronRadii::new(1.0);

/// Planck length (`lp`). CODATA 2022 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lp", dimension = Length, ratio = 1.616_255e-35)]
pub struct PlanckLength;
/// A quantity measured in Planck lengths.
pub type PlanckLengths = Quantity<PlanckLength>;
/// One Planck length.
pub const LP: PlanckLengths = PlanckLengths::new(1.0);

/// Reduced Compton wavelength of the electron (`lambda_bar_e`). CODATA 2022 value in metres.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lambda_bar_e", dimension = Length, ratio = 3.861_592_674_4e-13)]
pub struct ElectronReducedComptonWavelength;
/// A quantity measured in reduced Compton wavelengths of the electron.
pub type ElectronReducedComptonWavelengths = Quantity<ElectronReducedComptonWavelength>;
/// One reduced Compton wavelength of the electron.
pub const LAMBDA_BAR_E: ElectronReducedComptonWavelengths =
    ElectronReducedComptonWavelengths::new(1.0);

// ── fundamental-physics ──────────────────────────────────────────────────
crate::impl_unit_from_conversions_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    BohrRadius, ClassicalElectronRadius, PlanckLength, ElectronReducedComptonWavelength
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    BohrRadius, ClassicalElectronRadius, PlanckLength, ElectronReducedComptonWavelength
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! length_fundamental_physics_units {
    ($cb:path) => {
        $cb!(
            PlanckLength,
            BohrRadius,
            ClassicalElectronRadius,
            ElectronReducedComptonWavelength,
        );
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn bohr_radius_to_meters() {
        let a0 = BohrRadii::new(1.0);
        let meters: Meters = a0.to();
        assert_abs_diff_eq!(meters.value(), 5.291_772_105_44e-11, epsilon = 1e-24);
    }

    #[test]
    fn reduced_compton_to_femtometers() {
        let lambda = ElectronReducedComptonWavelengths::new(1.0);
        let femtometers: Femtometers = lambda.to();
        assert_abs_diff_eq!(femtometers.value(), 386.159_267_44, epsilon = 1e-10);
    }

    proptest! {
        #[test]
        fn bohr_meter_roundtrip(v in -1.0e12_f64..1.0e12_f64) {
            let a0 = BohrRadii::new(v);
            let roundtrip: BohrRadii = a0.to::<Meter>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
