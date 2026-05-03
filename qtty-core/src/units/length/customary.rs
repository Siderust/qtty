// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Inch (`0.0254 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "in", dimension = Length, ratio = 254.0 / 10_000.0)]
pub struct Inch;
/// A quantity measured in inches.
pub type Inches = Quantity<Inch>;
/// One inch.
pub const INCH: Inches = Inches::new(1.0);

/// Foot (`0.3048 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ft", dimension = Length, ratio = 3048.0 / 10_000.0)]
pub struct Foot;
/// A quantity measured in feet.
pub type Feet = Quantity<Foot>;
/// One foot.
pub const FT: Feet = Feet::new(1.0);

/// Yard (`0.9144 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "yd", dimension = Length, ratio = 9144.0 / 10_000.0)]
pub struct Yard;
/// A quantity measured in yards.
pub type Yards = Quantity<Yard>;
/// One yard.
pub const YD: Yards = Yards::new(1.0);

/// (Statute) mile (`1609.344 m` exactly).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "mi", dimension = Length, ratio = 1_609_344.0 / 1_000.0)]
pub struct Mile;
/// A quantity measured in miles.
pub type Miles = Quantity<Mile>;
/// One mile.
pub const MI: Miles = Miles::new(1.0);

// ── customary ────────────────────────────────────────────────────────────
crate::impl_unit_from_conversions_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    Inch, Foot, Yard, Mile
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Meter, Decimeter, Centimeter, Millimeter, Micrometer, Nanometer, Picometer, Femtometer,
    Attometer, Zeptometer, Yoctometer, Decameter, Hectometer, Kilometer, Megameter, Gigameter,
    Terameter, Petameter, Exameter, Zettameter, Yottameter;
    Inch, Foot, Yard, Mile
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! length_customary_units {
    ($cb:path) => {
        $cb!(Inch, Foot, Yard, Mile,);
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn inch_to_meter_exact() {
        let inch = Inches::new(1.0);
        let meter: Meters = inch.to();
        assert_abs_diff_eq!(meter.value(), 0.0254, epsilon = 1e-15);
    }

    #[test]
    fn mile_to_feet() {
        let mile = Miles::new(1.0);
        let feet: Feet = mile.to();
        assert_abs_diff_eq!(feet.value(), 5_280.0, epsilon = 1e-9);
    }

    proptest! {
        #[test]
        fn foot_meter_roundtrip(v in -1.0e9_f64..1.0e9_f64) {
            let feet = Feet::new(v);
            let roundtrip: Feet = feet.to::<Meter>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
