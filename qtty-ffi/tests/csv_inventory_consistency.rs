// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Compile-time and runtime consistency checks between `units.csv` (the FFI
//! registry source) and the canonical per-dimension inventory macros in
//! `qtty-core`.
//!
//! ## What this checks
//!
//! 1. **Forward (inventory → CSV)**: Every unit in dimension inventories that
//!    have FFI coverage has a corresponding `UnitId` variant with matching
//!    symbol and ratio.
//! 2. **Reverse (CSV → inventory)**: Every name in `units.csv` appears in at
//!    least one dimension inventory (with a known name mapping for nominal
//!    length units which use a `Nominal` prefix in the FFI layer).
//!
//! Dimensions currently covered by the FFI: Angle, Length, Mass, Power, Time.
//! Area and Volume are not yet exposed via FFI; they are excluded from the
//! forward check but would cause a compile error here once they are added to
//! `units.csv`.
//!
//! These checks ensure that `units.csv` and `qtty-core` cannot silently drift
//! apart.

use qtty::Unit;
use qtty_ffi::{registry, UnitId};
use std::collections::HashSet;

// ─────────────────────────────────────────────────────────────────────────────
// Forward check: inventory units → CSV/UnitId (compile-time)
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

// Nominal length units use a `Nominal` prefix in the FFI enum (e.g.,
// `SolarRadius` in qtty-core → `NominalSolarRadius` in UnitId).  Assert
// each mapping compiles.
const _: UnitId = UnitId::NominalSolarRadius;
const _: UnitId = UnitId::NominalSolarDiameter;
const _: UnitId = UnitId::NominalEarthRadius;
const _: UnitId = UnitId::NominalEarthEquatorialRadius;
const _: UnitId = UnitId::NominalEarthPolarRadius;
const _: UnitId = UnitId::NominalJupiterRadius;
const _: UnitId = UnitId::NominalLunarRadius;
const _: UnitId = UnitId::NominalLunarDistance;

// ─────────────────────────────────────────────────────────────────────────────
// Forward check: symbol and ratio match (runtime)
// ─────────────────────────────────────────────────────────────────────────────

// Known divergences between qtty-core and qtty-ffi that should be reconciled
// separately.  Tracked here so the consistency test remains useful while these
// are outstanding.
//
// Symbol divergences: cosmetic formatting differences (subscripts, unicode,
// abbreviation style).
//
// Ratio divergences:
//   Year/Decade/Century/Millennium: different year definition
//     (Julian year 365.25d in core vs a slightly different value in FFI).
//   SynodicMonth: slightly different value (~7e-8 relative).
//   Grain: factor-of-10 difference (likely a bug in one source).

/// Units whose **symbol** differs between core and FFI.
const KNOWN_SYMBOL_DIVERGENCES: &[&str] = &[];

/// Units whose **ratio** differs between core and FFI (excluded from
/// cross-ratio checks).
const KNOWN_RATIO_DIVERGENCES: &[&str] = &[];

/// Normalize Unicode micro sign (U+00B5) to Greek small letter mu (U+03BC)
/// so that symbol comparisons are not tripped by this common discrepancy.
fn normalize_mu(s: &str) -> String {
    s.replace('\u{00B5}', "\u{03BC}")
}

// Collect symbol and ratio divergences into thread-local storage.
// Uses thread_local to work around cross-crate macro hygiene.
std::thread_local! {
    static MISMATCHES: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn push_mismatch(msg: String) {
    MISMATCHES.with(|m| m.borrow_mut().push(msg));
}

fn take_mismatches() -> Vec<String> {
    MISMATCHES.with(|m| std::mem::take(&mut *m.borrow_mut()))
}

/// Collect symbol mismatches for a dimension.
macro_rules! check_symbols_lax {
    ($($name:ident),+ $(,)?) => {
        $(
            {
                let id = UnitId::$name;
                let core_sym = <qtty::unit::$name as Unit>::SYMBOL;
                let ffi_sym = id.symbol();
                let name_str = stringify!($name);
                let matches = normalize_mu(core_sym) == normalize_mu(ffi_sym);
                if KNOWN_SYMBOL_DIVERGENCES.contains(&name_str) {
                    if matches {
                        push_mismatch(format!(
                            "FIXED symbol: {} — remove from KNOWN_SYMBOL_DIVERGENCES",
                            name_str,
                        ));
                    }
                } else if !matches {
                    push_mismatch(format!(
                        "NEW symbol divergence: {} core='{}' ffi='{}'",
                        name_str, core_sym, ffi_sym,
                    ));
                }
            }
        )+
    };
}

/// Collect `(name, core_ratio, ffi_ratio)` tuples and check cross-ratios,
/// reporting mismatches instead of panicking.
macro_rules! check_ratios_lax {
    ($($name:ident),+ $(,)?) => {
        {
            let entries: Vec<(&str, f64, f64)> = vec![
                $( (
                    stringify!($name),
                    <qtty::unit::$name as Unit>::RATIO,
                    registry::meta(UnitId::$name)
                        .unwrap_or_else(|| panic!("No registry metadata for {:?}", UnitId::$name))
                        .scale_to_canonical,
                ) ),+
            ];
            // Filter out known ratio divergences.
            let filtered: Vec<_> = entries
                .into_iter()
                .filter(|(name, _, _)| !KNOWN_RATIO_DIVERGENCES.contains(name))
                .collect();
            report_cross_ratio_mismatches(&filtered);
        }
    };
}

/// Report cross-ratio mismatches to thread-local storage.
fn report_cross_ratio_mismatches(entries: &[(&str, f64, f64)]) {
    if entries.len() <= 1 {
        return;
    }
    let (ref_name, ref_core, ref_ffi) = entries[0];
    for &(name, core, ffi) in &entries[1..] {
        let core_rel = core / ref_core;
        let ffi_rel = ffi / ref_ffi;
        let rel_err = ((core_rel - ffi_rel) / core_rel).abs();
        if rel_err >= 1e-8 {
            push_mismatch(format!(
                "NEW ratio divergence: {} vs {} core_rel={} ffi_rel={} rel_err={}",
                name, ref_name, core_rel, ffi_rel, rel_err,
            ));
        }
    }
}

/// Verify that no *new* symbol or ratio divergences have appeared, and that
/// all known divergences listed above are still present (prompting cleanup
/// when fixed).
#[test]
fn inventory_consistent_with_ffi() {
    // Symbol checks.
    qtty_core::angular_units!(check_symbols_lax);
    qtty_core::length_units!(check_symbols_lax);
    qtty_core::time_units!(check_symbols_lax);
    qtty_core::mass_units!(check_symbols_lax);
    qtty_core::power_units!(check_symbols_lax);

    // Ratio checks.
    qtty_core::angular_units!(check_ratios_lax);
    qtty_core::length_units!(check_ratios_lax);
    qtty_core::time_units!(check_ratios_lax);
    qtty_core::mass_units!(check_ratios_lax);
    qtty_core::power_units!(check_ratios_lax);

    let mismatches = take_mismatches();
    if !mismatches.is_empty() {
        eprintln!("=== CONSISTENCY FAILURES ===");
        for m in &mismatches {
            eprintln!("  {m}");
        }
        panic!(
            "{} consistency failures — new divergences or fixed entries \
             that need list cleanup (see stderr)",
            mismatches.len(),
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reverse check: CSV names → inventory names
// ─────────────────────────────────────────────────────────────────────────────

/// Callback that, when invoked by an inventory macro, produces a const array
/// of unit name strings.  Each invocation defines a new `const` item.
macro_rules! def_angular_names   { ($($n:ident),+ $(,)?) => { const ANGULAR_NAMES:   &[&str] = &[$(stringify!($n)),+]; }; }
macro_rules! def_length_names    { ($($n:ident),+ $(,)?) => { const LENGTH_NAMES:    &[&str] = &[$(stringify!($n)),+]; }; }
macro_rules! def_length_nom_names{ ($($n:ident),+ $(,)?) => { const LENGTH_NOM_NAMES:&[&str] = &[$(stringify!($n)),+]; }; }
macro_rules! def_time_names      { ($($n:ident),+ $(,)?) => { const TIME_NAMES:      &[&str] = &[$(stringify!($n)),+]; }; }
macro_rules! def_mass_names      { ($($n:ident),+ $(,)?) => { const MASS_NAMES:      &[&str] = &[$(stringify!($n)),+]; }; }
macro_rules! def_power_names     { ($($n:ident),+ $(,)?) => { const POWER_NAMES:     &[&str] = &[$(stringify!($n)),+]; }; }
macro_rules! def_area_names      { ($($n:ident),+ $(,)?) => { const AREA_NAMES:      &[&str] = &[$(stringify!($n)),+]; }; }
macro_rules! def_volume_names    { ($($n:ident),+ $(,)?) => { const VOLUME_NAMES:    &[&str] = &[$(stringify!($n)),+]; }; }

qtty_core::angular_units!(def_angular_names);
qtty_core::length_units!(def_length_names);
qtty_core::length_nominal_units!(def_length_nom_names);
qtty_core::time_units!(def_time_names);
qtty_core::mass_units!(def_mass_names);
qtty_core::power_units!(def_power_names);
qtty_core::area_units!(def_area_names);
qtty_core::volume_units!(def_volume_names);

/// Known name mapping: FFI name → inventory name (for nominal units).
const NOMINAL_FFI_TO_CORE: &[(&str, &str)] = &[
    ("NominalSolarRadius", "SolarRadius"),
    ("NominalSolarDiameter", "SolarDiameter"),
    ("NominalEarthRadius", "EarthRadius"),
    ("NominalEarthEquatorialRadius", "EarthEquatorialRadius"),
    ("NominalEarthPolarRadius", "EarthPolarRadius"),
    ("NominalJupiterRadius", "JupiterRadius"),
    ("NominalLunarRadius", "LunarRadius"),
    ("NominalLunarDistance", "LunarDistance"),
];

#[test]
fn csv_names_are_in_inventories() {
    let mut inventory_names: HashSet<&str> = HashSet::new();

    // Unitless is not in any dimension inventory; add it explicitly.
    inventory_names.insert("Unitless");

    // Collect from every dimension inventory.
    for slice in [
        ANGULAR_NAMES,
        LENGTH_NAMES,
        LENGTH_NOM_NAMES,
        TIME_NAMES,
        MASS_NAMES,
        POWER_NAMES,
        AREA_NAMES,
        VOLUME_NAMES,
    ] {
        for name in slice {
            inventory_names.insert(name);
        }
    }

    // Build set of known nominal FFI names.
    let nominal_ffi_names: HashSet<&str> =
        NOMINAL_FFI_TO_CORE.iter().map(|(ffi, _)| *ffi).collect();

    // Parse units.csv and check each name is in an inventory.
    let csv = include_str!("../units.csv");
    for line in csv.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.splitn(6, ',').collect();
        assert!(fields.len() >= 4, "Malformed CSV line: {line}");
        let name = fields[2];

        if nominal_ffi_names.contains(name) {
            // Nominal unit — verify the mapped core name exists in inventory.
            let core_name = NOMINAL_FFI_TO_CORE
                .iter()
                .find(|(ffi, _)| *ffi == name)
                .map(|(_, core)| *core)
                .unwrap();
            assert!(
                inventory_names.contains(core_name),
                "Nominal mapping for CSV unit '{}' → '{}' is not in inventories",
                name,
                core_name,
            );
        } else {
            assert!(
                inventory_names.contains(name),
                "CSV unit '{}' is not in any qtty-core dimension inventory \
                 and has no nominal mapping",
                name,
            );
        }
    }
}
