//! Extern "C" API for FFI consumers.
//!
//! This module exposes `#[no_mangle] pub extern "C"` functions that form the stable C ABI
//! for `qtty-ffi`. These functions can be called from C/C++ code or any language with C FFI support.
//!
//! # Safety
//!
//! All functions in this module:
//! - Never panic across FFI boundaries (all panics are caught and converted to error codes)
//! - Validate all input pointers before use
//! - Return status codes to indicate success or failure
//!
//! # Status Codes
//!
//! - `QTTY_OK` (0): Success
//! - `QTTY_ERR_UNKNOWN_UNIT` (-1): Invalid or unrecognized unit ID
//! - `QTTY_ERR_INCOMPATIBLE_DIM` (-2): Units have different dimensions
//! - `QTTY_ERR_NULL_OUT` (-3): Required output pointer was null
//! - `QTTY_ERR_INVALID_VALUE` (-4): Invalid value (reserved)

use crate::registry;
use crate::types::{
    DimensionId, QttyQuantity, UnitId, QTTY_ERR_NULL_OUT,
    QTTY_ERR_UNKNOWN_UNIT, QTTY_OK,
};
use core::ffi::c_char;

// =============================================================================
// Helper macro to catch panics
// =============================================================================

/// Catches any panic and returns an error code instead of unwinding across FFI.
macro_rules! catch_panic {
    ($default:expr, $body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(result) => result,
            Err(_) => $default,
        }
    }};
}

// =============================================================================
// Unit Validation / Info Functions
// =============================================================================

/// Checks if a unit ID is valid (recognized by the registry).
///
/// # Arguments
///
/// * `unit` - The unit ID to validate
///
/// # Returns
///
/// `true` if the unit is valid, `false` otherwise.
///
/// # Safety
///
/// This function is safe to call from any context.
#[no_mangle]
pub extern "C" fn qtty_unit_is_valid(unit: UnitId) -> bool {
    catch_panic!(false, registry::meta(unit).is_some())
}

/// Gets the dimension of a unit.
///
/// # Arguments
///
/// * `unit` - The unit ID to query
/// * `out` - Pointer to store the dimension ID
///
/// # Returns
///
/// * `QTTY_OK` on success
/// * `QTTY_ERR_NULL_OUT` if `out` is null
/// * `QTTY_ERR_UNKNOWN_UNIT` if the unit is not recognized
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a `DimensionId`,
/// or is null (in which case an error is returned).
#[no_mangle]
pub extern "C" fn qtty_unit_dimension(unit: UnitId, out: *mut DimensionId) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }

        match registry::dimension(unit) {
            Some(dim) => {
                // SAFETY: We checked that `out` is not null
                unsafe { *out = dim };
                QTTY_OK
            }
            None => QTTY_ERR_UNKNOWN_UNIT,
        }
    })
}

/// Checks if two units are compatible (same dimension).
///
/// # Arguments
///
/// * `a` - First unit ID
/// * `b` - Second unit ID
/// * `out` - Pointer to store the result
///
/// # Returns
///
/// * `QTTY_OK` on success
/// * `QTTY_ERR_NULL_OUT` if `out` is null
/// * `QTTY_ERR_UNKNOWN_UNIT` if either unit is not recognized
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a `bool`,
/// or is null (in which case an error is returned).
#[no_mangle]
pub extern "C" fn qtty_units_compatible(a: UnitId, b: UnitId, out: *mut bool) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }

        // Validate both units exist
        if registry::meta(a).is_none() || registry::meta(b).is_none() {
            return QTTY_ERR_UNKNOWN_UNIT;
        }

        // SAFETY: We checked that `out` is not null
        unsafe { *out = registry::compatible(a, b) };
        QTTY_OK
    })
}

// =============================================================================
// Quantity Construction and Conversion Functions
// =============================================================================

/// Creates a new quantity with the given value and unit.
///
/// # Arguments
///
/// * `value` - The numeric value
/// * `unit` - The unit ID
/// * `out` - Pointer to store the resulting quantity
///
/// # Returns
///
/// * `QTTY_OK` on success
/// * `QTTY_ERR_NULL_OUT` if `out` is null
/// * `QTTY_ERR_UNKNOWN_UNIT` if the unit is not recognized
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a `QttyQuantity`,
/// or is null (in which case an error is returned).
#[no_mangle]
pub extern "C" fn qtty_quantity_make(value: f64, unit: UnitId, out: *mut QttyQuantity) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }

        // Validate unit exists
        if registry::meta(unit).is_none() {
            return QTTY_ERR_UNKNOWN_UNIT;
        }

        // SAFETY: We checked that `out` is not null
        unsafe {
            *out = QttyQuantity::new(value, unit);
        }
        QTTY_OK
    })
}

/// Converts a quantity to a different unit.
///
/// # Arguments
///
/// * `src` - The source quantity
/// * `dst_unit` - The target unit ID
/// * `out` - Pointer to store the converted quantity
///
/// # Returns
///
/// * `QTTY_OK` on success
/// * `QTTY_ERR_NULL_OUT` if `out` is null
/// * `QTTY_ERR_UNKNOWN_UNIT` if either unit is not recognized
/// * `QTTY_ERR_INCOMPATIBLE_DIM` if units have different dimensions
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a `QttyQuantity`,
/// or is null (in which case an error is returned).
#[no_mangle]
pub extern "C" fn qtty_quantity_convert(
    src: QttyQuantity,
    dst_unit: UnitId,
    out: *mut QttyQuantity,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }

        match registry::convert_value(src.value, src.unit, dst_unit) {
            Ok(converted_value) => {
                // SAFETY: We checked that `out` is not null
                unsafe {
                    *out = QttyQuantity::new(converted_value, dst_unit);
                }
                QTTY_OK
            }
            Err(code) => code,
        }
    })
}

/// Converts a value from one unit to another.
///
/// This is a convenience function that operates on raw values instead of `QttyQuantity` structs.
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `src_unit` - The source unit ID
/// * `dst_unit` - The target unit ID
/// * `out_value` - Pointer to store the converted value
///
/// # Returns
///
/// * `QTTY_OK` on success
/// * `QTTY_ERR_NULL_OUT` if `out_value` is null
/// * `QTTY_ERR_UNKNOWN_UNIT` if either unit is not recognized
/// * `QTTY_ERR_INCOMPATIBLE_DIM` if units have different dimensions
///
/// # Safety
///
/// The caller must ensure that `out_value` points to valid, writable memory for an `f64`,
/// or is null (in which case an error is returned).
#[no_mangle]
pub extern "C" fn qtty_quantity_convert_value(
    value: f64,
    src_unit: UnitId,
    dst_unit: UnitId,
    out_value: *mut f64,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out_value.is_null() {
            return QTTY_ERR_NULL_OUT;
        }

        match registry::convert_value(value, src_unit, dst_unit) {
            Ok(converted) => {
                // SAFETY: We checked that `out_value` is not null
                unsafe {
                    *out_value = converted;
                }
                QTTY_OK
            }
            Err(code) => code,
        }
    })
}

/// Gets the name of a unit as a NUL-terminated C string.
///
/// # Arguments
///
/// * `unit` - The unit ID to query
///
/// # Returns
///
/// A pointer to a static, NUL-terminated C string with the unit name,
/// or a null pointer if the unit is not recognized.
///
/// # Safety
///
/// The returned pointer points to static memory and is valid for the lifetime
/// of the program. The caller must not attempt to free or modify the returned string.
#[no_mangle]
pub extern "C" fn qtty_unit_name(unit: UnitId) -> *const c_char {
    catch_panic!(core::ptr::null(), {
        if registry::meta(unit).is_some() {
            unit.name_cstr()
        } else {
            core::ptr::null()
        }
    })
}

// =============================================================================
// Version Info
// =============================================================================

/// Returns the FFI ABI version.
///
/// This can be used by consumers to verify compatibility. The version is
/// incremented when breaking changes are made to the ABI.
///
/// Current version: 1
#[no_mangle]
pub extern "C" fn qtty_ffi_version() -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QTTY_ERR_INCOMPATIBLE_DIM;
    use approx::assert_relative_eq;
    use core::f64::consts::PI;

    #[test]
    fn test_unit_is_valid() {
        assert!(qtty_unit_is_valid(UnitId::Meter));
        assert!(qtty_unit_is_valid(UnitId::Second));
        assert!(qtty_unit_is_valid(UnitId::Radian));
    }

    #[test]
    fn test_unit_dimension() {
        let mut dim = DimensionId::Length;

        let status = qtty_unit_dimension(UnitId::Meter, &mut dim);
        assert_eq!(status, QTTY_OK);
        assert_eq!(dim, DimensionId::Length);

        let status = qtty_unit_dimension(UnitId::Second, &mut dim);
        assert_eq!(status, QTTY_OK);
        assert_eq!(dim, DimensionId::Time);

        let status = qtty_unit_dimension(UnitId::Radian, &mut dim);
        assert_eq!(status, QTTY_OK);
        assert_eq!(dim, DimensionId::Angle);
    }

    #[test]
    fn test_unit_dimension_null_out() {
        let status = qtty_unit_dimension(UnitId::Meter, core::ptr::null_mut());
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_units_compatible() {
        let mut result = false;

        let status = qtty_units_compatible(UnitId::Meter, UnitId::Kilometer, &mut result);
        assert_eq!(status, QTTY_OK);
        assert!(result);

        let status = qtty_units_compatible(UnitId::Meter, UnitId::Second, &mut result);
        assert_eq!(status, QTTY_OK);
        assert!(!result);
    }

    #[test]
    fn test_units_compatible_null_out() {
        let status = qtty_units_compatible(UnitId::Meter, UnitId::Kilometer, core::ptr::null_mut());
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_make() {
        let mut q = QttyQuantity::default();

        let status = qtty_quantity_make(1000.0, UnitId::Meter, &mut q);
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(q.value, 1000.0);
        assert_eq!(q.unit, UnitId::Meter);
    }

    #[test]
    fn test_quantity_make_null_out() {
        let status = qtty_quantity_make(1000.0, UnitId::Meter, core::ptr::null_mut());
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_convert_meters_to_kilometers() {
        let src = QttyQuantity::new(1000.0, UnitId::Meter);
        let mut dst = QttyQuantity::default();

        let status = qtty_quantity_convert(src, UnitId::Kilometer, &mut dst);
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(dst.value, 1.0, epsilon = 1e-12);
        assert_eq!(dst.unit, UnitId::Kilometer);
    }

    #[test]
    fn test_quantity_convert_seconds_to_hours() {
        let src = QttyQuantity::new(3600.0, UnitId::Second);
        let mut dst = QttyQuantity::default();

        let status = qtty_quantity_convert(src, UnitId::Hour, &mut dst);
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(dst.value, 1.0, epsilon = 1e-12);
        assert_eq!(dst.unit, UnitId::Hour);
    }

    #[test]
    fn test_quantity_convert_degrees_to_radians() {
        let src = QttyQuantity::new(180.0, UnitId::Degree);
        let mut dst = QttyQuantity::default();

        let status = qtty_quantity_convert(src, UnitId::Radian, &mut dst);
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(dst.value, PI, epsilon = 1e-12);
        assert_eq!(dst.unit, UnitId::Radian);
    }

    #[test]
    fn test_quantity_convert_incompatible() {
        let src = QttyQuantity::new(100.0, UnitId::Meter);
        let mut dst = QttyQuantity::default();

        let status = qtty_quantity_convert(src, UnitId::Second, &mut dst);
        assert_eq!(status, QTTY_ERR_INCOMPATIBLE_DIM);
    }

    #[test]
    fn test_quantity_convert_null_out() {
        let src = QttyQuantity::new(1000.0, UnitId::Meter);

        let status = qtty_quantity_convert(src, UnitId::Kilometer, core::ptr::null_mut());
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_convert_value() {
        let mut out = 0.0;

        let status = qtty_quantity_convert_value(1000.0, UnitId::Meter, UnitId::Kilometer, &mut out);
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_quantity_convert_value_null_out() {
        let status =
            qtty_quantity_convert_value(1000.0, UnitId::Meter, UnitId::Kilometer, core::ptr::null_mut());
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_unit_name() {
        let name_ptr = qtty_unit_name(UnitId::Meter);
        assert!(!name_ptr.is_null());

        // SAFETY: We verified the pointer is not null and points to static memory
        let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) };
        assert_eq!(name.to_str().unwrap(), "Meter");
    }

    #[test]
    fn test_ffi_version() {
        assert_eq!(qtty_ffi_version(), 1);
    }
}
