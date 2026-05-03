// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Carat: `1 ct = 0.2 g` (exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ct", dimension = Mass, ratio = 1.0 / 5.0)]
pub struct Carat;
/// Shorthand type alias for [`Carat`].
pub type Ct = Carat;
/// Quantity measured in carats.
pub type Carats = Quantity<Ct>;
/// One carat.
pub const CT: Carats = Carats::new(1.0);

/// Grain: `1 gr = 64.79891 mg` (exact) == `0.064_798_91 g`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "gr", dimension = Mass, ratio = 6_479_891.0 / 100_000_000.0)]
pub struct Grain;
/// Shorthand type alias for [`Grain`].
pub type Gr = Grain;
/// Quantity measured in grains.
pub type Grains = Quantity<Gr>;
/// One grain.
pub const GR: Grains = Grains::new(1.0);

/// Avoirdupois pound: `1 lb = 0.45359237 kg` (exact) == `453.59237 g`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "lb", dimension = Mass, ratio = 45_359_237.0 / 100_000.0)]
pub struct Pound;
/// Shorthand type alias for [`Pound`].
pub type Lb = Pound;
/// Quantity measured in pounds.
pub type Pounds = Quantity<Lb>;
/// One pound.
pub const LB: Pounds = Pounds::new(1.0);

/// Avoirdupois ounce: `1 oz = 1/16 lb` (exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "oz", dimension = Mass, ratio = (45_359_237.0 / 100_000.0) / 16.0)]
pub struct Ounce;
/// Shorthand type alias for [`Ounce`].
pub type Oz = Ounce;
/// Quantity measured in ounces.
pub type Ounces = Quantity<Oz>;
/// One ounce.
pub const OZ: Ounces = Ounces::new(1.0);

/// Avoirdupois stone: `1 st = 14 lb` (exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "st", dimension = Mass, ratio = (45_359_237.0 / 100_000.0) * 14.0)]
pub struct Stone;
/// Shorthand type alias for [`Stone`].
pub type St = Stone;
/// Quantity measured in stones.
pub type Stones = Quantity<St>;
/// One stone.
pub const ST: Stones = Stones::new(1.0);

/// Short ton (US customary): `2000 lb` (exact given lb).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ton_us", dimension = Mass, ratio = (45_359_237.0 / 100_000.0) * 2000.0)]
pub struct ShortTon;
/// Quantity measured in short tons (US).
pub type ShortTons = Quantity<ShortTon>;
/// One short ton (US).
pub const TON_US: ShortTons = ShortTons::new(1.0);

/// Long ton (Imperial): `2240 lb` (exact given lb).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "ton_uk", dimension = Mass, ratio = (45_359_237.0 / 100_000.0) * 2240.0)]
pub struct LongTon;
/// Quantity measured in long tons (UK).
pub type LongTons = Quantity<LongTon>;
/// One long ton (UK).
pub const TON_UK: LongTons = LongTons::new(1.0);

crate::impl_unit_from_conversions_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    Carat, Grain, Pound, Ounce, Stone, ShortTon, LongTon
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
    Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
    Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne;
    Carat, Grain, Pound, Ounce, Stone, ShortTon, LongTon
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! mass_customary_units {
    ($cb:path) => {
        $cb!(Carat, Grain, Pound, Ounce, Stone, ShortTon, LongTon,);
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    #[test]
    fn pound_to_grams() {
        let pounds = Pounds::new(1.0);
        let grams: Grams = pounds.to();
        assert_abs_diff_eq!(grams.value(), 453.592_37, epsilon = 1e-10);
    }

    #[test]
    fn stone_to_pounds() {
        let stone = Stones::new(1.0);
        let pounds: Pounds = stone.to();
        assert_abs_diff_eq!(pounds.value(), 14.0, epsilon = 1e-12);
    }

    proptest! {
        #[test]
        fn ounce_gram_roundtrip(v in -1.0e9_f64..1.0e9_f64) {
            let ounces = Ounces::new(v);
            let roundtrip: Ounces = ounces.to::<Gram>().to();
            prop_assert!((roundtrip.value() - v).abs() <= v.abs().max(1.0) * 1e-12);
        }
    }
}
