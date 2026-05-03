// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Compile-time and runtime consistency checks between `discriminants.csv`
//! (the FFI discriminant registry) and the canonical per-dimension inventory
//! macros in `qtty-core`.
//!
//! ## What this checks
//!
//! 1. **Forward (inventory → FFI)**: Every unit in dimension inventories that
//!    have FFI coverage has a corresponding `UnitId` variant.  This is a
//!    compile-time check — if `UnitId::$name` doesn't exist, compilation fails.
//!
//! 2. **Ratio & symbol consistency**: Since build.rs now derives all metadata
//!    from `<Type as qtty::Unit>::RATIO` and `::SYMBOL`, ratio/symbol drift
//!    is impossible by construction.  A lightweight runtime smoke-test confirms
//!    the trait-derived values match what qtty-core reports.
//!
//! 3. **Reverse (discriminants → Rust type)**: `build.rs` generates references
//!    to `qtty::unit::*` marker types for every discriminant. Missing or stale
//!    entries fail compilation during the crate build, so this file does not
//!    need to duplicate that check.
//!
//! Dimensions currently covered: Angle, Length, Mass, Power, Time, Area, Volume.

use qtty::Unit;
use qtty_ffi::{registry, UnitId};

// ─────────────────────────────────────────────────────────────────────────────
// Forward check: inventory units → UnitId variants (compile-time)
// ─────────────────────────────────────────────────────────────────────────────

/// Compile-time assertion: every inventory unit name is a valid `UnitId`
/// variant.  If `UnitId::$name` doesn't exist, compilation fails.
macro_rules! assert_unit_has_ffi_id {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: UnitId = UnitId::$name;
        )+
    };
}

// Dimensions whose inventory names match their FFI variant names 1:1.
qtty_core::angular_units!(assert_unit_has_ffi_id);
qtty_core::length_units!(assert_unit_has_ffi_id);
qtty_core::time_units!(assert_unit_has_ffi_id);
qtty_core::mass_units!(assert_unit_has_ffi_id);
qtty_core::power_units!(assert_unit_has_ffi_id);
qtty_core::area_units!(assert_unit_has_ffi_id);
qtty_core::volume_units!(assert_unit_has_ffi_id);

// Nominal length units use a `Nominal` prefix in the FFI enum.
const _: UnitId = UnitId::NominalSolarRadius;
const _: UnitId = UnitId::NominalSolarDiameter;
const _: UnitId = UnitId::NominalEarthRadius;
const _: UnitId = UnitId::NominalEarthEquatorialRadius;
const _: UnitId = UnitId::NominalEarthPolarRadius;
const _: UnitId = UnitId::NominalJupiterRadius;
const _: UnitId = UnitId::NominalLunarRadius;
const _: UnitId = UnitId::NominalLunarDistance;

// ─────────────────────────────────────────────────────────────────────────────
// Smoke-test: trait-derived metadata matches qtty-core (runtime)
// ─────────────────────────────────────────────────────────────────────────────

/// Verify that registry metadata matches the `Unit` trait for a set of units.
///
/// This is a belt-and-suspenders check: since build.rs generates match arms
/// that reference `<T as Unit>::RATIO` directly, the only way this could fail
/// is if a wrong type path were mapped to a UnitId variant (which would also
/// be a compile error in most cases).
macro_rules! check_trait_metadata {
    ($($name:ident),+ $(,)?) => {
        $(
            {
                let id = UnitId::$name;
                let meta = registry::meta(id)
                    .unwrap_or_else(|| panic!("No registry metadata for {:?}", id));

                // Symbol must match the Unit trait
                let trait_symbol = <qtty::unit::$name as Unit>::SYMBOL;
                assert_eq!(
                    id.symbol(), trait_symbol,
                    "Symbol mismatch for {:?}: ffi='{}' trait='{}'",
                    id, id.symbol(), trait_symbol,
                );

                // Ratio must match the Unit trait
                let trait_ratio = <qtty::unit::$name as Unit>::RATIO;
                assert!(
                    (meta.scale_to_canonical - trait_ratio).abs() < 1e-30_f64.max(trait_ratio.abs() * 1e-15),
                    "Ratio mismatch for {:?}: ffi={} trait={}",
                    id, meta.scale_to_canonical, trait_ratio,
                );
            }
        )+
    };
}

#[test]
fn trait_metadata_consistent_with_registry() {
    qtty_core::angular_units!(check_trait_metadata);
    qtty_core::length_units!(check_trait_metadata);
    qtty_core::time_units!(check_trait_metadata);
    qtty_core::mass_units!(check_trait_metadata);
    qtty_core::power_units!(check_trait_metadata);
    qtty_core::area_units!(check_trait_metadata);
    qtty_core::volume_units!(check_trait_metadata);
}
