// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Compile-time consistency checks between `qtty-core` inventory macros and
//! the `qtty` facade re-exports.
//!
//! ## What this checks
//!
//! Two invariants are enforced at compile time:
//!
//! 1. **Unit marker re-exports** (`qtty::unit::*`): every unit in a dimension
//!    inventory is re-exported from `pub mod unit { ... }` in `qtty/src/lib.rs`.
//! 2. **Scalar quantity aliases** (`qtty::*`): every unit in a dimension
//!    inventory has a corresponding `pub type $name<S = f64>` alias generated
//!    by `define_scalar_aliases!` in `qtty/src/scalar_aliases.rs`.
//!
//! ## How it works
//!
//! Each dimension's inventory macro (`qtty_core::time_units!`, etc.) accepts a
//! callback path and expands to `$callback!(Unit1, Unit2, ...)`. The local
//! `assert_unit_reexported!` and `assert_alias_exists!` callbacks each generate
//! a `const _: () = { fn _check(_: TYPE) {} };` item per unit. If `TYPE` does
//! not exist, the file fails to compile with a clear unresolved-type error.
//!
//! ## What to do when this fails
//!
//! Add the missing unit to the appropriate `pub use qtty_core::units::dim::...`
//! block in `lib.rs`, or add the missing `pub type $name` alias to the
//! `define_scalar_aliases!` macro in `scalar_aliases.rs`.

/// Assert that every listed unit name resolves as `qtty::unit::$name`
/// (marker struct re-export).
macro_rules! assert_unit_reexported {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::unit::$name) {}
            };
        )+
    };
}

/// Assert that every listed unit name resolves as `qtty::$name`
/// (default-scalar quantity alias).
macro_rules! assert_alias_exists {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::$name) {}
            };
        )+
    };
}

// ── Unit marker re-exports (qtty::unit::*) ──────────────────────────────────

qtty_core::angular_units!(assert_unit_reexported);
qtty_core::length_units!(assert_unit_reexported);
qtty_core::length_nominal_units!(assert_unit_reexported);
qtty_core::time_units!(assert_unit_reexported);
qtty_core::mass_units!(assert_unit_reexported);
qtty_core::power_units!(assert_unit_reexported);
qtty_core::area_units!(assert_unit_reexported);
qtty_core::volume_units!(assert_unit_reexported);

// ── Scalar quantity aliases (qtty::*) ───────────────────────────────────────

qtty_core::angular_units!(assert_alias_exists);
qtty_core::length_units!(assert_alias_exists);
qtty_core::length_nominal_units!(assert_alias_exists);
qtty_core::time_units!(assert_alias_exists);
qtty_core::mass_units!(assert_alias_exists);
qtty_core::power_units!(assert_alias_exists);
qtty_core::area_units!(assert_alias_exists);
qtty_core::volume_units!(assert_alias_exists);
