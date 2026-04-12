// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Compile-time consistency checks between `qtty-core` inventory macros and
//! the `qtty` facade re-exports.
//!
//! ## What this checks
//!
//! Several invariants are enforced at compile time:
//!
//! 1. **Unit marker re-exports** (`qtty::unit::*`): every unit in a dimension
//!    inventory is re-exported from `pub mod unit { ... }` in `qtty/src/lib.rs`.
//! 2. **Scalar quantity aliases** (`qtty::*`): every unit in a dimension
//!    inventory has a corresponding `pub type $name<S = f64>` alias generated
//!    at the crate root in `qtty/src/lib.rs`.
//! 3. **Scalar modules** (`qtty::f32::*`, `qtty::i32::*`, etc.): every unit in
//!    the public inventory is available from each scalar-specific facade.
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
//! block in `lib.rs`, or fix the alias-generation macros in `qtty/src/lib.rs`.

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

macro_rules! assert_f32_alias_exists {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::f32::$name) {}
            };
        )+
    };
}

macro_rules! assert_f64_alias_exists {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::f64::$name) {}
            };
        )+
    };
}

macro_rules! assert_i8_alias_exists {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::i8::$name) {}
            };
        )+
    };
}

macro_rules! assert_i16_alias_exists {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::i16::$name) {}
            };
        )+
    };
}

macro_rules! assert_i32_alias_exists {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::i32::$name) {}
            };
        )+
    };
}

macro_rules! assert_i64_alias_exists {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::i64::$name) {}
            };
        )+
    };
}

macro_rules! assert_i128_alias_exists {
    ($($name:ident),+ $(,)?) => {
        $(
            const _: () = {
                fn _check(_: qtty::i128::$name) {}
            };
        )+
    };
}

// ── Unit marker re-exports (qtty::unit::*) ──────────────────────────────────

qtty::__qtty_invoke_all_inventories!(assert_unit_reexported);
qtty::__qtty_invoke_optional_inventories!(assert_unit_reexported);

// ── Scalar quantity aliases (qtty::*) ───────────────────────────────────────

qtty::__qtty_invoke_all_inventories!(assert_alias_exists);
qtty::__qtty_invoke_optional_inventories!(assert_alias_exists);

// ── Scalar module aliases (qtty::f32::*, qtty::i8::*, …) ───────────────────

qtty::__qtty_invoke_all_inventories!(assert_f32_alias_exists);
qtty::__qtty_invoke_optional_inventories!(assert_f32_alias_exists);
qtty::__qtty_invoke_all_inventories!(assert_f64_alias_exists);
qtty::__qtty_invoke_optional_inventories!(assert_f64_alias_exists);
qtty::__qtty_invoke_all_inventories!(assert_i8_alias_exists);
qtty::__qtty_invoke_optional_inventories!(assert_i8_alias_exists);
qtty::__qtty_invoke_all_inventories!(assert_i16_alias_exists);
qtty::__qtty_invoke_optional_inventories!(assert_i16_alias_exists);
qtty::__qtty_invoke_all_inventories!(assert_i32_alias_exists);
qtty::__qtty_invoke_optional_inventories!(assert_i32_alias_exists);
qtty::__qtty_invoke_all_inventories!(assert_i64_alias_exists);
qtty::__qtty_invoke_optional_inventories!(assert_i64_alias_exists);
qtty::__qtty_invoke_all_inventories!(assert_i128_alias_exists);
qtty::__qtty_invoke_optional_inventories!(assert_i128_alias_exists);
