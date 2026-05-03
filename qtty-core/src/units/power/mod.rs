// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

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
//! use qtty_core::power::{Kilowatts, Watt};
//!
//! let kw = Kilowatts::new(1.0);
//! let w = kw.to::<Watt>();
//! assert_eq!(w.value(), 1000.0);
//! ```
//!
//! ## All power units (default)
//!
//! ```rust
//! use qtty_core::power::*;
//!
//! macro_rules! touch {
//!     ($T:ty, $v:expr) => {{ let q = <$T>::new($v); let _c = q; assert!(q == q); }};
//! }
//!
//! touch!(Watts, 1.0);
//! touch!(Kilowatts, 1.0);   touch!(Megawatts, 1.0);  touch!(Gigawatts, 1.0);
//! touch!(Milliwatts, 1.0);  touch!(Microwatts, 1.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export from the dimension module.
pub use crate::dimension::Power;

/// Marker trait for power units.
pub trait PowerUnit: Unit<Dim = Power> {}
impl<T: Unit<Dim = Power>> PowerUnit for T {}

#[cfg(feature = "fundamental-physics")]
mod fundamental_physics;
#[cfg(feature = "fundamental-physics")]
pub use fundamental_physics::*;
#[cfg(feature = "customary")]
mod customary;
#[cfg(feature = "customary")]
pub use customary::*;
#[cfg(feature = "astro")]
mod astro;
#[cfg(feature = "astro")]
pub use astro::*;

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

/// Canonical list of always-available (metric SI) power units.
///
/// Exported (`#[doc(hidden)]`) for use in `qtty`\'s scalar alias generation and
/// compile-time consistency checks.  Feature-gated units (astro, customary,
/// fundamental-physics) are in their sub-modules.
#[macro_export]
#[doc(hidden)]
macro_rules! power_units {
    ($cb:path) => {
        $cb!(
            Watt, Yoctowatt, Zeptowatt, Attowatt, Femtowatt, Picowatt, Nanowatt, Microwatt,
            Milliwatt, Deciwatt, Decawatt, Hectowatt, Kilowatt, Megawatt, Gigawatt, Terawatt,
            Petawatt, Exawatt, Zettawatt, Yottawatt
        );
    };
}

// Generate bidirectional From impls between base metric SI power units.
power_units!(crate::impl_unit_from_conversions);

// ─────────────────────────────────────────────────────────────────────────────
// Cross-unit ops: default (metric) units
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "cross-unit-ops")]
power_units!(crate::impl_unit_cross_unit_ops);

// ── Cross-feature: power families ────────────────────────────────────────────
#[cfg(all(feature = "astro", feature = "customary"))]
crate::__impl_from_each_extra_to_bases!(
    {SolarLuminosity}
    HorsepowerMetric, HorsepowerElectric
);
#[cfg(all(feature = "astro", feature = "customary", feature = "cross-unit-ops"))]
crate::__impl_cross_ops_each_extra_to_bases!(
    {SolarLuminosity}
    HorsepowerMetric, HorsepowerElectric
);

#[cfg(all(feature = "astro", feature = "fundamental-physics"))]
crate::__impl_from_each_extra_to_bases!(
    {SolarLuminosity}
    ErgPerSecond
);
#[cfg(all(
    feature = "astro",
    feature = "fundamental-physics",
    feature = "cross-unit-ops"
))]
crate::__impl_cross_ops_each_extra_to_bases!(
    {SolarLuminosity}
    ErgPerSecond
);

#[cfg(all(feature = "customary", feature = "fundamental-physics"))]
crate::__impl_from_each_extra_to_bases!(
    {HorsepowerMetric, HorsepowerElectric}
    ErgPerSecond
);
#[cfg(all(
    feature = "customary",
    feature = "fundamental-physics",
    feature = "cross-unit-ops"
))]
crate::__impl_cross_ops_each_extra_to_bases!(
    {HorsepowerMetric, HorsepowerElectric}
    ErgPerSecond
);

// Compile-time check: every base power unit is registered as BuiltinUnit.
#[cfg(test)]
power_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use proptest::prelude::*;

    // ─────────────────────────────────────────────────────────────────────────────
    // Basic conversions
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    #[cfg(feature = "astro")]
    fn solar_luminosity_to_watts() {
        let sol = SolarLuminosities::new(1.0);
        let w = sol.to::<Watt>();
        // 1 L☉ = 3.828e26 W
        assert_relative_eq!(w.value(), 3.828e26, max_relative = 1e-9);
    }

    #[test]
    #[cfg(feature = "astro")]
    fn watts_to_solar_luminosity() {
        let w = Watts::new(3.828e26);
        let sol = w.to::<SolarLuminosity>();
        assert_relative_eq!(sol.value(), 1.0, max_relative = 1e-9);
    }

    #[test]
    #[cfg(feature = "astro")]
    fn multiple_solar_luminosities() {
        let sol = SolarLuminosities::new(3.0);
        let w = sol.to::<Watt>();
        assert_relative_eq!(w.value(), 3.0 * 3.828e26, max_relative = 1e-9);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Solar luminosity sanity checks
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    #[cfg(feature = "astro")]
    fn solar_luminosity_ratio_sanity() {
        // RATIO should be 3.828e26
        assert_relative_eq!(SolarLuminosity::RATIO, 3.828e26, max_relative = 1e-9);
    }

    #[test]
    #[cfg(feature = "astro")]
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
    #[cfg(feature = "astro")]
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
        #[cfg(feature = "astro")]
        fn prop_roundtrip_w_sol(w in 1e20..1e30f64) {
            let original = Watts::new(w);
            let converted = original.to::<SolarLuminosity>();
            let back = converted.to::<Watt>();
            prop_assert!((back.value() - original.value()).abs() / original.value() < 1e-12);
        }
    }

    // ─── SI-prefixed watt units ──────────────────────────────────────────────

    #[test]
    fn kilowatt_to_watt() {
        let kw = Kilowatts::new(1.0);
        let w = kw.to::<Watt>();
        assert_relative_eq!(w.value(), 1_000.0, max_relative = 1e-12);
    }

    #[test]
    fn megawatt_to_kilowatt() {
        let mw = Megawatts::new(1.0);
        let kw = mw.to::<Kilowatt>();
        assert_relative_eq!(kw.value(), 1_000.0, max_relative = 1e-12);
    }

    #[test]
    fn milliwatt_to_watt() {
        let mw = Milliwatts::new(1000.0);
        let w = mw.to::<Watt>();
        assert_relative_eq!(w.value(), 1.0, max_relative = 1e-12);
    }

    // ─── Non-SI power units ──────────────────────────────────────────────────

    #[test]
    #[cfg(feature = "fundamental-physics")]
    fn erg_per_second_to_watt() {
        let erg_s = Quantity::<ErgPerSecond>::new(1e7);
        let w = erg_s.to::<Watt>();
        // 1e7 erg/s = 1 W
        assert_relative_eq!(w.value(), 1.0, max_relative = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn metric_horsepower_to_watt() {
        let ps = HorsepowerMetrics::new(1.0);
        let w = ps.to::<Watt>();
        // 1 PS = 735.49875 W
        assert_relative_eq!(w.value(), 735.498_75, max_relative = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn electric_horsepower_to_watt() {
        let hp = HorsepowerElectrics::new(1.0);
        let w = hp.to::<Watt>();
        // 1 hp_e = 746 W (exact)
        assert_relative_eq!(w.value(), 746.0, max_relative = 1e-12);
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(Watt::SYMBOL, "W");
        assert_eq!(Kilowatt::SYMBOL, "kW");
        assert_eq!(Megawatt::SYMBOL, "MW");
        #[cfg(feature = "customary")]
        assert_eq!(HorsepowerMetric::SYMBOL, "PS");
        #[cfg(feature = "fundamental-physics")]
        assert_eq!(ErgPerSecond::SYMBOL, "erg/s");
    }
}
