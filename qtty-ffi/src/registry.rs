//! Unit registry and conversion logic for FFI.
//!
//! This module provides a Rust-only unit registry that maps [`UnitId`] values to their
//! metadata (dimension, scaling factor, name) and implements conversion between compatible units.
//!
//! # Conversion Formula
//!
//! Conversions use a canonical unit per dimension:
//! - Length: Meter
//! - Time: Second
//! - Angle: Radian
//!
//! The conversion formula is:
//! ```text
//! v_canonical = v_src * src.scale_to_canonical
//! v_dst = v_canonical / dst.scale_to_canonical
//! ```
//!
//! Which simplifies to:
//! ```text
//! v_dst = v_src * (src.scale_to_canonical / dst.scale_to_canonical)
//! ```

use crate::types::{
    DimensionId, UnitId, QTTY_ERR_INCOMPATIBLE_DIM, QTTY_ERR_UNKNOWN_UNIT, QTTY_OK,
};

// =============================================================================
// Unit Metadata
// =============================================================================

/// Metadata about a unit for internal registry use.
///
/// This struct is Rust-only and not exposed via FFI.
#[derive(Debug, Clone, Copy)]
pub struct UnitMeta {
    /// The dimension this unit belongs to.
    pub dim: DimensionId,
    /// Scaling factor to convert to the canonical unit for this dimension.
    ///
    /// For example, for Kilometer: `scale_to_canonical = 1000.0` (1 km = 1000 m)
    pub scale_to_canonical: f64,
    /// Human-readable name of the unit.
    pub name: &'static str,
}

// =============================================================================
// Registry Functions
// =============================================================================

/// Returns metadata for the given unit ID.
///
/// Returns `None` if the unit ID is not recognized.
#[inline]
pub fn meta(id: UnitId) -> Option<UnitMeta> {
    include!(concat!(env!("OUT_DIR"), "/unit_registry.rs"))
}

/// Returns the dimension for the given unit ID.
///
/// Returns `None` if the unit ID is not recognized.
#[inline]
pub fn dimension(id: UnitId) -> Option<DimensionId> {
    meta(id).map(|m| m.dim)
}

/// Checks if two units are compatible (same dimension).
///
/// Returns `true` if both units have the same dimension, `false` otherwise.
/// Also returns `false` if either unit is not recognized.
#[inline]
pub fn compatible(a: UnitId, b: UnitId) -> bool {
    match (dimension(a), dimension(b)) {
        (Some(da), Some(db)) => da == db,
        _ => false,
    }
}

/// Converts a value from one unit to another.
///
/// # Arguments
///
/// * `v` - The value to convert
/// * `src` - The source unit
/// * `dst` - The destination unit
///
/// # Returns
///
/// * `Ok(converted_value)` on success
/// * `Err(QTTY_ERR_UNKNOWN_UNIT)` if either unit is not recognized
/// * `Err(QTTY_ERR_INCOMPATIBLE_DIM)` if units have different dimensions
///
/// # Example
///
/// ```rust
/// use qtty_ffi::{registry, UnitId};
///
/// let meters = registry::convert_value(1000.0, UnitId::Meter, UnitId::Kilometer);
/// assert!((meters.unwrap() - 1.0).abs() < 1e-12);
/// ```
#[inline]
pub fn convert_value(v: f64, src: UnitId, dst: UnitId) -> Result<f64, i32> {
    let src_meta = meta(src).ok_or(QTTY_ERR_UNKNOWN_UNIT)?;
    let dst_meta = meta(dst).ok_or(QTTY_ERR_UNKNOWN_UNIT)?;

    if src_meta.dim != dst_meta.dim {
        return Err(QTTY_ERR_INCOMPATIBLE_DIM);
    }

    // If same unit, no conversion needed
    if src == dst {
        return Ok(v);
    }

    // Convert: v_canonical = v * src_scale, then v_dst = v_canonical / dst_scale
    let v_canonical = v * src_meta.scale_to_canonical;
    let v_dst = v_canonical / dst_meta.scale_to_canonical;

    Ok(v_dst)
}

/// Converts a value from one unit to another, returning a status code.
///
/// This is a convenience function that returns `QTTY_OK` on success and
/// the appropriate error code on failure. The converted value is stored
/// in `result` (which must be initialized).
///
/// # Arguments
///
/// * `v` - The value to convert
/// * `src` - The source unit
/// * `dst` - The destination unit
/// * `result` - Mutable reference to store the converted value
///
/// # Returns
///
/// * `QTTY_OK` on success
/// * `QTTY_ERR_UNKNOWN_UNIT` if either unit is not recognized
/// * `QTTY_ERR_INCOMPATIBLE_DIM` if units have different dimensions
#[inline]
pub fn convert_value_status(v: f64, src: UnitId, dst: UnitId, result: &mut f64) -> i32 {
    match convert_value(v, src, dst) {
        Ok(converted) => {
            *result = converted;
            QTTY_OK
        }
        Err(code) => code,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use core::f64::consts::PI;

    #[test]
    fn test_meta_returns_correct_dimensions() {
        assert_eq!(meta(UnitId::Meter).unwrap().dim, DimensionId::Length);
        assert_eq!(meta(UnitId::Kilometer).unwrap().dim, DimensionId::Length);
        assert_eq!(meta(UnitId::Second).unwrap().dim, DimensionId::Time);
        assert_eq!(meta(UnitId::Minute).unwrap().dim, DimensionId::Time);
        assert_eq!(meta(UnitId::Hour).unwrap().dim, DimensionId::Time);
        assert_eq!(meta(UnitId::Day).unwrap().dim, DimensionId::Time);
        assert_eq!(meta(UnitId::Radian).unwrap().dim, DimensionId::Angle);
        assert_eq!(meta(UnitId::Degree).unwrap().dim, DimensionId::Angle);
    }

    #[test]
    fn test_compatible_same_dimension() {
        assert!(compatible(UnitId::Meter, UnitId::Kilometer));
        assert!(compatible(UnitId::Second, UnitId::Hour));
        assert!(compatible(UnitId::Radian, UnitId::Degree));
    }

    #[test]
    fn test_compatible_different_dimension() {
        assert!(!compatible(UnitId::Meter, UnitId::Second));
        assert!(!compatible(UnitId::Hour, UnitId::Radian));
        assert!(!compatible(UnitId::Degree, UnitId::Kilometer));
    }

    #[test]
    fn test_convert_meters_to_kilometers() {
        let result = convert_value(1000.0, UnitId::Meter, UnitId::Kilometer).unwrap();
        assert_relative_eq!(result, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_kilometers_to_meters() {
        let result = convert_value(1.0, UnitId::Kilometer, UnitId::Meter).unwrap();
        assert_relative_eq!(result, 1000.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_seconds_to_minutes() {
        let result = convert_value(60.0, UnitId::Second, UnitId::Minute).unwrap();
        assert_relative_eq!(result, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_seconds_to_hours() {
        let result = convert_value(3600.0, UnitId::Second, UnitId::Hour).unwrap();
        assert_relative_eq!(result, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_hours_to_seconds() {
        let result = convert_value(1.0, UnitId::Hour, UnitId::Second).unwrap();
        assert_relative_eq!(result, 3600.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_days_to_hours() {
        let result = convert_value(1.0, UnitId::Day, UnitId::Hour).unwrap();
        assert_relative_eq!(result, 24.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_degrees_to_radians() {
        let result = convert_value(180.0, UnitId::Degree, UnitId::Radian).unwrap();
        assert_relative_eq!(result, PI, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_radians_to_degrees() {
        let result = convert_value(PI, UnitId::Radian, UnitId::Degree).unwrap();
        assert_relative_eq!(result, 180.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_same_unit() {
        let result = convert_value(42.0, UnitId::Meter, UnitId::Meter).unwrap();
        assert_relative_eq!(result, 42.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_incompatible_dimensions() {
        let result = convert_value(1.0, UnitId::Meter, UnitId::Second);
        assert_eq!(result, Err(QTTY_ERR_INCOMPATIBLE_DIM));
    }

    #[test]
    fn test_convert_preserves_special_values() {
        // NaN
        let nan_result = convert_value(f64::NAN, UnitId::Meter, UnitId::Kilometer).unwrap();
        assert!(nan_result.is_nan());

        // Infinity
        let inf_result = convert_value(f64::INFINITY, UnitId::Second, UnitId::Minute).unwrap();
        assert!(inf_result.is_infinite() && inf_result.is_sign_positive());

        // Negative infinity
        let neg_inf_result =
            convert_value(f64::NEG_INFINITY, UnitId::Second, UnitId::Minute).unwrap();
        assert!(neg_inf_result.is_infinite() && neg_inf_result.is_sign_negative());
    }

    #[test]
    fn test_convert_value_status_success() {
        let mut out = 0.0;
        let status = convert_value_status(2.0, UnitId::Hour, UnitId::Minute, &mut out);
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out, 120.0, epsilon = 1e-12);
    }

    #[test]
    fn test_convert_value_status_incompatible_dimension() {
        let mut out = -1.0;
        let status = convert_value_status(1.0, UnitId::Meter, UnitId::Second, &mut out);
        assert_eq!(status, QTTY_ERR_INCOMPATIBLE_DIM);
        assert_relative_eq!(out, -1.0, epsilon = 1e-12);
    }
}
