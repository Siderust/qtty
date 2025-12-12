//! # Power Units Module
//!
//! Commonly used power units in astronomy.
//!
//! ## Units
//! - **Watt (W)**: SI base unit of power.
//! - **Solar Luminosity (L☉)**: power radiated by the Sun.
//!
//! ## Example
//! ```rust,ignore
//! use crate::*;
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

use crate::{Dimension, Quantity, Unit};
use units_derive::Unit;

/// Fundamental dimension – power.
pub enum Power {}
impl Dimension for Power {}

/// Marker trait for power units.
pub trait PowerUnit: Unit<Dim = Power> {}
impl<T: Unit<Dim = Power>> PowerUnit for T {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "W", dimension = Power, ratio = 1.0)]
pub struct Watt;
pub type W = Watt;
pub type Watts = Quantity<W>;
pub const WATT: Watts = Watts::new(1.0);

// 1 L☉ = 3.828 × 10²⁶ W (IAU 2015)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "L☉", dimension = Power, ratio = 3.828e26)]
pub struct SolarLuminosity;
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
        let original = Watts::new(1e26);
        let converted = original.to::<SolarLuminosity>();
        let back = converted.to::<Watt>();
        assert_relative_eq!(back.value(), original.value(), max_relative = 1e-12);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-based tests
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        #[test]
        fn prop_roundtrip_w_sol(w in 1e20..1e30f64) {
            let original = Watts::new(w);
            let converted = original.to::<SolarLuminosity>();
            let back = converted.to::<Watt>();
            prop_assert!((back.value() - original.value()).abs() / original.value() < 1e-12);
        }
    }
}
