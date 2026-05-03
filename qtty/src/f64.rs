// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Re-exports of quantity types specialized to `f64` scalar (default).
//!
//! This module provides type aliases for all built-in unit types using `f64`
//! as the underlying scalar type. These are identical to the default `f64`
//! specializations exported at the crate root, but are provided for symmetry
//! with the other scalar modules.
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
        $(pub type $unit = $crate::Quantity<$crate::unit::$unit, f64>;)+
    };
}

crate::__qtty_invoke_all_inventories!(_alias);
crate::__qtty_invoke_optional_inventories!(_alias);
