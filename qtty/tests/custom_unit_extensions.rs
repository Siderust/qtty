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
