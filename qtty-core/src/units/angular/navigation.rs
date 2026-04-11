// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

use super::*;
use qtty_derive::Unit;

/// Gradian (also called gon; `1/400` of a full turn = `0.9` degree).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Unit)]
#[unit(symbol = "gon", dimension = Angular, ratio = 0.9)]
pub struct Gradian;
/// Type alias shorthand for [`Gradian`].
pub type Gon = Gradian;
/// Convenience alias for a gradian quantity.
pub type Gradians = Quantity<Gon>;
/// One gradian.
pub const GON: Gradians = Gradians::new(1.0);

crate::impl_unit_from_conversions_between!(
    Degree, Radian, Milliradian, Turn;
    Gradian
);

#[cfg(feature = "cross-unit-ops")]
crate::impl_unit_cross_unit_ops_between!(
    Degree, Radian, Milliradian, Turn;
    Gradian
);

// ─────────────────────────────────────────────────────────────────────────────
// Inventory macro (used by qtty-ffi build.rs)
// ─────────────────────────────────────────────────────────────────────────────
#[macro_export]
#[doc(hidden)]
macro_rules! angular_navigation_units {
    ($cb:path) => {
        $cb!(Gradian,);
    };
}
