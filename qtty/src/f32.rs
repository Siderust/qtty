// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Re-exports of quantity types specialized to `f32` scalar.
//!
//! This module provides type aliases for all unit types using `f32` as the
//! underlying scalar type. Use this when memory efficiency is more important
//! than precision.
//!
//! # Example
//!
//! ```rust
//! use qtty::f32::{Degree, Meter, Second};
//!
//! let angle: Degree = Degree::new(90.0_f32);
//! let distance: Meter = Meter::new(100.0_f32);
//! let time: Second = Second::new(10.0_f32);
//! ```

macro_rules! _alias {
    ($($unit:ident),+ $(,)?) => {
        $(pub type $unit<S = f32> = $crate::Quantity<$crate::unit::$unit, S>;)+
    };
}
qtty_core::angular_units!(_alias);
qtty_core::length_units!(_alias);
qtty_core::length_nominal_units!(_alias);
qtty_core::time_units!(_alias);
qtty_core::mass_units!(_alias);
qtty_core::power_units!(_alias);
qtty_core::area_units!(_alias);
qtty_core::volume_units!(_alias);
