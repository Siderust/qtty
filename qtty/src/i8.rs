// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Re-exports of quantity types specialized to `i8` scalar.
//!
//! This module provides type aliases for all unit types using `i8` as the
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
//! use qtty::i8::{Meter, Second};
//!
//! let distance: Meter = Meter::new(120);
//! let time: Second = Second::new(10);
//! ```

macro_rules! _alias {
    ($($unit:ident),+ $(,)?) => {
        $(pub type $unit<S = i8> = $crate::Quantity<$crate::unit::$unit, S>;)+
    };
}
qtty_core::angular_units!(_alias);
qtty_core::length_units!(_alias);
qtty_core::time_units!(_alias);
qtty_core::mass_units!(_alias);
qtty_core::power_units!(_alias);
qtty_core::area_units!(_alias);
qtty_core::volume_units!(_alias);
