// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Re-exports of quantity types specialized to `i128` scalar.
//!
//! This module provides type aliases for all unit types using `i128` as the
//! underlying scalar type. Integer quantities provide compile-time unit safety
//! for discrete values (counters, pixel coordinates, ADC readings, etc.).
//!
//! Integer quantities support basic arithmetic but **not** unit conversion via
//! [`to()`](crate::Quantity::to). Use [`to_lossy()`](crate::Quantity::to_lossy)
//! for lossy (truncating) conversion between units.
//!
//! # Example
//!
//! ```rust
//! use qtty::i128::{Meter, Second};
//!
//! let distance: Meter = Meter::new(1500);
//! let time: Second = Second::new(10);
//! ```

macro_rules! _alias {
    ($($unit:ident),+ $(,)?) => {
        $(pub type $unit = $crate::Quantity<$crate::unit::$unit, i128>;)+
    };
}

crate::__qtty_invoke_all_inventories!(_alias);
crate::__qtty_invoke_optional_inventories!(_alias);
