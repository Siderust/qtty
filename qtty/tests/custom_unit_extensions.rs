// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Downstream-style custom unit tests using only the public `qtty` facade.

use qtty::{Meter, Quantity};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, qtty::Unit)]
#[unit(crate = qtty, symbol = "smoot", dimension = qtty::Length, ratio = 1.7018)]
pub struct Smoot;

pub type Smoots = Quantity<Smoot>;

qtty::impl_unit_arithmetic_pairs_between!(qtty::unit::Meter, qtty::unit::Kilometer; Smoot);

#[test]
fn downstream_custom_unit_can_convert_to_builtins() {
    let smoot = Smoots::new(1.0);
    let meters: Meter = smoot.to();
    assert!((meters.value() - 1.7018).abs() < 1e-12);
}

#[test]
fn downstream_custom_unit_divides_with_builtins() {
    let meters = Meter::new(10.0);
    let smoots = Smoots::new(2.0);
    let ratio: Quantity<qtty::Per<qtty::unit::Meter, Smoot>> = meters / smoots;
    assert!((ratio.value() - 5.0).abs() < 1e-12);
}

#[test]
fn downstream_custom_unit_multiplies_with_builtins() {
    let smoots = Smoots::new(2.0);
    let meters = Meter::new(3.0);
    let product: Quantity<qtty::Prod<Smoot, qtty::unit::Meter>> = smoots * meters;
    assert!((product.value() - 6.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// Regression: __impl_cross_ops_one_to_many! large-magnitude overflow
//
// impl_unit_cross_unit_ops_between! cannot be invoked from an integration-test
// crate (orphan rule E0117 — Quantity<Smoot> has a foreign outer type
// constructor).  The equivalent built-in path (astro units that go through the
// same helper) is covered by `qtty-core/tests/audit_regressions.rs` under the
// `cross_unit_one_to_many_overflow` module.  The facade-level surface for those
// same impls is exercised below via the qtty re-exported astro types.
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(all(feature = "astro", feature = "cross-unit-ops"))]
mod cross_unit_one_to_many_overflow_via_facade {
    use core::cmp::Ordering;
    use qtty::unit::{LightYear, Yottameter};
    use qtty::Quantity;

    /// Distinct large-magnitude astro/SI quantities must not compare equal
    /// through the qtty facade (re-exports the fixed __impl_cross_ops_one_to_many!).
    #[test]
    fn large_magnitude_distinct_values_are_not_equal() {
        let ly = Quantity::<LightYear>::new(1e300);
        let ym = Quantity::<Yottameter>::new(2e300);
        assert_ne!(ly, ym, "1e300 ly must not equal 2e300 Ym");
        assert_ne!(ym, ly, "symmetry");
    }

    /// A correctly converted value must still compare equal.
    #[test]
    fn converted_value_is_equal() {
        let ly = Quantity::<LightYear>::new(1.0);
        let ym: Quantity<Yottameter> = ly.to();
        assert_eq!(ly, ym, "1 ly must equal its Ym equivalent");
        assert_eq!(ym, ly, "symmetry");
    }

    /// Ordering is consistent across the facade.
    #[test]
    fn partial_cmp_consistency() {
        let ly = Quantity::<LightYear>::new(1.0);
        let ym: Quantity<Yottameter> = ly.to();
        let fwd = ly.partial_cmp(&ym);
        let rev = ym.partial_cmp(&ly);
        match (fwd, rev) {
            (Some(Ordering::Less), Some(Ordering::Greater))
            | (Some(Ordering::Greater), Some(Ordering::Less))
            | (Some(Ordering::Equal), Some(Ordering::Equal)) => {}
            (None, None) => {}
            other => panic!("partial_cmp inconsistent: {other:?}"),
        }
    }
}
