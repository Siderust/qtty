// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Re-exports of quantity types specialized to `i64` scalar.
//!
//! This module provides type aliases for all unit types using `i64` as the
//! underlying scalar type. Integer quantities provide compile-time unit safety
//! for discrete values (timestamps, counters, high-precision integer measurements, etc.).
//!
//! Integer quantities support basic arithmetic but **not** unit conversion via
//! [`to()`](crate::Quantity::to). Use [`to_lossy()`](crate::Quantity::to_lossy)
//! for lossy (truncating) conversion between units.
//!
//! # Example
//!
//! ```rust
//! use qtty::i64::{Meter, Nanosecond};
//!
//! let distance: Meter = Meter::new(1_000_000);
//! let time: Nanosecond = Nanosecond::new(500_000_000);
//! ```

crate::scalar_aliases::define_scalar_aliases!(i64);
