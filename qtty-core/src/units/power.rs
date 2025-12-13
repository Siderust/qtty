//! Power units.
//!
//! The canonical scaling unit for this dimension is [`Watt`] (`Watt::RATIO == 1.0`).
//!
//! This module focuses on completeness without baking in avoidable precision loss:
//! - Full SI prefix ladder on the watt (yocto … yotta).
//! - A small set of widely used non-SI units with unambiguous definitions.
//! - Nominal astronomical reference: solar luminosity (IAU).
//!
//! ```rust
//! use qtty_core::power::{SolarLuminosities, Watt};
//!
//! let sol = SolarLuminosities::new(1.0);
//! let w = sol.to::<Watt>();
//! assert!((w.value() - 3.828e26).abs() < 1e18);
//! ```

use crate::{Dimension, Quantity, Unit};
use qtty_derive::Unit;

/// Fundamental dimension – power.
pub enum Power {}
impl Dimension for Power {}

/// Marker trait for power units.
pub trait PowerUnit: Unit<Dim = Power> {}
impl<T: Unit<Dim = Power>> PowerUnit for T {}

/// Watt (SI coherent derived unit).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "W", dimension = Power, ratio = 1.0)]
pub struct Watt;
/// Type alias shorthand for [`Watt`].
pub type W = Watt;
/// A quantity measured in watts.
pub type Watts = Quantity<W>;
/// One watt.
pub const WATT: Watts = Watts::new(1.0);

macro_rules! si_watt {
    ($name:ident, $sym:literal, $ratio:expr, $alias:ident, $qty:ident, $one:ident) => {
        #[doc = concat!("SI-prefixed watt unit (", stringify!($ratio), " W).")]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
        #[unit(symbol = $sym, dimension = Power, ratio = $ratio)]
        pub struct $name;
        #[doc = concat!("Type alias shorthand for [`", stringify!($name), "`].")]
        pub type $alias = $name;
        #[doc = concat!("A quantity measured in ", stringify!($name), "s.")]
        pub type $qty = Quantity<$alias>;
        #[doc = concat!("One ", stringify!($name), ".")]
        pub const $one: $qty = $qty::new(1.0);
    };
}

// Full SI prefix ladder on watt
si_watt!(Yoctowatt, "yW", 1e-24, Yw, Yoctowatts, YW);
si_watt!(Zeptowatt, "zW", 1e-21, Zw, Zeptowatts, ZW);
si_watt!(Attowatt, "aW", 1e-18, Aw, Attowatts, AW);
si_watt!(Femtowatt, "fW", 1e-15, Fw, Femtowatts, FW);
si_watt!(Picowatt, "pW", 1e-12, Pw, Picowatts, PW);
si_watt!(Nanowatt, "nW", 1e-9, Nw, Nanowatts, NW);
si_watt!(Microwatt, "µW", 1e-6, Uw, Microwatts, UW);
si_watt!(Milliwatt, "mW", 1e-3, Mw, Milliwatts, MW_1);

si_watt!(Deciwatt, "dW", 1e-1, Dw, Deciwatts, DW);
si_watt!(Decawatt, "daW", 1e1, Daw, Decawatts, DAW);
si_watt!(Hectowatt, "hW", 1e2, Hw, Hectowatts, HW);
si_watt!(Kilowatt, "kW", 1e3, Kw, Kilowatts, KW);
si_watt!(Megawatt, "MW", 1e6, MW, Megawatts, MEGAWATT);
si_watt!(Gigawatt, "GW", 1e9, GW, Gigawatts, GW_1);
si_watt!(Terawatt, "TW", 1e12, TW, Terawatts, TW_1);
si_watt!(Petawatt, "PW", 1e15, PW, Petawatts, PETAWATT);
si_watt!(Exawatt, "EW", 1e18, EW, Exawatts, EW_1);
si_watt!(Zettawatt, "ZW", 1e21, ZW, Zettawatts, ZW_1);
si_watt!(Yottawatt, "YW", 1e24, YW, Yottawatts, YW_1);

/// Erg per second (`erg/s`).
///
/// Exact: `1 erg = 1e-7 J`, so `1 erg/s = 1e-7 W`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "erg/s", dimension = Power, ratio = 1e-7)]
pub struct ErgPerSecond;
/// A quantity measured in erg/s.
pub type ErgsPerSecond = Quantity<ErgPerSecond>;
/// One erg/s.
pub const ERG_PER_S: ErgsPerSecond = ErgsPerSecond::new(1.0);

/// Metric horsepower (`PS`), defined as exactly `735.49875 W`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "PS", dimension = Power, ratio = 73_549_875.0 / 100_000.0)]
pub struct HorsepowerMetric;
/// A quantity measured in metric horsepower.
pub type HorsepowerMetrics = Quantity<HorsepowerMetric>;
/// One metric horsepower.
pub const PS: HorsepowerMetrics = HorsepowerMetrics::new(1.0);

/// Electric horsepower (`hp_e`), defined as exactly `746 W`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "hp_e", dimension = Power, ratio = 746.0)]
pub struct HorsepowerElectric;
/// A quantity measured in electric horsepower.
pub type HorsepowerElectrics = Quantity<HorsepowerElectric>;
/// One electric horsepower.
pub const HP_E: HorsepowerElectrics = HorsepowerElectrics::new(1.0);

/// Solar luminosity (IAU nominal constant; watts per L☉).
///
/// This is a *nominal reference* value intended for consistent conversion.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "L☉", dimension = Power, ratio = 3.828e26)]
pub struct SolarLuminosity;
/// A quantity measured in solar luminosities.
pub type SolarLuminosities = Quantity<SolarLuminosity>;
/// One solar luminosity.
pub const L_SUN: SolarLuminosities = SolarLuminosities::new(1.0);

// Generate all bidirectional From implementations between power units
crate::impl_unit_conversions!(
    Watt,
    Yoctowatt,
    Zeptowatt,
    Attowatt,
    Femtowatt,
    Picowatt,
    Nanowatt,
    Microwatt,
    Milliwatt,
    Deciwatt,
    Decawatt,
    Hectowatt,
    Kilowatt,
    Megawatt,
    Gigawatt,
    Terawatt,
    Petawatt,
    Exawatt,
    Zettawatt,
    Yottawatt,
    ErgPerSecond,
    HorsepowerMetric,
    HorsepowerElectric,
    SolarLuminosity
);

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
