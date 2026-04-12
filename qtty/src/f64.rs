// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Re-exports of quantity types specialized to `f64` scalar (default).
//!
//! This module provides type aliases for all unit types using `f64` as the
//! underlying scalar type. These are identical to the types exported at the
//! crate root, but are provided for symmetry with the [`crate::f32`] module.
//!
//! # Example
//!
//! ```rust
//! use qtty::f64::{Degree, Meter, Second};
//!
//! let angle: Degree = Degree::new(90.0);
//! let distance: Meter = Meter::new(100.0);
//! let time: Second = Second::new(10.0);
//! ```

macro_rules! _alias {
    ($($unit:ident),+ $(,)?) => {
        $(pub type $unit<S = f64> = $crate::Quantity<$crate::unit::$unit, S>;)+
    };
}
qtty_core::angular_units!(_alias);
qtty_core::length_units!(_alias);
qtty_core::time_units!(_alias);
qtty_core::mass_units!(_alias);
qtty_core::power_units!(_alias);
qtty_core::area_units!(_alias);
qtty_core::volume_units!(_alias);
