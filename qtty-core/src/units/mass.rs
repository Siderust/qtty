//! Mass units.
//!
//! The canonical scaling unit for this dimension is [`Gram`] (`Gram::RATIO == 1.0`).
//!
//! ```rust
//! use qtty_core::mass::{Kilograms, SolarMass};
//!
//! let m = Kilograms::new(1.0);
//! let sm = m.to::<SolarMass>();
//! assert!(sm.value() < 1.0);
//! ```

use crate::{Dimension, Quantity, Unit};
use qtty_derive::Unit;

/// Dimension tag for mass.
pub enum Mass {}
impl Dimension for Mass {}
/// Marker trait for any [`Unit`] whose dimension is [`Mass`].
pub trait MassUnit: Unit<Dim = Mass> {}
impl<T: Unit<Dim = Mass>> MassUnit for T {}

/// Gram.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "g", dimension = Mass, ratio = 1.0)]
pub struct Gram;
/// A quantity measured in grams.
pub type Grams = Quantity<Gram>;

/// Kilogram (`1000 g`).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "Kg", dimension = Mass, ratio = 1_000.0)]
pub struct Kilogram;
/// Type alias shorthand for [`Kilogram`].
pub type Kg = Kilogram;
/// A quantity measured in kilograms.
pub type Kilograms = Quantity<Kg>;
/// One kilogram.
pub const KG: Kilograms = Kilograms::new(1.0);

/// Solar mass (grams per M☉).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "M☉", dimension = Mass, ratio = 1.98847e33)]
pub struct SolarMass;
/// A quantity measured in solar masses.
pub type SolarMasses = Quantity<SolarMass>;

// Generate all bidirectional From implementations between mass units
crate::impl_unit_conversions!(Gram, Kilogram, SolarMass);

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
