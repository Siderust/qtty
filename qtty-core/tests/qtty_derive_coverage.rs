// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

#![cfg(feature = "std")]

use approx::assert_abs_diff_eq;
use qtty_core::length::Meters;
use qtty_core::Quantity;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, qtty_derive::Unit)]
#[unit(crate = qtty_core, symbol = "smoot", dimension = qtty_core::Length, ratio = 1.7018)]
struct Smoot;

type Smoots = Quantity<Smoot>;

#[test]
fn downstream_crate_path_units_compile_and_convert() {
    let smoot = Smoots::new(1.0);
    let meters: Meters = smoot.to();
    assert_abs_diff_eq!(meters.value(), 1.7018, epsilon = 1e-12);
}

#[test]
fn derive_macro_compile_fail_cases_are_covered() {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))
        .expect("qtty-core manifest dir should exist during trybuild runs");
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/fixtures/qtty_derive/missing_unit_attribute.rs");
    cases.compile_fail("tests/fixtures/qtty_derive/missing_dimension.rs");
    cases.compile_fail("tests/fixtures/qtty_derive/unknown_field.rs");
}
