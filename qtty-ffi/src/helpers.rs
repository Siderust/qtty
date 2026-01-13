//! Helper functions and trait implementations for downstream crate integration.
//!
//! This module provides pre-implemented conversions between `qtty` types and [`QttyQuantity`],
//! making it easy for downstream Rust crates to integrate with the FFI layer.
//!
//! # Usage
//!
//! The conversions are implemented using the [`impl_unit_ffi!`] macro. Each supported unit
//! has `From` and `TryFrom` implementations for converting to/from [`QttyQuantity`].
//!
//! ## Converting to FFI format
//!
//! ```rust
//! use qtty::length::Meters;
//! use qtty_ffi::QttyQuantity;
//!
//! let meters = Meters::new(1000.0);
//! let ffi_qty: QttyQuantity = meters.into();
//! ```
//!
//! ## Converting from FFI format
//!
//! ```rust
//! use qtty::length::Kilometers;
//! use qtty_ffi::{QttyQuantity, UnitId};
//!
//! // A quantity received from FFI (could be in any compatible unit)
//! let ffi_qty = QttyQuantity::new(1000.0, UnitId::Meter);
//!
//! // Convert to Kilometers (automatic unit conversion happens)
//! let km: Kilometers = ffi_qty.try_into().unwrap();
//! assert!((km.value() - 1.0).abs() < 1e-12);
//! ```
//!
//! ## Error handling
//!
//! Conversion from [`QttyQuantity`] can fail if the dimensions are incompatible:
//!
//! ```rust
//! use qtty::length::Meters;
//! use qtty_ffi::{QttyQuantity, UnitId, QTTY_ERR_INCOMPATIBLE_DIM};
//!
//! let time_qty = QttyQuantity::new(60.0, UnitId::Second);
//! let result: Result<Meters, i32> = time_qty.try_into();
//! assert_eq!(result, Err(QTTY_ERR_INCOMPATIBLE_DIM));
//! ```

use crate::{impl_unit_ffi, QttyQuantity, UnitId};

// =============================================================================
// Length Unit Conversions
// =============================================================================

impl_unit_ffi!(qtty::length::Meters, UnitId::Meter);
impl_unit_ffi!(qtty::length::Kilometers, UnitId::Kilometer);

// =============================================================================
// Time Unit Conversions
// =============================================================================

impl_unit_ffi!(qtty::time::Seconds, UnitId::Second);
impl_unit_ffi!(qtty::time::Minutes, UnitId::Minute);
impl_unit_ffi!(qtty::time::Hours, UnitId::Hour);
impl_unit_ffi!(qtty::time::Days, UnitId::Day);

// =============================================================================
// Angle Unit Conversions
// =============================================================================

impl_unit_ffi!(qtty::angular::Radians, UnitId::Radian);
impl_unit_ffi!(qtty::angular::Degrees, UnitId::Degree);

// =============================================================================
// Explicit Helper Functions (Alternative API)
// =============================================================================

/// Converts `Meters` to an FFI-safe `QttyQuantity`.
#[inline]
pub fn meters_into_ffi(m: qtty::length::Meters) -> QttyQuantity {
    m.into()
}

/// Attempts to convert a `QttyQuantity` to `Meters`.
///
/// Returns an error if the quantity's unit is not length-compatible.
#[inline]
pub fn try_into_meters(q: QttyQuantity) -> Result<qtty::length::Meters, i32> {
    q.try_into()
}

/// Converts `Kilometers` to an FFI-safe `QttyQuantity`.
#[inline]
pub fn kilometers_into_ffi(km: qtty::length::Kilometers) -> QttyQuantity {
    km.into()
}

/// Attempts to convert a `QttyQuantity` to `Kilometers`.
///
/// Returns an error if the quantity's unit is not length-compatible.
#[inline]
pub fn try_into_kilometers(q: QttyQuantity) -> Result<qtty::length::Kilometers, i32> {
    q.try_into()
}

/// Converts `Seconds` to an FFI-safe `QttyQuantity`.
#[inline]
pub fn seconds_into_ffi(s: qtty::time::Seconds) -> QttyQuantity {
    s.into()
}

/// Attempts to convert a `QttyQuantity` to `Seconds`.
///
/// Returns an error if the quantity's unit is not time-compatible.
#[inline]
pub fn try_into_seconds(q: QttyQuantity) -> Result<qtty::time::Seconds, i32> {
    q.try_into()
}

/// Converts `Minutes` to an FFI-safe `QttyQuantity`.
#[inline]
pub fn minutes_into_ffi(m: qtty::time::Minutes) -> QttyQuantity {
    m.into()
}

/// Attempts to convert a `QttyQuantity` to `Minutes`.
///
/// Returns an error if the quantity's unit is not time-compatible.
#[inline]
pub fn try_into_minutes(q: QttyQuantity) -> Result<qtty::time::Minutes, i32> {
    q.try_into()
}

/// Converts `Hours` to an FFI-safe `QttyQuantity`.
#[inline]
pub fn hours_into_ffi(h: qtty::time::Hours) -> QttyQuantity {
    h.into()
}

/// Attempts to convert a `QttyQuantity` to `Hours`.
///
/// Returns an error if the quantity's unit is not time-compatible.
#[inline]
pub fn try_into_hours(q: QttyQuantity) -> Result<qtty::time::Hours, i32> {
    q.try_into()
}

/// Converts `Days` to an FFI-safe `QttyQuantity`.
#[inline]
pub fn days_into_ffi(d: qtty::time::Days) -> QttyQuantity {
    d.into()
}

/// Attempts to convert a `QttyQuantity` to `Days`.
///
/// Returns an error if the quantity's unit is not time-compatible.
#[inline]
pub fn try_into_days(q: QttyQuantity) -> Result<qtty::time::Days, i32> {
    q.try_into()
}

/// Converts `Radians` to an FFI-safe `QttyQuantity`.
#[inline]
pub fn radians_into_ffi(r: qtty::angular::Radians) -> QttyQuantity {
    r.into()
}

/// Attempts to convert a `QttyQuantity` to `Radians`.
///
/// Returns an error if the quantity's unit is not angle-compatible.
#[inline]
pub fn try_into_radians(q: QttyQuantity) -> Result<qtty::angular::Radians, i32> {
    q.try_into()
}

/// Converts `Degrees` to an FFI-safe `QttyQuantity`.
#[inline]
pub fn degrees_into_ffi(d: qtty::angular::Degrees) -> QttyQuantity {
    d.into()
}

/// Attempts to convert a `QttyQuantity` to `Degrees`.
///
/// Returns an error if the quantity's unit is not angle-compatible.
#[inline]
pub fn try_into_degrees(q: QttyQuantity) -> Result<qtty::angular::Degrees, i32> {
    q.try_into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QTTY_ERR_INCOMPATIBLE_DIM;
    use approx::assert_relative_eq;
    use core::f64::consts::PI;

    #[test]
    fn test_meters_roundtrip() {
        let original = qtty::length::Meters::new(42.5);
        let ffi: QttyQuantity = original.into();
        let back: qtty::length::Meters = ffi.try_into().unwrap();
        assert_relative_eq!(back.value(), 42.5, epsilon = 1e-12);
    }

    #[test]
    fn test_kilometers_roundtrip() {
        let original = qtty::length::Kilometers::new(1.5);
        let ffi: QttyQuantity = original.into();
        let back: qtty::length::Kilometers = ffi.try_into().unwrap();
        assert_relative_eq!(back.value(), 1.5, epsilon = 1e-12);
    }

    #[test]
    fn test_meters_to_kilometers_via_ffi() {
        let meters = qtty::length::Meters::new(1000.0);
        let ffi: QttyQuantity = meters.into();
        let km: qtty::length::Kilometers = ffi.try_into().unwrap();
        assert_relative_eq!(km.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_seconds_roundtrip() {
        let original = qtty::time::Seconds::new(3600.0);
        let ffi: QttyQuantity = original.into();
        let back: qtty::time::Seconds = ffi.try_into().unwrap();
        assert_relative_eq!(back.value(), 3600.0, epsilon = 1e-12);
    }

    #[test]
    fn test_hours_to_seconds_via_ffi() {
        let hours = qtty::time::Hours::new(1.0);
        let ffi: QttyQuantity = hours.into();
        let secs: qtty::time::Seconds = ffi.try_into().unwrap();
        assert_relative_eq!(secs.value(), 3600.0, epsilon = 1e-12);
    }

    #[test]
    fn test_degrees_to_radians_via_ffi() {
        let degrees = qtty::angular::Degrees::new(180.0);
        let ffi: QttyQuantity = degrees.into();
        let radians: qtty::angular::Radians = ffi.try_into().unwrap();
        assert_relative_eq!(radians.value(), PI, epsilon = 1e-12);
    }

    #[test]
    fn test_incompatible_conversion_fails() {
        let meters = qtty::length::Meters::new(100.0);
        let ffi: QttyQuantity = meters.into();
        let result: Result<qtty::time::Seconds, i32> = ffi.try_into();
        assert_eq!(result, Err(QTTY_ERR_INCOMPATIBLE_DIM));
    }

    #[test]
    fn test_explicit_helper_functions() {
        let m = qtty::length::Meters::new(1000.0);
        let ffi = meters_into_ffi(m);
        let km = try_into_kilometers(ffi).unwrap();
        assert_relative_eq!(km.value(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_all_helper_functions_cover_unit_variants() {
        use qtty::angular::{Degrees, Radians};
        use qtty::length::{Kilometers, Meters};
        use qtty::time::{Days, Hours, Minutes, Seconds};

        let meters_qty = meters_into_ffi(Meters::new(12.0));
        assert_eq!(meters_qty.unit, UnitId::Meter);
        assert_relative_eq!(meters_qty.value, 12.0, epsilon = 1e-12);

        let kilometers_qty = kilometers_into_ffi(Kilometers::new(3.0));
        assert_eq!(kilometers_qty.unit, UnitId::Kilometer);
        assert_relative_eq!(kilometers_qty.value, 3.0, epsilon = 1e-12);

        let meters_from_kilometers = try_into_meters(kilometers_qty).unwrap();
        assert_relative_eq!(meters_from_kilometers.value(), 3000.0, epsilon = 1e-12);

        let kilometers_from_meters = try_into_kilometers(meters_qty).unwrap();
        assert_relative_eq!(kilometers_from_meters.value(), 0.012, epsilon = 1e-12);

        let seconds_qty = seconds_into_ffi(Seconds::new(90.0));
        assert_eq!(seconds_qty.unit, UnitId::Second);
        assert_relative_eq!(seconds_qty.value, 90.0, epsilon = 1e-12);

        let minutes_qty = minutes_into_ffi(Minutes::new(2.5));
        assert_eq!(minutes_qty.unit, UnitId::Minute);
        assert_relative_eq!(minutes_qty.value, 2.5, epsilon = 1e-12);

        let seconds_from_minutes = try_into_seconds(minutes_qty).unwrap();
        assert_relative_eq!(seconds_from_minutes.value(), 150.0, epsilon = 1e-12);

        let minutes_from_seconds = try_into_minutes(seconds_qty).unwrap();
        assert_relative_eq!(minutes_from_seconds.value(), 1.5, epsilon = 1e-12);

        let hours_qty = hours_into_ffi(Hours::new(4.0));
        assert_eq!(hours_qty.unit, UnitId::Hour);
        assert_relative_eq!(hours_qty.value, 4.0, epsilon = 1e-12);

        let hours_from_minutes = try_into_hours(minutes_into_ffi(Minutes::new(180.0))).unwrap();
        assert_relative_eq!(hours_from_minutes.value(), 3.0, epsilon = 1e-12);

        let days_qty = days_into_ffi(Days::new(1.25));
        assert_eq!(days_qty.unit, UnitId::Day);
        assert_relative_eq!(days_qty.value, 1.25, epsilon = 1e-12);

        let days_from_hours = try_into_days(hours_into_ffi(Hours::new(48.0))).unwrap();
        assert_relative_eq!(days_from_hours.value(), 2.0, epsilon = 1e-12);

        let radians_qty = radians_into_ffi(Radians::new(PI));
        assert_eq!(radians_qty.unit, UnitId::Radian);
        assert_relative_eq!(radians_qty.value, PI, epsilon = 1e-12);

        let degrees_qty = degrees_into_ffi(Degrees::new(270.0));
        assert_eq!(degrees_qty.unit, UnitId::Degree);
        assert_relative_eq!(degrees_qty.value, 270.0, epsilon = 1e-12);

        let radians_from_degrees = try_into_radians(degrees_qty).unwrap();
        assert_relative_eq!(radians_from_degrees.value(), 1.5 * PI, epsilon = 1e-12);

        let degrees_from_radians = try_into_degrees(radians_qty).unwrap();
        assert_relative_eq!(degrees_from_radians.value(), 180.0, epsilon = 1e-12);
    }
}
