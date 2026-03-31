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
//! - **C API**: `extern "C"` functions for raw quantity construction, conversion, and formatting
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
//! - [`QttyStatus::Ok`] (0): Success
//! - [`QttyStatus::UnknownUnit`] (-1): Invalid unit ID
//! - [`QttyStatus::IncompatibleDim`] (-2): Dimension mismatch
//! - [`QttyStatus::NullOut`] (-3): Null output pointer
//! - [`QttyStatus::BufferTooSmall`] (-4): Output buffer too small
//! - [`QttyStatus::InternalPanic`] (-5): Rust panic caught at the FFI boundary
//!
//! Format flags for [`qtty_quantity_format`]:
//!
//! - [`QTTY_FMT_DEFAULT`] (0): decimal notation
//! - [`QTTY_FMT_LOWER_EXP`] (1): scientific notation (lowercase `e`)
//! - [`QTTY_FMT_UPPER_EXP`] (2): scientific notation (uppercase `E`)
//!
//! # Thread Safety
//!
//! All functions are thread-safe. The library contains no global mutable state.

#![deny(missing_docs)]
// PyO3 generated code contains unsafe operations, so we can't enforce this when pyo3 feature is enabled
#![cfg_attr(not(feature = "pyo3"), deny(unsafe_op_in_unsafe_fn))]

// Core modules
mod ffi;
pub mod helpers;
#[macro_use]
pub mod macros;
pub mod registry;
mod types;

// Re-export FFI functions
pub use ffi::{
    qtty_derived_convert, qtty_derived_make, qtty_ffi_version, qtty_quantity_convert,
    qtty_quantity_convert_value, qtty_quantity_format, qtty_quantity_make, qtty_unit_dimension,
    qtty_unit_is_valid, qtty_unit_name, qtty_units_compatible,
};

// Named unit ID constants for C caller ergonomics — each is the u32 discriminant
// of the corresponding UnitId variant.  These are stable ABI values.
/// Raw unit ID constant: Meter (Length, SI).
pub const QTTY_UNIT_METER: u32 = UnitId::Meter as u32;
/// Raw unit ID constant: Kilometer (Length, SI).
pub const QTTY_UNIT_KILOMETER: u32 = UnitId::Kilometer as u32;
/// Raw unit ID constant: Second (Time, SI).
pub const QTTY_UNIT_SECOND: u32 = UnitId::Second as u32;
/// Raw unit ID constant: Minute (Time).
pub const QTTY_UNIT_MINUTE: u32 = UnitId::Minute as u32;
/// Raw unit ID constant: Hour (Time).
pub const QTTY_UNIT_HOUR: u32 = UnitId::Hour as u32;
/// Raw unit ID constant: Day (Time).
pub const QTTY_UNIT_DAY: u32 = UnitId::Day as u32;
/// Raw unit ID constant: Radian (Angle).
pub const QTTY_UNIT_RADIAN: u32 = UnitId::Radian as u32;
/// Raw unit ID constant: Degree (Angle).
pub const QTTY_UNIT_DEGREE: u32 = UnitId::Degree as u32;

// Re-export types
pub use types::{
    DimensionId, QttyDerivedQuantity, QttyQuantity, QttyStatus, UnitId, QTTY_FMT_DEFAULT,
    QTTY_FMT_LOWER_EXP, QTTY_FMT_UPPER_EXP,
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

    #[test]
    fn test_qtty_quantity_layout() {
        assert_eq!(size_of::<QttyQuantity>(), 16);
        assert_eq!(align_of::<QttyQuantity>(), 8);
    }

    #[test]
    fn test_unit_id_layout() {
        assert_eq!(size_of::<UnitId>(), 4);
        assert_eq!(align_of::<UnitId>(), 4);
    }

    #[test]
    fn test_dimension_id_layout() {
        assert_eq!(size_of::<DimensionId>(), 4);
        assert_eq!(align_of::<DimensionId>(), 4);
    }

    #[test]
    fn test_qtty_status_layout() {
        assert_eq!(size_of::<QttyStatus>(), 4);
        assert_eq!(align_of::<QttyStatus>(), 4);
    }

    #[test]
    fn test_known_conversion_meters_to_kilometers() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(1000.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert!((out.value - 1.0).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Kilometer);
    }

    #[test]
    fn test_known_conversion_seconds_to_hours() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(3600.0, UnitId::Second);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Hour as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert!((out.value - 1.0).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Hour);
    }

    #[test]
    fn test_known_conversion_degrees_to_radians() {
        use core::f64::consts::PI;
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(180.0, UnitId::Degree);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Radian as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert!((out.value - PI).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Radian);
    }

    #[test]
    fn test_incompatible_conversion_fails() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(100.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Second as u32, &mut out) };
        assert_eq!(status, QttyStatus::IncompatibleDim);
    }

    #[test]
    fn test_null_out_pointer() {
        let src = QttyQuantity::new(100.0, UnitId::Meter);
        let status =
            unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, core::ptr::null_mut()) };
        assert_eq!(status, QttyStatus::NullOut);
    }

    #[test]
    fn test_invalid_unit_id_rejected() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(100.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_convert(src, 0, &mut out) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }

    #[test]
    fn test_unit_id_constants_match_enum_discriminants() {
        assert_eq!(QTTY_UNIT_METER, UnitId::Meter as u32);
        assert_eq!(QTTY_UNIT_KILOMETER, UnitId::Kilometer as u32);
        assert_eq!(QTTY_UNIT_SECOND, UnitId::Second as u32);
        assert_eq!(QTTY_UNIT_HOUR, UnitId::Hour as u32);
        assert_eq!(QTTY_UNIT_DAY, UnitId::Day as u32);
        assert_eq!(QTTY_UNIT_RADIAN, UnitId::Radian as u32);
        assert_eq!(QTTY_UNIT_DEGREE, UnitId::Degree as u32);
    }
}
