//! # Mass Units Module
//!
//! This module provides types and utilities for handling mass-related calculations
//! in astronomical and scientific contexts. It includes representations for various
//! mass units and conversions between them.
//!
//! ## Features
//! - **Kilograms (kg)**: The SI base unit of mass, with arithmetic operations.
//! - **Solar Masses (M☉)**: The mass of the Sun, commonly used in astronomy.
//!
//! ## Example Usage
//! ```rust
//! use siderust_units::{Kilograms, SolarMasses};
//!
//! let m_kg = Kilograms::new(5.0);
//! assert_eq!(m_kg.value(), 5.0);
//!
//! let m_solar = SolarMasses::new(2.0);
//! assert_eq!(m_solar.value(), 2.0);
//! ```

use super::*;

pub enum Mass {}
impl Dimension for Mass {}
pub trait MassUnit: Unit<Dim = Mass> {}
impl<T: Unit<Dim = Mass>> MassUnit for T {}

define_unit!("g", Gram, Mass, 1.0);
pub type Grams = Quantity<Gram>;

define_unit!("Kg", Kilogram, Mass, 1_000.0);
pub type Kg = Kilogram;
pub type Kilograms = Quantity<Kg>;
pub const KG: Kilograms = Kilograms::new(1.0);

define_unit!("M☉", SolarMass, Mass, 1.98847e33);
pub type SolarMasses = Quantity<SolarMass>;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn gram_to_kilogram() {
        let g = Grams::new(1000.0);
        let kg = g.to::<Kilogram>();
        assert_abs_diff_eq!(kg.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn kilogram_to_gram() {
        let kg = Kilograms::new(1.0);
        let g = kg.to::<Gram>();
        assert_abs_diff_eq!(g.value(), 1000.0, epsilon = 1e-9);
    }

    #[test]
    fn solar_mass_to_grams() {
        let sm = SolarMasses::new(1.0);
        let g = sm.to::<Gram>();
        // 1 M☉ ≈ 1.98847e33 grams
        assert_relative_eq!(g.value(), 1.98847e33, max_relative = 1e-5);
    }

    #[test]
    fn solar_mass_to_kilograms() {
        let sm = SolarMasses::new(1.0);
        let kg = sm.to::<Kilogram>();
        // 1 M☉ ≈ 1.98847e30 kg
        assert_relative_eq!(kg.value(), 1.98847e30, max_relative = 1e-5);
    }

    #[test]
    fn kilograms_to_solar_mass() {
        // Earth mass ≈ 5.97e24 kg ≈ 3e-6 M☉
        let earth_kg = Kilograms::new(5.97e24);
        let earth_sm = earth_kg.to::<SolarMass>();
        assert_relative_eq!(earth_sm.value(), 3.0e-6, max_relative = 0.01);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Solar mass sanity checks
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn solar_mass_ratio_sanity() {
        // 1 M☉ = 1.98847e33 g, so RATIO should be that value
        assert_relative_eq!(SolarMass::RATIO, 1.98847e33, max_relative = 1e-5);
    }

    #[test]
    fn solar_mass_order_of_magnitude() {
        // The Sun's mass is about 2e30 kg
        let sun = SolarMasses::new(1.0);
        let kg = sun.to::<Kilogram>();
        // Should be between 1e30 and 1e31
        assert!(kg.value() > 1e30);
        assert!(kg.value() < 1e31);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_g_kg() {
        let original = Grams::new(12345.0);
        let kg = original.to::<Kilogram>();
        let back = kg.to::<Gram>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-9);
    }

    #[test]
    fn roundtrip_kg_solar_mass() {
        let original = Kilograms::new(1e25);
        let sm = original.to::<SolarMass>();
        let back = sm.to::<Kilogram>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Unit constant
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unit_constant_kg() {
        assert_eq!(KG.value(), 1.0);
    }

    #[test]
    fn constant_arithmetic() {
        let five_kg = 5.0 * KG;
        assert_eq!(five_kg.value(), 5.0);

        let grams = five_kg.to::<Gram>();
        assert_abs_diff_eq!(grams.value(), 5000.0, epsilon = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_grams() {
        let g = Grams::new(500.0);
        assert_eq!(format!("{}", g), "500 \"g\"");
    }

    #[test]
    fn display_kilograms() {
        let kg = Kilograms::new(75.5);
        assert_eq!(format!("{}", kg), "75.5 \"Kg\"");
    }

    #[test]
    fn display_solar_mass() {
        let sm = SolarMasses::new(1.0);
        assert_eq!(format!("{}", sm), "1 \"M☉\"");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn zero_mass() {
        let zero = Grams::new(0.0);
        assert_eq!(zero.to::<Kilogram>().value(), 0.0);
        assert_eq!(zero.to::<SolarMass>().value(), 0.0);
    }

    #[test]
    fn very_small_mass() {
        // Electron mass ≈ 9.1e-28 g
        let electron = Grams::new(9.1e-28);
        let kg = electron.to::<Kilogram>();
        assert_relative_eq!(kg.value(), 9.1e-31, max_relative = 1e-9);
    }

    #[test]
    fn very_large_mass() {
        // Milky Way mass ≈ 1.5e12 M☉
        let galaxy = SolarMasses::new(1.5e12);
        let kg = galaxy.to::<Kilogram>();
        // Should be about 3e42 kg
        assert!(kg.value() > 1e42);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_g_kg_roundtrip(g_val in -1e15..1e15f64) {
            let g = Grams::new(g_val);
            let kg = g.to::<Kilogram>();
            let back = kg.to::<Gram>();
            assert_relative_eq!(back.value(), g.value(), max_relative = 1e-12);
        }

        #[test]
        fn prop_g_to_kg_ratio(g_val in -1e15..1e15f64) {
            let g = Grams::new(g_val);
            let kg = g.to::<Kilogram>();
            assert_relative_eq!(kg.value(), g_val / 1000.0, max_relative = 1e-12);
        }

        #[test]
        fn prop_kg_sm_roundtrip(kg_val in 1e20..1e35f64) {
            let kg = Kilograms::new(kg_val);
            let sm = kg.to::<SolarMass>();
            let back = sm.to::<Kilogram>();
            assert_relative_eq!(back.value(), kg.value(), max_relative = 1e-12);
        }
    }
}
