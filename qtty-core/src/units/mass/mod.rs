// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

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
//! use qtty_core::mass::{Kilograms, Gram};
//!
//! let m = Kilograms::new(1.0);
//! let g = m.to::<Gram>();
//! assert_eq!(g.value(), 1000.0);
//! ```
//!
//! ## All mass units (default)
//!
//! ```rust
//! use qtty_core::mass::*;
//!
//! macro_rules! touch {
//!     ($T:ty, $v:expr) => {{ let q = <$T>::new($v); let _c = q; assert!(q == q); }};
//! }
//!
//! touch!(Grams, 1.0);     touch!(Tonnes, 1.0);
//! ```

use crate::{Quantity, Unit};
use qtty_derive::Unit;

/// Re-export from the dimension module.
pub use crate::dimension::Mass;

/// Marker trait for any [`Unit`] whose dimension is [`Mass`].
pub trait MassUnit: Unit<Dim = Mass> {}
impl<T: Unit<Dim = Mass>> MassUnit for T {}

#[cfg(feature = "customary")]
mod customary;
#[cfg(feature = "customary")]
pub use customary::*;
#[cfg(feature = "fundamental-physics")]
mod fundamental_physics;
#[cfg(feature = "fundamental-physics")]
pub use fundamental_physics::*;
#[cfg(feature = "astro")]
mod astro;
#[cfg(feature = "astro")]
pub use astro::*;

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

/// Canonical list of always-available (metric SI) mass units.
///
/// Exported (`#[doc(hidden)]`) for use in `qtty`\'s scalar alias generation and
/// compile-time consistency checks.  Feature-gated units (customary, astro,
/// fundamental-physics) are in their sub-modules.
#[macro_export]
#[doc(hidden)]
macro_rules! mass_units {
    ($cb:path) => {
        $cb!(
            Gram, Yoctogram, Zeptogram, Attogram, Femtogram, Picogram, Nanogram, Microgram,
            Milligram, Centigram, Decigram, Decagram, Hectogram, Kilogram, Megagram, Gigagram,
            Teragram, Petagram, Exagram, Zettagram, Yottagram, Tonne
        );
    };
}

// Generate bidirectional From impls between base metric SI mass units.
mass_units!(crate::impl_unit_from_conversions);

// ─────────────────────────────────────────────────────────────────────────────
// Cross-unit ops: default (metric) units
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "cross-unit-ops")]
mass_units!(crate::impl_unit_cross_unit_ops);

// ── Cross-feature: mass families ─────────────────────────────────────────────
#[cfg(all(feature = "astro", feature = "customary"))]
crate::__impl_from_each_extra_to_bases!(
    {SolarMass}
    Carat, Grain, Pound, Ounce, Stone, ShortTon, LongTon
);
#[cfg(all(feature = "astro", feature = "customary", feature = "cross-unit-ops"))]
crate::__impl_cross_ops_each_extra_to_bases!(
    {SolarMass}
    Carat, Grain, Pound, Ounce, Stone, ShortTon, LongTon
);

#[cfg(all(feature = "astro", feature = "fundamental-physics"))]
crate::__impl_from_each_extra_to_bases!(
    {SolarMass}
    AtomicMassUnit
);
#[cfg(all(
    feature = "astro",
    feature = "fundamental-physics",
    feature = "cross-unit-ops"
))]
crate::__impl_cross_ops_each_extra_to_bases!(
    {SolarMass}
    AtomicMassUnit
);

#[cfg(all(feature = "customary", feature = "fundamental-physics"))]
crate::__impl_from_each_extra_to_bases!(
    {Carat, Grain, Pound, Ounce, Stone, ShortTon, LongTon}
    AtomicMassUnit
);
#[cfg(all(
    feature = "customary",
    feature = "fundamental-physics",
    feature = "cross-unit-ops"
))]
crate::__impl_cross_ops_each_extra_to_bases!(
    {Carat, Grain, Pound, Ounce, Stone, ShortTon, LongTon}
    AtomicMassUnit
);

// Compile-time check: every base mass unit is registered as BuiltinUnit.
#[cfg(test)]
mass_units!(crate::assert_units_are_builtin);

#[cfg(all(test, feature = "std"))]
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
    #[cfg(feature = "astro")]
    fn solar_mass_to_grams() {
        let sm = SolarMasses::new(1.0);
        let g = sm.to::<Gram>();
        // 1 M☉ ≈ 1.988416e33 grams
        assert_relative_eq!(g.value(), 1.988416e33, max_relative = 1e-5);
    }

    #[test]
    #[cfg(feature = "astro")]
    fn solar_mass_to_kilograms() {
        let sm = SolarMasses::new(1.0);
        let kg = sm.to::<Kilogram>();
        // 1 M☉ ≈ 1.988416e30 kg
        assert_relative_eq!(kg.value(), 1.988416e30, max_relative = 1e-5);
    }

    #[test]
    #[cfg(feature = "astro")]
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
    #[cfg(feature = "astro")]
    fn solar_mass_ratio_sanity() {
        // 1 M☉ = 1.988416e33 g, so RATIO should be that value
        assert_relative_eq!(SolarMass::RATIO, 1.988416e33, max_relative = 1e-5);
    }

    #[test]
    #[cfg(feature = "astro")]
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
    #[cfg(feature = "astro")]
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

    // ─── Non-SI mass units ──────────────────────────────────────────────────

    #[test]
    fn tonne_to_kilogram() {
        let t = Tonnes::new(1.0);
        let kg = t.to::<Kilogram>();
        assert_relative_eq!(kg.value(), 1_000.0, max_relative = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn carat_to_gram() {
        let ct = Carats::new(5.0);
        let g = ct.to::<Gram>();
        // 1 ct = 0.2 g
        assert_relative_eq!(g.value(), 1.0, max_relative = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn grain_to_milligram() {
        let gr = Grains::new(1.0);
        let mg = gr.to::<Milligram>();
        // ratio in code: 6_479_891 / 100_000_000 g = 64.79891 mg
        assert_relative_eq!(mg.value(), 64.798_91, max_relative = 1e-6);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn pound_to_gram() {
        let lb = Pounds::new(1.0);
        let g = lb.to::<Gram>();
        // 1 lb = 453.59237 g
        assert_relative_eq!(g.value(), 453.592_37, max_relative = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn ounce_to_gram() {
        let oz = Ounces::new(16.0);
        let g = oz.to::<Gram>();
        // 16 oz = 1 lb = 453.59237 g
        assert_relative_eq!(g.value(), 453.592_37, max_relative = 1e-9);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn stone_to_pound() {
        let st = Stones::new(1.0);
        let lb = st.to::<Pound>();
        // 1 st = 14 lb
        assert_relative_eq!(lb.value(), 14.0, max_relative = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn short_ton_to_pound() {
        let ton = ShortTons::new(1.0);
        let lb = ton.to::<Pound>();
        // 1 US short ton = 2000 lb
        assert_relative_eq!(lb.value(), 2000.0, max_relative = 1e-12);
    }

    #[test]
    #[cfg(feature = "customary")]
    fn long_ton_to_pound() {
        let ton = LongTons::new(1.0);
        let lb = ton.to::<Pound>();
        // 1 UK long ton = 2240 lb
        assert_relative_eq!(lb.value(), 2240.0, max_relative = 1e-12);
    }

    #[test]
    #[cfg(feature = "fundamental-physics")]
    fn atomic_mass_unit_to_gram() {
        // 1 u ≈ 1.660539e-24 g
        let u = AtomicMassUnits::new(1.0);
        let g = u.to::<Gram>();
        assert_relative_eq!(g.value(), 1.660_539_068_92e-24, max_relative = 1e-6);
    }

    // ─── SI gram-prefix sampling ────────────────────────────────────────────

    #[test]
    fn milligram_to_gram() {
        let mg = Milligrams::new(1000.0);
        let g = mg.to::<Gram>();
        assert_relative_eq!(g.value(), 1.0, max_relative = 1e-12);
    }

    #[test]
    fn microgram_to_milligram() {
        let ug = Micrograms::new(1000.0);
        let mg = ug.to::<Milligram>();
        assert_relative_eq!(mg.value(), 1.0, max_relative = 1e-12);
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(Kilogram::SYMBOL, "kg");
        assert_eq!(Gram::SYMBOL, "g");
        #[cfg(feature = "customary")]
        assert_eq!(Pound::SYMBOL, "lb");
        assert_eq!(Tonne::SYMBOL, "t");
    }
}
