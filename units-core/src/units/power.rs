//! Power units.
//!
//! The canonical scaling unit for this dimension is [`Watt`] (`Watt::RATIO == 1.0`).
//!
//! ```rust
//! use unit_core::power::{SolarLuminosities, Watt};
//!
//! let sol = SolarLuminosities::new(1.0);
//! let w = sol.to::<Watt>();
//! assert!((w.value() - 3.828e26).abs() < 1e18);
//! ```

use crate::{Dimension, Quantity, Unit};
use unit_derive::Unit;

/// Fundamental dimension – power.
pub enum Power {}
impl Dimension for Power {}

/// Marker trait for power units.
pub trait PowerUnit: Unit<Dim = Power> {}
impl<T: Unit<Dim = Power>> PowerUnit for T {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "W", dimension = Power, ratio = 1.0)]
/// Watt.
pub struct Watt;
/// Type alias shorthand for [`Watt`].
pub type W = Watt;
/// A quantity measured in watts.
pub type Watts = Quantity<W>;
/// One watt.
pub const WATT: Watts = Watts::new(1.0);

/// Solar luminosity (IAU 2015; watts per L☉).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "L☉", dimension = Power, ratio = 3.828e26)]
pub struct SolarLuminosity;
/// A quantity measured in solar luminosities.
pub type SolarLuminosities = Quantity<SolarLuminosity>;
/// One solar luminosity.
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
