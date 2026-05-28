// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Re-exports of quantity types specialized to `u32` scalar.
//!
//! This module provides type aliases for all unit types using `u32` as the
//! underlying scalar type. Unsigned integer quantities are appropriate for
//! non-negative discrete values such as week counters, subsecond nanoseconds,
//! or any quantity that is guaranteed to be non-negative.
//!
//! `u32` quantities support basic arithmetic but **not** negation (`-qty`) or
//! unit conversion via [`to()`](crate::Quantity::to). Use
//! [`to_lossy()`](crate::Quantity::to_lossy) for lossy (truncating) conversion
//! between units.
//!
//! # Example
//!
//! ```rust
//! use qtty::u32::{Nanosecond, Second};
//!
//! let sow: Second = Second::new(345_600);
//! let sub: Nanosecond = Nanosecond::new(123_456_789);
//! ```

macro_rules! _alias {
    ($($unit:ident),+ $(,)?) => {
        $(pub type $unit = $crate::Quantity<$crate::unit::$unit, u32>;)+
    };
}

crate::__qtty_invoke_all_inventories!(_alias);
crate::__qtty_invoke_optional_inventories!(_alias);
crate::dimensionless_units!(_alias);
