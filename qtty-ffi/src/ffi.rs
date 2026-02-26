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
    DimensionId, QttyDerivedQuantity, QttyQuantity, UnitId, QTTY_ERR_BUFFER_TOO_SMALL,
    QTTY_ERR_INCOMPATIBLE_DIM, QTTY_ERR_INVALID_VALUE, QTTY_ERR_NULL_OUT, QTTY_ERR_UNKNOWN_UNIT,
    QTTY_FMT_LOWER_EXP, QTTY_FMT_UPPER_EXP, QTTY_OK,
};
use core::ffi::c_char;
use std::ffi::{CStr, CString};

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
pub unsafe extern "C" fn qtty_unit_dimension(unit: UnitId, out: *mut DimensionId) -> i32 {
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
pub unsafe extern "C" fn qtty_units_compatible(a: UnitId, b: UnitId, out: *mut bool) -> i32 {
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
pub unsafe extern "C" fn qtty_quantity_make(
    value: f64,
    unit: UnitId,
    out: *mut QttyQuantity,
) -> i32 {
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
pub unsafe extern "C" fn qtty_quantity_convert(
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
pub unsafe extern "C" fn qtty_quantity_convert_value(
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
// String Formatting
// =============================================================================

/// Formats a quantity as a human-readable string into a caller-provided buffer.
///
/// Produces a string like `"1234.57 m"`, `"1.23e3 km"`, or `"1.23E3 km"` depending
/// on the `flags` parameter. The precision and format type mirror Rust's `{:.2}`,
/// `{:.4e}`, and `{:.4E}` format annotations, allowing callers to pass the same
/// format parameters that the Rust `Display`, `LowerExp`, and `UpperExp` trait impls
/// use internally.
///
/// # Arguments
///
/// * `qty`       - The quantity (`value + unit`) to format.
/// * `precision` - Number of decimal digits after the point.  Pass `-1` for the
///   default precision (shortest exact representation for floats).
/// * `flags`     - Selects the notation:
///   - `QTTY_FMT_DEFAULT`   (0): decimal notation, e.g. `"1234.568 m"`
///   - `QTTY_FMT_LOWER_EXP` (1): scientific with lowercase `e`, e.g. `"1.235e3 m"`
///   - `QTTY_FMT_UPPER_EXP` (2): scientific with uppercase `E`, e.g. `"1.235E3 m"`
/// * `buf`       - Caller-allocated output buffer (must be non-null).
/// * `buf_len`   - Size of `buf` in bytes (must include space for the NUL terminator).
///
/// # Returns
///
/// * Non-negative: number of bytes written, **excluding** the NUL terminator.
/// * `QTTY_ERR_NULL_OUT`        if `buf` is null.
/// * `QTTY_ERR_UNKNOWN_UNIT`    if `qty.unit` is not a recognized unit ID.
/// * `QTTY_ERR_BUFFER_TOO_SMALL` if `buf_len` is too small; the formatted string
///   (including the NUL terminator) requires `-return_value` bytes.
///
/// # Safety
///
/// The caller must ensure that `buf` points to a writable allocation of at least
/// `buf_len` bytes.  The written string is always NUL-terminated on success.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_format(
    qty: QttyQuantity,
    precision: i32,
    flags: u32,
    buf: *mut c_char,
    buf_len: usize,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if buf.is_null() || buf_len == 0 {
            return QTTY_ERR_NULL_OUT;
        }

        if crate::registry::meta(qty.unit).is_none() {
            return QTTY_ERR_UNKNOWN_UNIT;
        }

        let symbol = qty.unit.symbol();
        let formatted = match flags {
            QTTY_FMT_LOWER_EXP => {
                if precision >= 0 {
                    format!(
                        "{:.prec$e} {}",
                        qty.value,
                        symbol,
                        prec = precision as usize
                    )
                } else {
                    format!("{:e} {}", qty.value, symbol)
                }
            }
            QTTY_FMT_UPPER_EXP => {
                if precision >= 0 {
                    format!(
                        "{:.prec$E} {}",
                        qty.value,
                        symbol,
                        prec = precision as usize
                    )
                } else {
                    format!("{:E} {}", qty.value, symbol)
                }
            }
            // QTTY_FMT_DEFAULT or any unrecognised flag → decimal notation
            _ => {
                if precision >= 0 {
                    format!("{:.prec$} {}", qty.value, symbol, prec = precision as usize)
                } else {
                    format!("{} {}", qty.value, symbol)
                }
            }
        };

        let bytes = formatted.as_bytes();
        let needed = bytes.len() + 1; // +1 for NUL terminator

        if buf_len < needed {
            return QTTY_ERR_BUFFER_TOO_SMALL;
        }

        // SAFETY: buf is non-null (checked above) and buf_len >= needed
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, buf, bytes.len());
            *buf.add(bytes.len()) = 0; // NUL terminator
        }

        bytes.len() as i32
    })
}

// JSON Serialization / Deserialization via serde_json
//
// These helpers use serde for robust JSON serialization/deserialization.
// They produce/consume either a plain numeric value (e.g. "123.45") or an object
// with `value` and `unit` fields: {"value":123.45,"unit":"Meter"}
// =============================================================================

/// Frees a string previously allocated by one of the `qtty_*_to_json*` functions.
///
/// # Safety
///
/// The pointer must have been returned by a `qtty_*_to_json*` function and must
/// not have been freed previously. Passing a null pointer is safe (no-op).
#[no_mangle]
pub unsafe extern "C" fn qtty_string_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    // Reclaim the CString to free the memory allocated by `into_raw`.
    unsafe {
        let _ = CString::from_raw(s);
    }
}

/// Serializes a quantity's value as a plain JSON number string (e.g. "123.45").
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a `*mut c_char`,
/// or is null (in which case an error is returned). The returned string must be freed
/// with [`qtty_string_free`].
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_to_json_value(
    src: QttyQuantity,
    out: *mut *mut c_char,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }
        let s = serde_json::to_string(&src.value).unwrap_or_default();
        let c = CString::new(s).unwrap_or_default();
        unsafe {
            *out = c.into_raw();
        }
        QTTY_OK
    })
}

/// Deserializes a quantity from a plain JSON numeric string with an explicit unit.
///
/// # Safety
///
/// The caller must ensure that `json` points to a valid NUL-terminated C string,
/// and `out` points to valid, writable memory for a `QttyQuantity`.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_from_json_value(
    unit: UnitId,
    json: *const c_char,
    out: *mut QttyQuantity,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if json.is_null() || out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }
        let cstr = unsafe { CStr::from_ptr(json) };
        let s = match cstr.to_str() {
            Ok(v) => v,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };
        let v: f64 = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };
        if registry::meta(unit).is_none() {
            return QTTY_ERR_UNKNOWN_UNIT;
        }
        unsafe {
            *out = QttyQuantity::new(v, unit);
        }
        QTTY_OK
    })
}

/// Serializes a quantity to a full JSON object: `{"value":123.45,"unit":"Meter"}`.
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a `*mut c_char`,
/// or is null (in which case an error is returned). The returned string must be freed
/// with [`qtty_string_free`].
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_to_json(src: QttyQuantity, out: *mut *mut c_char) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }
        let s = match serde_json::to_string(&src) {
            Ok(s) => s,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };
        let c = CString::new(s).unwrap_or_default();
        unsafe {
            *out = c.into_raw();
        }
        QTTY_OK
    })
}

/// Deserializes a quantity from a JSON object: `{"value":123.45,"unit":"Meter"}`.
///
/// # Safety
///
/// The caller must ensure that `json` points to a valid NUL-terminated C string,
/// and `out` points to valid, writable memory for a `QttyQuantity`.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_from_json(
    json: *const c_char,
    out: *mut QttyQuantity,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if json.is_null() || out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }
        let cstr = unsafe { CStr::from_ptr(json) };
        let s = match cstr.to_str() {
            Ok(v) => v,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };
        let qty: QttyQuantity = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };
        // Validate that the unit is known
        if registry::meta(qty.unit).is_none() {
            return QTTY_ERR_UNKNOWN_UNIT;
        }
        unsafe {
            *out = qty;
        }
        QTTY_OK
    })
}

// =============================================================================
// Derived Quantity (Compound Unit) Functions
// =============================================================================

/// Creates a new derived quantity (compound unit like m/s).
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a
/// `QttyDerivedQuantity`, or is null (in which case an error is returned).
#[no_mangle]
pub unsafe extern "C" fn qtty_derived_make(
    value: f64,
    numerator: UnitId,
    denominator: UnitId,
    out: *mut QttyDerivedQuantity,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }
        if registry::meta(numerator).is_none() || registry::meta(denominator).is_none() {
            return QTTY_ERR_UNKNOWN_UNIT;
        }
        unsafe {
            *out = QttyDerivedQuantity::new(value, numerator, denominator);
        }
        QTTY_OK
    })
}

/// Converts a derived quantity to different units.
///
/// The numerator and denominator are converted independently while preserving
/// the compound value. For example, 100 m/s → 360 km/h.
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a
/// `QttyDerivedQuantity`, or is null (in which case an error is returned).
#[no_mangle]
pub unsafe extern "C" fn qtty_derived_convert(
    src: QttyDerivedQuantity,
    target_num: UnitId,
    target_den: UnitId,
    out: *mut QttyDerivedQuantity,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }
        match src.convert_to(target_num, target_den) {
            Some(converted) => {
                unsafe {
                    *out = converted;
                }
                QTTY_OK
            }
            None => {
                // Determine a more specific error code
                if registry::meta(src.numerator).is_none()
                    || registry::meta(src.denominator).is_none()
                    || registry::meta(target_num).is_none()
                    || registry::meta(target_den).is_none()
                {
                    QTTY_ERR_UNKNOWN_UNIT
                } else {
                    QTTY_ERR_INCOMPATIBLE_DIM
                }
            }
        }
    })
}

/// Serializes a derived quantity to a JSON object.
///
/// # Safety
///
/// The caller must ensure that `out` points to valid, writable memory for a `*mut c_char`.
/// The returned string must be freed with [`qtty_string_free`].
#[no_mangle]
pub unsafe extern "C" fn qtty_derived_to_json(
    src: QttyDerivedQuantity,
    out: *mut *mut c_char,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }
        let s = match serde_json::to_string(&src) {
            Ok(s) => s,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };
        let c = CString::new(s).unwrap_or_default();
        unsafe {
            *out = c.into_raw();
        }
        QTTY_OK
    })
}

/// Deserializes a derived quantity from a JSON object.
///
/// # Safety
///
/// The caller must ensure that `json` points to a valid NUL-terminated C string,
/// and `out` points to valid, writable memory for a `QttyDerivedQuantity`.
#[no_mangle]
pub unsafe extern "C" fn qtty_derived_from_json(
    json: *const c_char,
    out: *mut QttyDerivedQuantity,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if json.is_null() || out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }
        let cstr = unsafe { CStr::from_ptr(json) };
        let s = match cstr.to_str() {
            Ok(v) => v,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };
        let qty: QttyDerivedQuantity = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };
        if registry::meta(qty.numerator).is_none() || registry::meta(qty.denominator).is_none() {
            return QTTY_ERR_UNKNOWN_UNIT;
        }
        unsafe {
            *out = qty;
        }
        QTTY_OK
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
    use crate::types::QTTY_FMT_DEFAULT;
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

        let status = unsafe { qtty_unit_dimension(UnitId::Meter, &mut dim) };
        assert_eq!(status, QTTY_OK);
        assert_eq!(dim, DimensionId::Length);

        let status = unsafe { qtty_unit_dimension(UnitId::Second, &mut dim) };
        assert_eq!(status, QTTY_OK);
        assert_eq!(dim, DimensionId::Time);

        let status = unsafe { qtty_unit_dimension(UnitId::Radian, &mut dim) };
        assert_eq!(status, QTTY_OK);
        assert_eq!(dim, DimensionId::Angle);
    }

    #[test]
    fn test_unit_dimension_null_out() {
        let status = unsafe { qtty_unit_dimension(UnitId::Meter, core::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_units_compatible() {
        let mut result = false;

        let status =
            unsafe { qtty_units_compatible(UnitId::Meter, UnitId::Kilometer, &mut result) };
        assert_eq!(status, QTTY_OK);
        assert!(result);

        let status = unsafe { qtty_units_compatible(UnitId::Meter, UnitId::Second, &mut result) };
        assert_eq!(status, QTTY_OK);
        assert!(!result);
    }

    #[test]
    fn test_units_compatible_null_out() {
        let status = unsafe {
            qtty_units_compatible(UnitId::Meter, UnitId::Kilometer, core::ptr::null_mut())
        };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_make() {
        let mut q = QttyQuantity::default();

        let status = unsafe { qtty_quantity_make(1000.0, UnitId::Meter, &mut q) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(q.value, 1000.0);
        assert_eq!(q.unit, UnitId::Meter);
    }

    #[test]
    fn test_quantity_make_null_out() {
        let status = unsafe { qtty_quantity_make(1000.0, UnitId::Meter, core::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_convert_meters_to_kilometers() {
        let src = QttyQuantity::new(1000.0, UnitId::Meter);
        let mut dst = QttyQuantity::default();

        let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer, &mut dst) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(dst.value, 1.0, epsilon = 1e-12);
        assert_eq!(dst.unit, UnitId::Kilometer);
    }

    #[test]
    fn test_quantity_convert_seconds_to_hours() {
        let src = QttyQuantity::new(3600.0, UnitId::Second);
        let mut dst = QttyQuantity::default();

        let status = unsafe { qtty_quantity_convert(src, UnitId::Hour, &mut dst) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(dst.value, 1.0, epsilon = 1e-12);
        assert_eq!(dst.unit, UnitId::Hour);
    }

    #[test]
    fn test_quantity_convert_degrees_to_radians() {
        let src = QttyQuantity::new(180.0, UnitId::Degree);
        let mut dst = QttyQuantity::default();

        let status = unsafe { qtty_quantity_convert(src, UnitId::Radian, &mut dst) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(dst.value, PI, epsilon = 1e-12);
        assert_eq!(dst.unit, UnitId::Radian);
    }

    #[test]
    fn test_quantity_convert_incompatible() {
        let src = QttyQuantity::new(100.0, UnitId::Meter);
        let mut dst = QttyQuantity::default();

        let status = unsafe { qtty_quantity_convert(src, UnitId::Second, &mut dst) };
        assert_eq!(status, QTTY_ERR_INCOMPATIBLE_DIM);
    }

    #[test]
    fn test_quantity_convert_null_out() {
        let src = QttyQuantity::new(1000.0, UnitId::Meter);

        let status =
            unsafe { qtty_quantity_convert(src, UnitId::Kilometer, core::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_convert_value() {
        let mut out = 0.0;

        let status = unsafe {
            qtty_quantity_convert_value(1000.0, UnitId::Meter, UnitId::Kilometer, &mut out)
        };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_quantity_convert_value_null_out() {
        let status = unsafe {
            qtty_quantity_convert_value(
                1000.0,
                UnitId::Meter,
                UnitId::Kilometer,
                core::ptr::null_mut(),
            )
        };
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
    fn test_unit_name_all_dimensions() {
        // Each of: length, time, angle, mass, power
        for unit in [
            UnitId::Kilometer,
            UnitId::Hour,
            UnitId::Degree,
            UnitId::Kilogram,
            UnitId::Watt,
        ] {
            let ptr = qtty_unit_name(unit);
            assert!(
                !ptr.is_null(),
                "unit_name should not be null for {:?}",
                unit
            );
        }
    }

    #[test]
    fn test_quantity_convert_value_incompatible() {
        let mut out = 0.0;
        let status =
            unsafe { qtty_quantity_convert_value(1.0, UnitId::Meter, UnitId::Second, &mut out) };
        assert_eq!(status, QTTY_ERR_INCOMPATIBLE_DIM);
    }

    // ─── qtty_string_free ────────────────────────────────────────────────────

    #[test]
    fn test_string_free_null_is_noop() {
        // Must not crash
        unsafe { qtty_string_free(std::ptr::null_mut()) };
    }

    #[test]
    fn test_string_free_valid_ptr() {
        // Allocate a string via to_json_value then free it
        let src = QttyQuantity::new(1.0, UnitId::Meter);
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
        let status = unsafe { qtty_quantity_to_json_value(src, &mut ptr) };
        assert_eq!(status, QTTY_OK);
        assert!(!ptr.is_null());
        unsafe { qtty_string_free(ptr) }; // must not crash or leak
    }

    // ─── qtty_quantity_to_json_value / qtty_quantity_from_json_value ─────────

    #[test]
    fn test_quantity_to_json_value_success() {
        let src = QttyQuantity::new(42.5, UnitId::Meter);
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
        let status = unsafe { qtty_quantity_to_json_value(src, &mut ptr) };
        assert_eq!(status, QTTY_OK);
        assert!(!ptr.is_null());
        let s = unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap() };
        assert_eq!(s, "42.5");
        unsafe { qtty_string_free(ptr) };
    }

    #[test]
    fn test_quantity_to_json_value_null_out() {
        let src = QttyQuantity::new(1.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_to_json_value(src, std::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_from_json_value_success() {
        let json = std::ffi::CString::new("99.0").unwrap();
        let mut out = QttyQuantity::default();
        let status =
            unsafe { qtty_quantity_from_json_value(UnitId::Second, json.as_ptr(), &mut out) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out.value, 99.0);
        assert_eq!(out.unit, UnitId::Second);
    }

    #[test]
    fn test_quantity_from_json_value_null_json() {
        let mut out = QttyQuantity::default();
        let status =
            unsafe { qtty_quantity_from_json_value(UnitId::Meter, std::ptr::null(), &mut out) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_from_json_value_null_out() {
        let json = std::ffi::CString::new("1.0").unwrap();
        let status = unsafe {
            qtty_quantity_from_json_value(UnitId::Meter, json.as_ptr(), std::ptr::null_mut())
        };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_from_json_value_invalid_json() {
        let json = std::ffi::CString::new("not_a_number").unwrap();
        let mut out = QttyQuantity::default();
        let status =
            unsafe { qtty_quantity_from_json_value(UnitId::Meter, json.as_ptr(), &mut out) };
        assert_eq!(status, QTTY_ERR_INVALID_VALUE);
    }

    #[test]
    fn test_quantity_json_value_roundtrip() {
        let src = QttyQuantity::new(1234.567, UnitId::Kilometer);
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
        unsafe { qtty_quantity_to_json_value(src, &mut ptr) };
        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_from_json_value(UnitId::Kilometer, ptr, &mut out) };
        unsafe { qtty_string_free(ptr) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out.value, 1234.567, epsilon = 1e-9);
    }

    // ─── qtty_quantity_to_json / qtty_quantity_from_json ─────────────────────

    #[test]
    fn test_quantity_to_json_success() {
        let src = QttyQuantity::new(1.0, UnitId::Hour);
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
        let status = unsafe { qtty_quantity_to_json(src, &mut ptr) };
        assert_eq!(status, QTTY_OK);
        assert!(!ptr.is_null());
        let s = unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap() };
        // Must include "value" and unit name
        assert!(s.contains("value"));
        assert!(s.contains("Hour"));
        unsafe { qtty_string_free(ptr) };
    }

    #[test]
    fn test_quantity_to_json_null_out() {
        let src = QttyQuantity::new(1.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_to_json(src, std::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_from_json_success() {
        // Serialize first to get correct format
        let src = QttyQuantity::new(500.0, UnitId::Kilogram);
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
        unsafe { qtty_quantity_to_json(src, &mut ptr) };

        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_from_json(ptr, &mut out) };
        unsafe { qtty_string_free(ptr) };

        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out.value, 500.0);
        assert_eq!(out.unit, UnitId::Kilogram);
    }

    #[test]
    fn test_quantity_from_json_null_json() {
        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_from_json(std::ptr::null(), &mut out) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_from_json_null_out() {
        let json = std::ffi::CString::new(r#"{"value":1.0,"unit":"Meter"}"#).unwrap();
        let status = unsafe { qtty_quantity_from_json(json.as_ptr(), std::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_quantity_from_json_invalid_json() {
        let json = std::ffi::CString::new("not valid json at all").unwrap();
        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_from_json(json.as_ptr(), &mut out) };
        assert_eq!(status, QTTY_ERR_INVALID_VALUE);
    }

    #[test]
    fn test_quantity_json_object_roundtrip() {
        let src = QttyQuantity::new(PI, UnitId::Radian);
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
        unsafe { qtty_quantity_to_json(src, &mut ptr) };
        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_from_json(ptr, &mut out) };
        unsafe { qtty_string_free(ptr) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out.value, PI, epsilon = 1e-12);
        assert_eq!(out.unit, UnitId::Radian);
    }

    // ─── qtty_derived_make ───────────────────────────────────────────────────

    #[test]
    fn test_derived_make_success() {
        let mut out = QttyDerivedQuantity::default();
        let status = unsafe { qtty_derived_make(100.0, UnitId::Meter, UnitId::Second, &mut out) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out.value, 100.0);
        assert_eq!(out.numerator, UnitId::Meter);
        assert_eq!(out.denominator, UnitId::Second);
    }

    #[test]
    fn test_derived_make_null_out() {
        let status =
            unsafe { qtty_derived_make(1.0, UnitId::Meter, UnitId::Second, std::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    // ─── qtty_derived_convert ────────────────────────────────────────────────

    #[test]
    fn test_derived_convert_success() {
        // 100 m/s → 360 km/h
        let src = QttyDerivedQuantity::new(100.0, UnitId::Meter, UnitId::Second);
        let mut out = QttyDerivedQuantity::default();
        let status =
            unsafe { qtty_derived_convert(src, UnitId::Kilometer, UnitId::Hour, &mut out) };
        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out.value, 360.0, epsilon = 1e-9);
        assert_eq!(out.numerator, UnitId::Kilometer);
        assert_eq!(out.denominator, UnitId::Hour);
    }

    #[test]
    fn test_derived_convert_null_out() {
        let src = QttyDerivedQuantity::new(1.0, UnitId::Meter, UnitId::Second);
        let status = unsafe {
            qtty_derived_convert(src, UnitId::Kilometer, UnitId::Hour, std::ptr::null_mut())
        };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_derived_convert_incompatible_dim() {
        // m/s → kg/h: incompatible numerator dimension
        let src = QttyDerivedQuantity::new(1.0, UnitId::Meter, UnitId::Second);
        let mut out = QttyDerivedQuantity::default();
        let status = unsafe { qtty_derived_convert(src, UnitId::Kilogram, UnitId::Hour, &mut out) };
        assert_eq!(status, QTTY_ERR_INCOMPATIBLE_DIM);
    }

    // ─── qtty_derived_to_json / qtty_derived_from_json ───────────────────────

    #[test]
    fn test_derived_to_json_success() {
        let src = QttyDerivedQuantity::new(100.0, UnitId::Meter, UnitId::Second);
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
        let status = unsafe { qtty_derived_to_json(src, &mut ptr) };
        assert_eq!(status, QTTY_OK);
        assert!(!ptr.is_null());
        let s = unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap() };
        assert!(s.contains("value"));
        assert!(s.contains("Meter"));
        assert!(s.contains("Second"));
        unsafe { qtty_string_free(ptr) };
    }

    #[test]
    fn test_derived_to_json_null_out() {
        let src = QttyDerivedQuantity::new(1.0, UnitId::Meter, UnitId::Second);
        let status = unsafe { qtty_derived_to_json(src, std::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_derived_from_json_success() {
        // Roundtrip via to_json then from_json
        let src = QttyDerivedQuantity::new(360.0, UnitId::Kilometer, UnitId::Hour);
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
        unsafe { qtty_derived_to_json(src, &mut ptr) };

        let mut out = QttyDerivedQuantity::default();
        let status = unsafe { qtty_derived_from_json(ptr, &mut out) };
        unsafe { qtty_string_free(ptr) };

        assert_eq!(status, QTTY_OK);
        assert_relative_eq!(out.value, 360.0);
        assert_eq!(out.numerator, UnitId::Kilometer);
        assert_eq!(out.denominator, UnitId::Hour);
    }

    #[test]
    fn test_derived_from_json_null_json() {
        let mut out = QttyDerivedQuantity::default();
        let status = unsafe { qtty_derived_from_json(std::ptr::null(), &mut out) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_derived_from_json_null_out() {
        let json =
            std::ffi::CString::new(r#"{"value":1.0,"numerator":"Meter","denominator":"Second"}"#)
                .unwrap();
        let status = unsafe { qtty_derived_from_json(json.as_ptr(), std::ptr::null_mut()) };
        assert_eq!(status, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_derived_from_json_invalid_json() {
        let json = std::ffi::CString::new("not json").unwrap();
        let mut out = QttyDerivedQuantity::default();
        let status = unsafe { qtty_derived_from_json(json.as_ptr(), &mut out) };
        assert_eq!(status, QTTY_ERR_INVALID_VALUE);
    }

    #[test]
    fn test_ffi_version() {
        assert_eq!(qtty_ffi_version(), 1);
    }

    // -------------------------------------------------------------------------
    // qtty_quantity_format tests
    // -------------------------------------------------------------------------

    fn format_qty(qty: QttyQuantity, precision: i32, flags: u32) -> String {
        let mut buf = [0i8; 256];
        let result =
            unsafe { qtty_quantity_format(qty, precision, flags, buf.as_mut_ptr(), buf.len()) };
        assert!(result >= 0, "qtty_quantity_format returned error {result}");
        let c_str = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) };
        c_str.to_str().unwrap().to_owned()
    }

    #[test]
    fn test_format_default_no_precision() {
        let qty = QttyQuantity::new(1234.56789, UnitId::Second);
        let s = format_qty(qty, -1, QTTY_FMT_DEFAULT);
        assert_eq!(s, "1234.56789 s");
    }

    #[test]
    fn test_format_default_two_decimal_places() {
        let qty = QttyQuantity::new(1234.56789, UnitId::Second);
        let s = format_qty(qty, 2, QTTY_FMT_DEFAULT);
        assert_eq!(s, "1234.57 s");
    }

    #[test]
    fn test_format_lower_exp_no_precision() {
        let qty = QttyQuantity::new(1234.56789, UnitId::Second);
        let s = format_qty(qty, -1, QTTY_FMT_LOWER_EXP);
        assert_eq!(s, "1.23456789e3 s");
    }

    #[test]
    fn test_format_lower_exp_four_decimal_places() {
        let qty = QttyQuantity::new(1234.56789, UnitId::Second);
        let s = format_qty(qty, 4, QTTY_FMT_LOWER_EXP);
        assert_eq!(s, "1.2346e3 s");
    }

    #[test]
    fn test_format_upper_exp_four_decimal_places() {
        let qty = QttyQuantity::new(1234.56789, UnitId::Second);
        let s = format_qty(qty, 4, QTTY_FMT_UPPER_EXP);
        assert_eq!(s, "1.2346E3 s");
    }

    #[test]
    fn test_format_meters_default() {
        let qty = QttyQuantity::new(42.0, UnitId::Meter);
        let s = format_qty(qty, -1, QTTY_FMT_DEFAULT);
        assert_eq!(s, "42 m");
    }

    #[test]
    fn test_format_null_buf() {
        let qty = QttyQuantity::new(1.0, UnitId::Meter);
        let result =
            unsafe { qtty_quantity_format(qty, -1, QTTY_FMT_DEFAULT, core::ptr::null_mut(), 64) };
        assert_eq!(result, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_format_zero_buf_len() {
        let qty = QttyQuantity::new(1.0, UnitId::Meter);
        let mut buf = [0i8; 4];
        let result =
            unsafe { qtty_quantity_format(qty, -1, QTTY_FMT_DEFAULT, buf.as_mut_ptr(), 0) };
        assert_eq!(result, QTTY_ERR_NULL_OUT);
    }

    #[test]
    fn test_format_buffer_too_small() {
        let qty = QttyQuantity::new(1234.56789, UnitId::Second);
        let mut buf = [0i8; 4]; // way too small
        let result =
            unsafe { qtty_quantity_format(qty, 2, QTTY_FMT_DEFAULT, buf.as_mut_ptr(), buf.len()) };
        assert_eq!(result, QTTY_ERR_BUFFER_TOO_SMALL);
    }
}
