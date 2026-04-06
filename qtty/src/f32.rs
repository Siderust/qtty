//! Re-exports of quantity types specialized to `f32` scalar.
//!
//! This module provides type aliases for all unit types using `f32` as the
//! underlying scalar type. Use this when memory efficiency is more important
//! than precision.
//!
//! # Example
//!
//! ```rust
//! use qtty::f32::{Degrees, Meters, Seconds};
//!
//! let angle: Degrees = Degrees::new(90.0_f32);
//! let distance: Meters = Meters::new(100.0_f32);
//! let time: Seconds = Seconds::new(10.0_f32);
//! ```

crate::scalar_aliases::define_scalar_aliases!(f32);
