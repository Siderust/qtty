// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

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

use crate::types::{DimensionId, QttyStatus, UnitId};

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
/// * `Err(QttyStatus::UnknownUnit)` if either unit is not recognized
/// * `Err(QttyStatus::IncompatibleDim)` if units have different dimensions
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
pub fn convert_value(v: f64, src: UnitId, dst: UnitId) -> Result<f64, QttyStatus> {
    let src_meta = meta(src).ok_or(QttyStatus::UnknownUnit)?;
    let dst_meta = meta(dst).ok_or(QttyStatus::UnknownUnit)?;

    if src_meta.dim != dst_meta.dim {
        return Err(QttyStatus::IncompatibleDim);
    }

    // If same unit, no conversion needed
    if src == dst {
        return Ok(v);
    }

    // Avoid intermediate overflow: always multiply the value in the *smaller*-scale
    // unit by the ratio of the two scales, which is ≤ 1 from that unit's perspective.
    // This mirrors `qtty_core::Quantity::to()` and prevents intermediate `inf` even
    // when the final result is representable (e.g. 1e292 Ym → ~1.057e300 ly).
    let s = src_meta.scale_to_canonical;
    let d = dst_meta.scale_to_canonical;
    let v_dst = if s >= d {
        // src has the larger scale; multiply *dst* side (scale ≤ 1)
        v * (s / d)
    } else {
        // dst has the larger scale; multiply *src* side (scale ≤ 1)
        v / (d / s)
    };

    Ok(v_dst)
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
        assert_eq!(result, Err(QttyStatus::IncompatibleDim));
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

    /// Regression: large-magnitude conversion must not overflow to inf in the
    /// intermediate step when the final result is still representable.
    ///
    /// Old path: `v * src_scale` → `1e292 * 1e24` → inf before dividing.
    /// Fixed:    scale only the smaller-scale side by ratio ≤ 1.
    ///
    /// LightYear scale ≈ 9.461e15 m, Yottameter scale = 1e24 m.
    /// 1e292 Ym ≈ 1.057e300 ly (finite, < f64::MAX ≈ 1.798e308).
    #[test]
    fn test_convert_large_magnitude_no_intermediate_overflow() {
        let result = convert_value(1e292, UnitId::Yottameter, UnitId::LightYear);
        let result = result.expect("conversion must succeed");
        assert!(
            result.is_finite(),
            "1e292 Ym → ly must be finite, got {result}"
        );
        // Cross-check against the expected value (1e292 * (1e24 / METERS_PER_LY))
        let meters_per_ly: f64 = 299_792_458.0 * 86_400.0 * 365.25;
        let expected = 1e292 * (1e24 / meters_per_ly);
        assert!(
            (result / expected - 1.0).abs() < 1e-10,
            "expected ~{expected:e}, got {result:e}"
        );
    }

    /// Regression: the reverse direction (smaller → larger scale) must also
    /// stay finite for large magnitudes.
    #[test]
    fn test_convert_large_magnitude_reverse_no_intermediate_overflow() {
        // 1e300 ly → Ym: ly has smaller scale, Ym has larger scale.
        // The fixed path divides by (dst_scale / src_scale) ≤ 1 from the src side.
        let result = convert_value(1e300, UnitId::LightYear, UnitId::Yottameter);
        let result = result.expect("conversion must succeed");
        // 1e300 ly * METERS_PER_LY / 1e24 ≈ 9.46e291, which is finite.
        assert!(
            result.is_finite(),
            "1e300 ly → Ym must be finite, got {result}"
        );
    }
}
