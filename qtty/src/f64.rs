//! Re-exports of quantity types specialized to `f64` scalar (default).
//!
//! This module provides type aliases for all unit types using `f64` as the
//! underlying scalar type. These are identical to the types exported at the
//! crate root, but are provided for symmetry with the [`crate::f32`] module.
//!
//! # Example
//!
//! ```rust
//! use qtty::f64::{Degrees, Meters, Seconds};
//!
//! let angle: Degrees = Degrees::new(90.0);
//! let distance: Meters = Meters::new(100.0);
//! let time: Seconds = Seconds::new(10.0);
//! ```

pub use crate::angular::*;
pub use crate::area::*;
pub use crate::length::*;
pub use crate::mass::*;
pub use crate::power::*;
pub use crate::time::*;
pub use crate::volume::*;
