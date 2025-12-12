//! # Power Units Module
//!
//! Commonly used power units in astronomy.
//!
//! ## Units
//! - **Watt (W)**: SI base unit of power.
//! - **Solar Luminosity (L☉)**: power radiated by the Sun.
//!
//! ## Example
//! ```rust
//! use siderust_units::*;
//!
//! // 2 kW
//! let p_w = Watts::new(2_000.0);
//!
//! // 3 L☉
//! let p_sol = SolarLuminosities::new(3.0);
//!
//! // Convenient conversion
//! let p_w_equiv = p_sol.to::<Watt>();
//! assert!((p_w_equiv.value() - 3.0 * 3.828e26).abs() < 1e15);
//! ```

use super::*;

/// Fundamental dimension – power.
pub enum Power {}
impl Dimension for Power {}

/// Marker trait for power units.
pub trait PowerUnit: Unit<Dim = Power> {}
impl<T: Unit<Dim = Power>> PowerUnit for T {}

define_unit!("W", Watt, Power, 1.0);
pub type W = Watt;
pub type Watts = Quantity<W>;
pub const WATT: Watts = Watts::new(1.0);

// 1 L☉ = 3.828 × 10²⁶ W (IAU 2015)
define_unit!("L☉", SolarLuminosity, Power, 3.828e26);
pub type SolarLuminosities = Quantity<SolarLuminosity>;
pub const L_SUN: SolarLuminosities = SolarLuminosities::new(1.0);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn solar_luminosity_to_watts() {
        let sol = SolarLuminosities::new(1.0);
        let w = sol.to::<Watt>();
        // 1 L☉ = 3.828e26 W
        assert_relative_eq!(w.value(), 3.828e26, max_relative = 1e-9);
    }

    #[test]
    fn watts_to_solar_luminosity() {
        let w = Watts::new(3.828e26);
        let sol = w.to::<SolarLuminosity>();
        assert_relative_eq!(sol.value(), 1.0, max_relative = 1e-9);
    }

    #[test]
    fn multiple_solar_luminosities() {
        let sol = SolarLuminosities::new(3.0);
        let w = sol.to::<Watt>();
        assert_relative_eq!(w.value(), 3.0 * 3.828e26, max_relative = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Solar luminosity sanity checks
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn solar_luminosity_ratio_sanity() {
        // RATIO should be 3.828e26
        assert_relative_eq!(SolarLuminosity::RATIO, 3.828e26, max_relative = 1e-9);
    }

    #[test]
    fn solar_luminosity_order_of_magnitude() {
        let sun = SolarLuminosities::new(1.0);
        let w = sun.to::<Watt>();
        // Should be between 1e26 and 1e27
        assert!(w.value() > 1e26);
        assert!(w.value() < 1e27);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Roundtrip conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn roundtrip_w_sol() {
        let original = Watts::new(1e20);
        let sol = original.to::<SolarLuminosity>();
        let back = sol.to::<Watt>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    #[test]
    fn roundtrip_sol_w() {
        let original = SolarLuminosities::new(100.0);
        let w = original.to::<Watt>();
        let back = w.to::<SolarLuminosity>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Unit constants
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn unit_constants() {
        assert_eq!(WATT.value(), 1.0);
        assert_eq!(L_SUN.value(), 1.0);
    }

    #[test]
    fn constant_arithmetic() {
        let two_suns = 2.0 * L_SUN;
        assert_eq!(two_suns.value(), 2.0);

        let watts = two_suns.to::<Watt>();
        assert_relative_eq!(watts.value(), 2.0 * 3.828e26, max_relative = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Display
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn display_watts() {
        let w = Watts::new(1000.0);
        assert_eq!(format!("{}", w), "1000 \"W\"");
    }

    #[test]
    fn display_solar_luminosity() {
        let sol = SolarLuminosities::new(1.5);
        assert_eq!(format!("{}", sol), "1.5 \"L☉\"");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Realistic astronomical values
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn sirius_luminosity() {
        // Sirius A has luminosity ≈ 25 L☉
        let sirius = SolarLuminosities::new(25.0);
        let w = sirius.to::<Watt>();
        assert_relative_eq!(w.value(), 25.0 * 3.828e26, max_relative = 1e-9);
    }

    #[test]
    fn red_dwarf_luminosity() {
        // A typical red dwarf might have 0.001 L☉
        let red_dwarf = SolarLuminosities::new(0.001);
        let w = red_dwarf.to::<Watt>();
        assert_relative_eq!(w.value(), 0.001 * 3.828e26, max_relative = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn zero_power() {
        let zero = Watts::new(0.0);
        assert_eq!(zero.to::<SolarLuminosity>().value(), 0.0);
    }

    #[test]
    fn very_small_power() {
        let tiny = Watts::new(1.0);
        let sol = tiny.to::<SolarLuminosity>();
        // Should be about 2.6e-27 L☉
        assert!(sol.value() < 1e-25);
    }

    #[test]
    fn very_large_power() {
        // A quasar might emit 1e13 L☉
        let quasar = SolarLuminosities::new(1e13);
        let w = quasar.to::<Watt>();
        // Should be about 3.8e39 W
        assert!(w.value() > 1e39);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_w_sol_roundtrip(w_val in 1e10..1e35f64) {
            let w = Watts::new(w_val);
            let sol = w.to::<SolarLuminosity>();
            let back = sol.to::<Watt>();
            assert_relative_eq!(back.value(), w.value(), max_relative = 1e-12);
        }

        #[test]
        fn prop_sol_to_w_ratio(sol_val in 1e-10..1e10f64) {
            let sol = SolarLuminosities::new(sol_val);
            let w = sol.to::<Watt>();
            assert_relative_eq!(w.value(), sol_val * 3.828e26, max_relative = 1e-9);
        }
    }
}
