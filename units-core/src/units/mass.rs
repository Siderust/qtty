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
//! ```rust,ignore
//! use crate::{Kilograms, SolarMasses};
//!
//! let m_kg = Kilograms::new(5.0);
//! assert_eq!(m_kg.value(), 5.0);
//!
//! let m_solar = SolarMasses::new(2.0);
//! assert_eq!(m_solar.value(), 2.0);
//! ```

use crate::{Dimension, Quantity, Unit};
use units_derive::Unit;

pub enum Mass {}
impl Dimension for Mass {}
pub trait MassUnit: Unit<Dim = Mass> {}
impl<T: Unit<Dim = Mass>> MassUnit for T {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "g", dimension = Mass, ratio = 1.0)]
pub struct Gram;
pub type Grams = Quantity<Gram>;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Kg", dimension = Mass, ratio = 1_000.0)]
pub struct Kilogram;
pub type Kg = Kilogram;
pub type Kilograms = Quantity<Kg>;
pub const KG: Kilograms = Kilograms::new(1.0);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "M☉", dimension = Mass, ratio = 1.98847e33)]
pub struct SolarMass;
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
        assert!(kg.value() > 1e30);
        assert!(kg.value() < 1e31);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_g_kg() {
        let original = Grams::new(5000.0);
        let converted = original.to::<Kilogram>();
        let back = converted.to::<Gram>();
        assert_abs_diff_eq!(back.value(), original.value(), epsilon = 1e-9);
    }

    #[test]
    fn roundtrip_kg_solar() {
        let original = Kilograms::new(1e30);
        let converted = original.to::<SolarMass>();
        let back = converted.to::<Kilogram>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_g_kg(g in 1e-6..1e6f64) {
            let original = Grams::new(g);
            let converted = original.to::<Kilogram>();
            let back = converted.to::<Gram>();
            prop_assert!((back.value() - original.value()).abs() < 1e-9 * g.abs().max(1.0));
        }

        #[test]
        fn prop_g_kg_ratio(g in 1e-6..1e6f64) {
            let grams = Grams::new(g);
            let kg = grams.to::<Kilogram>();
            // 1000 g = 1 kg
            prop_assert!((grams.value() / kg.value() - 1000.0).abs() < 1e-9);
        }
    }
}
