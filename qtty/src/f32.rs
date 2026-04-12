// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Re-exports of quantity types specialized to `f32` scalar.
//!
//! This module provides type aliases for all built-in unit types using `f32`
//! as the underlying scalar type. Use this when memory efficiency is more
//! important than precision.
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
        $(pub type $unit = $crate::Quantity<$crate::unit::$unit, f32>;)+
    };
}

crate::__qtty_invoke_all_inventories!(_alias);
crate::__qtty_invoke_optional_inventories!(_alias);
