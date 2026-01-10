//! Mass units.
//!
//! The canonical scaling unit for this dimension is [`Gram`] (`Gram::RATIO == 1.0`).
//!
//! This module aims for practical completeness while avoiding avoidable precision loss:
//! - **SI grams**: full prefix ladder (yocto … yotta).
//! - **Defined non-SI**: tonne, avoirdupois units, carat, grain.
//! - **Science/astro**: atomic mass unit (u/Da), nominal solar mass.
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
/// One gram.
pub const G: Grams = Grams::new(1.0);

/// Helper macro to declare a gram-based SI mass unit.
///
/// Each invocation of this macro defines, for a given prefix on grams:
/// - a unit struct `$name` (e.g. `Kilogram`),
/// - a shorthand type alias `$alias` (e.g. `Kg`),
/// - a quantity type `$qty` (e.g. `Kilograms`), and
/// - a constant `$one` equal to `1.0` of that quantity.
///
/// The `$ratio` argument is the conversion factor to grams, i.e.
/// `$name::RATIO` such that `1 $sym = $ratio g`.
macro_rules! si_gram {
    ($name:ident, $sym:literal, $ratio:expr, $alias:ident, $qty:ident, $one:ident) => {
        #[doc = concat!("SI mass unit `", stringify!($name), "` with gram-based prefix (symbol `", $sym,"`).")]
        #[doc = concat!("By definition, `1 ", $sym, " = ", stringify!($ratio), " g`.")]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
        #[unit(symbol = $sym, dimension = Mass, ratio = $ratio)]
        pub struct $name;

        #[doc = concat!("Shorthand alias for [`", stringify!($name), "`]." )]
        pub type $alias = $name;

        #[doc = concat!("Quantity measured in ", stringify!($name), " (",$sym,").")]
        pub type $qty = Quantity<$alias>;

        #[doc = concat!("Constant equal to one ", stringify!($name), " (1 ",$sym,").")]
        pub const $one: $qty = $qty::new(1.0);
    };
}

// Full SI prefix ladder (gram-based)
si_gram!(Yoctogram, "yg", 1e-24, Yg, Yoctograms, YG);
si_gram!(Zeptogram, "zg", 1e-21, Zg, Zeptograms, ZG);
si_gram!(Attogram, "ag", 1e-18, Ag, Attograms, AG);
si_gram!(Femtogram, "fg", 1e-15, Fg, Femtograms, FG);
si_gram!(Picogram, "pg", 1e-12, Pg, Picograms, PG);
si_gram!(Nanogram, "ng", 1e-9, Ng, Nanograms, NG);
si_gram!(Microgram, "µg", 1e-6, Ug, Micrograms, UG);
si_gram!(Milligram, "mg", 1e-3, Mg, Milligrams, MG);
si_gram!(Centigram, "cg", 1e-2, Cg, Centigrams, CG);
si_gram!(Decigram, "dg", 1e-1, Dg, Decigrams, DG);

si_gram!(Decagram, "dag", 1e1, Dag, Decagrams, DAG);
si_gram!(Hectogram, "hg", 1e2, Hg, Hectograms, HG);
si_gram!(Kilogram, "kg", 1e3, Kg, Kilograms, KG);
si_gram!(Megagram, "Mg", 1e6, MgG, Megagrams, MEGAGRAM);
si_gram!(Gigagram, "Gg", 1e9, Gg, Gigagrams, GG);
si_gram!(Teragram, "Tg", 1e12, Tg, Teragrams, TG);
si_gram!(Petagram, "Pg", 1e15, PgG, Petagrams, PETAGRAM);
si_gram!(Exagram, "Eg", 1e18, Eg, Exagrams, EG);
si_gram!(Zettagram, "Zg", 1e21, ZgG, Zettagrams, ZETTAGRAM);
si_gram!(Yottagram, "Yg", 1e24, YgG, Yottagrams, YOTTAGRAM);

/// Tonne (metric ton): `1 t = 1_000_000 g` (exact).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "t", dimension = Mass, ratio = 1_000_000.0)]
pub struct Tonne;
/// Shorthand type alias for [`Tonne`].
pub type T = Tonne;
/// Quantity measured in tonnes.
pub type Tonnes = Quantity<T>;
/// One metric tonne.
pub const TONE: Tonnes = Tonnes::new(1.0);

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
#[unit(symbol = "gr", dimension = Mass, ratio = 6_479_891.0 / 1_000_000_000.0)]
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

/// Unified atomic mass unit (u), a.k.a. dalton (Da).
///
/// Stored in grams using the CODATA recommended value for `m_u` in kilograms, converted by `1 kg = 1000 g`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "u", dimension = Mass, ratio = 1.660_539_068_92e-24)]
pub struct AtomicMassUnit;
/// Type alias shorthand for [`AtomicMassUnit`].
pub type Dalton = AtomicMassUnit;
/// Quantity measured in atomic mass units.
pub type AtomicMassUnits = Quantity<AtomicMassUnit>;
/// One atomic mass unit.
pub const U: AtomicMassUnits = AtomicMassUnits::new(1.0);

/// Nominal solar mass (IAU 2015 Resolution B3; grams per M☉).
///
/// This is a **conversion constant** (nominal), not a “best estimate” of the Sun’s true mass.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "M☉", dimension = Mass, ratio = 1.988_416e33)]
pub struct SolarMass;
/// A quantity measured in solar masses.
pub type SolarMasses = Quantity<SolarMass>;
/// One nominal solar mass.
pub const MSUN: SolarMasses = SolarMasses::new(1.0);

// Generate all bidirectional From implementations between mass units
crate::impl_unit_conversions!(
    Gram,
    Yoctogram,
    Zeptogram,
    Attogram,
    Femtogram,
    Picogram,
    Nanogram,
    Microgram,
    Milligram,
    Centigram,
    Decigram,
    Decagram,
    Hectogram,
    Kilogram,
    Megagram,
    Gigagram,
    Teragram,
    Petagram,
    Exagram,
    Zettagram,
    Yottagram,
    Tonne,
    Carat,
    Grain,
    Pound,
    Ounce,
    Stone,
    ShortTon,
    LongTon,
    AtomicMassUnit,
    SolarMass
);

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
        // 1 M☉ ≈ 1.988416e33 grams
        assert_relative_eq!(g.value(), 1.988416e33, max_relative = 1e-5);
    }

    #[test]
    fn solar_mass_to_kilograms() {
        let sm = SolarMasses::new(1.0);
        let kg = sm.to::<Kilogram>();
        // 1 M☉ ≈ 1.988416e30 kg
        assert_relative_eq!(kg.value(), 1.988416e30, max_relative = 1e-5);
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
        // 1 M☉ = 1.988416e33 g, so RATIO should be that value
        assert_relative_eq!(SolarMass::RATIO, 1.988416e33, max_relative = 1e-5);
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
