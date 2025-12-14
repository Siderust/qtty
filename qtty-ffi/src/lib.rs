//! C-compatible FFI bindings for `qtty` physical quantities and unit conversions.
//!
//! `qtty-ffi` provides a stable C ABI for `qtty`, enabling interoperability with C/C++ code
//! and other languages with C FFI support. It also provides helper types and macros for
//! downstream Rust crates that need to expose their own FFI APIs using `qtty` quantities.
//!
//! # Features
//!
//! - **ABI-stable types**: `#[repr(C)]` and `#[repr(u32)]` types safe for FFI
//! - **Unit registry**: Mapping between FFI unit IDs and conversion factors
//! - **C API**: `extern "C"` functions for quantity construction and conversion
//! - **Rust helpers**: Macros and trait implementations for downstream integration
//!
//! # Quick Start (C/C++)
//!
//! Include the generated header and link against the library:
//!
//! ```c
//! #include "qtty_ffi.h"
//!
//! // Create a quantity
//! QttyQuantity meters;
//! qtty_quantity_make(1000.0, UnitId_Meter, &meters);
//!
//! // Convert to kilometers
//! QttyQuantity kilometers;
//! int32_t status = qtty_quantity_convert(meters, UnitId_Kilometer, &kilometers);
//! if (status == QTTY_OK) {
//!     // kilometers.value == 1.0
//! }
//! ```
//!
//! # Quick Start (Rust)
//!
//! Use the helper traits and macros for seamless conversion:
//!
//! ```rust
//! use qtty::length::{Meters, Kilometers};
//! use qtty_ffi::{QttyQuantity, UnitId};
//!
//! // Convert Rust type to FFI
//! let meters = Meters::new(1000.0);
//! let ffi_qty: QttyQuantity = meters.into();
//!
//! // Convert FFI back to Rust type (with automatic unit conversion)
//! let km: Kilometers = ffi_qty.try_into().unwrap();
//! assert!((km.value() - 1.0).abs() < 1e-12);
//! ```
//!
//! # ABI Stability
//!
//! The following are part of the ABI contract and will never change:
//!
//! - [`UnitId`] discriminant values (existing variants)
//! - [`DimensionId`] discriminant values (existing variants)
//! - [`QttyQuantity`] memory layout
//! - Status code values ([`QTTY_OK`], [`QTTY_ERR_UNKNOWN_UNIT`], etc.)
//! - Function signatures of exported `extern "C"` functions
//!
//! New variants may be added to enums (with new discriminant values), and new functions
//! may be added, but existing items will remain stable.
//!
//! # Supported Units (v1)
//!
//! ## Length
//! - [`UnitId::Meter`] - SI base unit
//! - [`UnitId::Kilometer`] - 1000 meters
//!
//! ## Time
//! - [`UnitId::Second`] - SI base unit
//! - [`UnitId::Minute`] - 60 seconds
//! - [`UnitId::Hour`] - 3600 seconds
//! - [`UnitId::Day`] - 86400 seconds
//!
//! ## Angle
//! - [`UnitId::Radian`] - SI unit
//! - [`UnitId::Degree`] - π/180 radians
//!
//! # Error Handling
//!
//! All FFI functions return status codes:
//!
//! - [`QTTY_OK`] (0): Success
//! - [`QTTY_ERR_UNKNOWN_UNIT`] (-1): Invalid unit ID
//! - [`QTTY_ERR_INCOMPATIBLE_DIM`] (-2): Dimension mismatch
//! - [`QTTY_ERR_NULL_OUT`] (-3): Null output pointer
//! - [`QTTY_ERR_INVALID_VALUE`] (-4): Invalid value (reserved)
//!
//! # Thread Safety
//!
//! All functions are thread-safe. The library contains no global mutable state.

#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

// Core modules
mod ffi;
pub mod helpers;
#[macro_use]
pub mod macros;
pub mod registry;
mod types;

// Re-export FFI functions
pub use ffi::{
    qtty_ffi_version, qtty_quantity_convert, qtty_quantity_convert_value, qtty_quantity_make,
    qtty_unit_dimension, qtty_unit_is_valid, qtty_unit_name, qtty_units_compatible,
};

// Re-export types
pub use types::{
    DimensionId, QttyQuantity, UnitId, QTTY_ERR_INCOMPATIBLE_DIM, QTTY_ERR_INVALID_VALUE,
    QTTY_ERR_NULL_OUT, QTTY_ERR_UNKNOWN_UNIT, QTTY_OK,
};

// The impl_unit_ffi! macro is automatically exported at crate root by #[macro_export]

// Re-export helper functions
pub use helpers::{
    days_into_ffi, degrees_into_ffi, hours_into_ffi, kilometers_into_ffi, meters_into_ffi,
    minutes_into_ffi, radians_into_ffi, seconds_into_ffi, try_into_days, try_into_degrees,
    try_into_hours, try_into_kilometers, try_into_meters, try_into_minutes, try_into_radians,
    try_into_seconds,
};

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    /// Test that QttyQuantity has the expected size and alignment for FFI.
    #[test]
    fn test_qtty_quantity_layout() {
        // QttyQuantity should be:
        // - f64 (8 bytes) + UnitId (4 bytes) + padding (4 bytes) = 16 bytes
        // - Aligned to 8 bytes (alignment of f64)
        assert_eq!(size_of::<QttyQuantity>(), 16);
        assert_eq!(align_of::<QttyQuantity>(), 8);
    }

    /// Test that UnitId has the expected size.
    #[test]
    fn test_unit_id_layout() {
        assert_eq!(size_of::<UnitId>(), 4);
        assert_eq!(align_of::<UnitId>(), 4);
    }

    /// Test that DimensionId has the expected size.
    #[test]
    fn test_dimension_id_layout() {
        assert_eq!(size_of::<DimensionId>(), 4);
        assert_eq!(align_of::<DimensionId>(), 4);
    }

    /// Test known conversion: 1000 meters → 1 kilometer
    #[test]
    fn test_known_conversion_meters_to_kilometers() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(1000.0, UnitId::Meter);

        let status = qtty_quantity_convert(src, UnitId::Kilometer, &mut out);

        assert_eq!(status, QTTY_OK);
        assert!((out.value - 1.0).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Kilometer);
    }

    /// Test known conversion: 3600 seconds → 1 hour
    #[test]
    fn test_known_conversion_seconds_to_hours() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(3600.0, UnitId::Second);

        let status = qtty_quantity_convert(src, UnitId::Hour, &mut out);

        assert_eq!(status, QTTY_OK);
        assert!((out.value - 1.0).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Hour);
    }

    /// Test known conversion: 180 degrees → π radians
    #[test]
    fn test_known_conversion_degrees_to_radians() {
        use core::f64::consts::PI;

        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(180.0, UnitId::Degree);

        let status = qtty_quantity_convert(src, UnitId::Radian, &mut out);

        assert_eq!(status, QTTY_OK);
        assert!((out.value - PI).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Radian);
    }

    /// Test incompatible conversion: meters → seconds fails
    #[test]
    fn test_incompatible_conversion_fails() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(100.0, UnitId::Meter);

        let status = qtty_quantity_convert(src, UnitId::Second, &mut out);

        assert_eq!(status, QTTY_ERR_INCOMPATIBLE_DIM);
    }

    /// Test null output pointer handling
    #[test]
    fn test_null_out_pointer() {
        let src = QttyQuantity::new(100.0, UnitId::Meter);

        let status = qtty_quantity_convert(src, UnitId::Kilometer, core::ptr::null_mut());

        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }
}
