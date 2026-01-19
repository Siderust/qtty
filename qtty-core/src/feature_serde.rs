//! Serde support for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `serde` feature. It provides serialization and deserialization
//! for `Quantity<U, S>` types, including helper modules for different serialization formats.
//!
//! The actual implementations are in `quantity.rs` to support generic scalar types.
//! This module re-exports the `serde_with_unit` helper.

pub use crate::quantity::serde_with_unit;
