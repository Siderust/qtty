//! Re-exports of quantity types specialized to `i32` scalar.
//!
//! This module provides type aliases for all unit types using `i32` as the
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
//! use qtty::i32::{Meters, Seconds};
//!
//! let distance: Meters = Meters::new(1500);
//! let time: Seconds = Seconds::new(10);
//! ```

crate::scalar_aliases::define_scalar_aliases!(i32);
