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

crate::scalar_aliases::define_scalar_aliases!(f64);
