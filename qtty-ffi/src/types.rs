//! ABI-stable FFI types for `qtty` quantities.
//!
//! This module defines the `#[repr(C)]` and `#[repr(u32)]` types that form the stable C ABI
//! for cross-language interoperability. These types are designed to be safe to pass across
//! FFI boundaries.
//!
//! # ABI Stability Contract
//!
//! The discriminant values for [`UnitId`] and [`DimensionId`] are part of the ABI contract
//! and **MUST NEVER CHANGE** once assigned. New variants may be added with new discriminant
//! values, but existing values must remain stable across all versions.

use core::ffi::c_char;

// =============================================================================
// Status Codes
// =============================================================================

/// Success status code.
pub const QTTY_OK: i32 = 0;

/// Error: the provided unit ID is not recognized/valid.
pub const QTTY_ERR_UNKNOWN_UNIT: i32 = -1;

/// Error: conversion requested between incompatible dimensions.
pub const QTTY_ERR_INCOMPATIBLE_DIM: i32 = -2;

/// Error: a required output pointer was null.
pub const QTTY_ERR_NULL_OUT: i32 = -3;

/// Error: the provided value is invalid (reserved for future use).
pub const QTTY_ERR_INVALID_VALUE: i32 = -4;

// =============================================================================
// Dimension Identifiers
// =============================================================================

/// Dimension identifier for FFI.
///
/// Represents the physical dimension of a quantity. All discriminant values are
/// explicitly assigned and are part of the ABI contract.
///
/// # ABI Contract
///
/// **Discriminant values must never change.** New dimensions may be added with
/// new explicit discriminant values.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DimensionId {
    /// Length dimension (e.g., meters, kilometers).
    Length = 1,
    /// Time dimension (e.g., seconds, hours).
    Time = 2,
    /// Angle dimension (e.g., radians, degrees).
    Angle = 3,
}

// =============================================================================
// Unit Identifiers
// =============================================================================

/// Unit identifier for FFI.
///
/// Each variant corresponds to a specific unit supported by the FFI layer.
/// All discriminant values are explicitly assigned and are part of the ABI contract.
///
/// # ABI Contract
///
/// **Discriminant values must never change.** New units may be added with
/// new explicit discriminant values. Units are grouped by dimension with
/// reserved ranges:
///
/// - Length units: 100-199
/// - Time units: 200-299
/// - Angle units: 300-399
///
/// This grouping is for organization only; the actual dimension is determined
/// by the registry, not by the discriminant range.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitId {
    // -------------------------------------------------------------------------
    // Length units (100-199)
    // -------------------------------------------------------------------------
    /// Meter (SI base unit for length).
    Meter = 100,
    /// Kilometer (1000 meters).
    Kilometer = 101,

    // -------------------------------------------------------------------------
    // Time units (200-299)
    // -------------------------------------------------------------------------
    /// Second (SI base unit for time).
    Second = 200,
    /// Minute (60 seconds).
    Minute = 201,
    /// Hour (3600 seconds).
    Hour = 202,
    /// Day (86400 seconds).
    Day = 203,

    // -------------------------------------------------------------------------
    // Angle units (300-399)
    // -------------------------------------------------------------------------
    /// Radian (SI unit for angles).
    Radian = 300,
    /// Degree (π/180 radians).
    Degree = 301,
}

impl UnitId {
    /// Returns the unit name as a static NUL-terminated C string.
    ///
    /// This is safe to call from C code and returns a pointer to static memory.
    #[inline]
    pub const fn name_cstr(&self) -> *const c_char {
        match self {
            UnitId::Meter => b"Meter\0".as_ptr() as *const c_char,
            UnitId::Kilometer => b"Kilometer\0".as_ptr() as *const c_char,
            UnitId::Second => b"Second\0".as_ptr() as *const c_char,
            UnitId::Minute => b"Minute\0".as_ptr() as *const c_char,
            UnitId::Hour => b"Hour\0".as_ptr() as *const c_char,
            UnitId::Day => b"Day\0".as_ptr() as *const c_char,
            UnitId::Radian => b"Radian\0".as_ptr() as *const c_char,
            UnitId::Degree => b"Degree\0".as_ptr() as *const c_char,
        }
    }

    /// Returns the unit name as a Rust string slice.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            UnitId::Meter => "Meter",
            UnitId::Kilometer => "Kilometer",
            UnitId::Second => "Second",
            UnitId::Minute => "Minute",
            UnitId::Hour => "Hour",
            UnitId::Day => "Day",
            UnitId::Radian => "Radian",
            UnitId::Degree => "Degree",
        }
    }

    /// Attempts to create a `UnitId` from a raw `u32` discriminant value.
    ///
    /// Returns `None` if the value does not correspond to a valid unit.
    #[inline]
    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            100 => Some(UnitId::Meter),
            101 => Some(UnitId::Kilometer),
            200 => Some(UnitId::Second),
            201 => Some(UnitId::Minute),
            202 => Some(UnitId::Hour),
            203 => Some(UnitId::Day),
            300 => Some(UnitId::Radian),
            301 => Some(UnitId::Degree),
            _ => None,
        }
    }
}

// =============================================================================
// Quantity Carrier Type
// =============================================================================

/// A POD quantity carrier type suitable for FFI.
///
/// This struct represents a physical quantity as a value paired with its unit.
/// It is `#[repr(C)]` to ensure a stable, predictable memory layout across
/// language boundaries.
///
/// # Memory Layout
///
/// - `value`: 8 bytes (f64)
/// - `unit`: 4 bytes (u32 via UnitId)
/// - Padding: 4 bytes (for alignment)
/// - Total: 16 bytes on most platforms
///
/// # Example
///
/// ```rust
/// use qtty_ffi::{QttyQuantity, UnitId};
///
/// let q = QttyQuantity {
///     value: 1000.0,
///     unit: UnitId::Meter,
/// };
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QttyQuantity {
    /// The numeric value of the quantity.
    pub value: f64,
    /// The unit identifier for this quantity.
    pub unit: UnitId,
}

impl QttyQuantity {
    /// Creates a new quantity with the given value and unit.
    #[inline]
    pub const fn new(value: f64, unit: UnitId) -> Self {
        Self { value, unit }
    }
}

impl Default for QttyQuantity {
    fn default() -> Self {
        Self {
            value: 0.0,
            unit: UnitId::Meter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_id_discriminants_are_stable() {
        // These values are part of the ABI contract and must never change
        assert_eq!(UnitId::Meter as u32, 100);
        assert_eq!(UnitId::Kilometer as u32, 101);
        assert_eq!(UnitId::Second as u32, 200);
        assert_eq!(UnitId::Minute as u32, 201);
        assert_eq!(UnitId::Hour as u32, 202);
        assert_eq!(UnitId::Day as u32, 203);
        assert_eq!(UnitId::Radian as u32, 300);
        assert_eq!(UnitId::Degree as u32, 301);
    }

    #[test]
    fn dimension_id_discriminants_are_stable() {
        // These values are part of the ABI contract and must never change
        assert_eq!(DimensionId::Length as u32, 1);
        assert_eq!(DimensionId::Time as u32, 2);
        assert_eq!(DimensionId::Angle as u32, 3);
    }

    #[test]
    fn unit_id_from_u32_roundtrips() {
        for unit in [
            UnitId::Meter,
            UnitId::Kilometer,
            UnitId::Second,
            UnitId::Minute,
            UnitId::Hour,
            UnitId::Day,
            UnitId::Radian,
            UnitId::Degree,
        ] {
            let value = unit as u32;
            assert_eq!(UnitId::from_u32(value), Some(unit));
        }
    }

    #[test]
    fn unit_id_from_u32_rejects_invalid() {
        assert_eq!(UnitId::from_u32(0), None);
        assert_eq!(UnitId::from_u32(99), None);
        assert_eq!(UnitId::from_u32(102), None);
        assert_eq!(UnitId::from_u32(999), None);
    }

    #[test]
    fn unit_names_are_not_empty() {
        for unit in [
            UnitId::Meter,
            UnitId::Kilometer,
            UnitId::Second,
            UnitId::Minute,
            UnitId::Hour,
            UnitId::Day,
            UnitId::Radian,
            UnitId::Degree,
        ] {
            assert!(!unit.name().is_empty());
        }
    }
}
