//! JSON serialization/deserialization FFI functions.
//!
//! Provides `extern "C"` functions for serializing and deserializing
//! [`QttyQuantity`] values to/from JSON, as well as a companion
//! `qtty_string_free` for releasing heap-allocated C strings returned by
//! these functions.
//!
//! # JSON formats
//!
//! * **value-only** (`qtty_quantity_to/from_json_value`): a bare JSON number,
//!   e.g. `123.456`.
//! * **full object** (`qtty_quantity_to/from_json`): a JSON object with
//!   `"value"` (f64) and `"unit_id"` (u32) fields,
//!   e.g. `{"value":123.456,"unit_id":10001}`.

use crate::types::{
    QttyQuantity, UnitId, QTTY_ERR_INVALID_VALUE, QTTY_ERR_NULL_OUT, QTTY_ERR_UNKNOWN_UNIT, QTTY_OK,
};
use core::ffi::c_char;
use std::ffi::{CStr, CString};

// =============================================================================
// Helper macro (mirrors the one in ffi.rs)
// =============================================================================

macro_rules! catch_panic {
    ($default:expr, $body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(result) => result,
            Err(_) => $default,
        }
    }};
}

// =============================================================================
// String management
// =============================================================================

/// Frees a C string previously returned by a `qtty_quantity_to_json*` function.
///
/// Passing a null pointer is safe and is a no-op.
///
/// # Safety
///
/// The pointer **must** have been returned by `qtty_quantity_to_json` or
/// `qtty_quantity_to_json_value` and must not be freed more than once.
#[no_mangle]
pub unsafe extern "C" fn qtty_string_free(s: *mut c_char) {
    if !s.is_null() {
        // SAFETY: Pointer was produced by `CString::into_raw` in this crate.
        unsafe { drop(CString::from_raw(s)) }
    }
}

// =============================================================================
// Value-only JSON (bare number)
// =============================================================================

/// Serializes only the numeric value of a quantity as a JSON number string.
///
/// On success, `*out_json` is set to a heap-allocated NUL-terminated C string
/// (e.g. `"123.456789"`) that the caller **must** release with
/// [`qtty_string_free`].
///
/// # Returns
///
/// * `QTTY_OK` on success.
/// * `QTTY_ERR_NULL_OUT` if `out_json` is null.
/// * `QTTY_ERR_INVALID_VALUE` if serialization fails.
///
/// # Safety
///
/// `out_json` must point to a writable `*mut c_char` location.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_to_json_value(
    src: QttyQuantity,
    out_json: *mut *mut c_char,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out_json.is_null() {
            return QTTY_ERR_NULL_OUT;
        }

        let s = match serde_json::to_string(&src.value) {
            Ok(s) => s,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };

        match CString::new(s) {
            Ok(c) => {
                // SAFETY: `out_json` is not null (checked above).
                unsafe { *out_json = c.into_raw() };
                QTTY_OK
            }
            Err(_) => QTTY_ERR_INVALID_VALUE,
        }
    })
}

/// Deserializes a JSON number string into a quantity with the given unit.
///
/// # Returns
///
/// * `QTTY_OK` on success.
/// * `QTTY_ERR_NULL_OUT` if either `json` or `out` is null.
/// * `QTTY_ERR_INVALID_VALUE` if `json` is not a valid JSON number.
///
/// # Safety
///
/// `json` must be a valid NUL-terminated C string.
/// `out` must point to a writable [`QttyQuantity`] location.
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

        // SAFETY: Caller guarantees `json` is a valid NUL-terminated string.
        let json_str = match unsafe { CStr::from_ptr(json) }.to_str() {
            Ok(s) => s,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };

        match serde_json::from_str::<f64>(json_str) {
            Ok(value) => {
                // SAFETY: `out` is not null (checked above).
                unsafe { *out = QttyQuantity::new(value, unit) };
                QTTY_OK
            }
            Err(_) => QTTY_ERR_INVALID_VALUE,
        }
    })
}

// =============================================================================
// Full-object JSON  {"value":<f64>, "unit_id":<u32>}
// =============================================================================

/// Serializes a quantity as a JSON object `{"value":<f64>,"unit_id":<u32>}`.
///
/// On success, `*out_json` is set to a heap-allocated NUL-terminated C string
/// that the caller **must** release with [`qtty_string_free`].
///
/// # Returns
///
/// * `QTTY_OK` on success.
/// * `QTTY_ERR_NULL_OUT` if `out_json` is null.
/// * `QTTY_ERR_INVALID_VALUE` if serialization fails.
///
/// # Safety
///
/// `out_json` must point to a writable `*mut c_char` location.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_to_json(
    src: QttyQuantity,
    out_json: *mut *mut c_char,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if out_json.is_null() {
            return QTTY_ERR_NULL_OUT;
        }

        let obj = serde_json::json!({
            "value": src.value,
            "unit_id": src.unit as u32,
        });

        let s = match serde_json::to_string(&obj) {
            Ok(s) => s,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };

        match CString::new(s) {
            Ok(c) => {
                // SAFETY: `out_json` is not null (checked above).
                unsafe { *out_json = c.into_raw() };
                QTTY_OK
            }
            Err(_) => QTTY_ERR_INVALID_VALUE,
        }
    })
}

/// Deserializes a JSON object `{"value":<f64>,"unit_id":<u32>}` into a quantity.
///
/// # Returns
///
/// * `QTTY_OK` on success.
/// * `QTTY_ERR_NULL_OUT` if either `json` or `out` is null.
/// * `QTTY_ERR_INVALID_VALUE` if `json` is not a valid JSON object with the
///   expected fields.
/// * `QTTY_ERR_UNKNOWN_UNIT` if the `unit_id` field does not map to a known unit.
///
/// # Safety
///
/// `json` must be a valid NUL-terminated C string.
/// `out` must point to a writable [`QttyQuantity`] location.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_from_json(
    json: *const c_char,
    out: *mut QttyQuantity,
) -> i32 {
    catch_panic!(QTTY_ERR_UNKNOWN_UNIT, {
        if json.is_null() || out.is_null() {
            return QTTY_ERR_NULL_OUT;
        }

        // SAFETY: Caller guarantees `json` is a valid NUL-terminated string.
        let json_str = match unsafe { CStr::from_ptr(json) }.to_str() {
            Ok(s) => s,
            Err(_) => return QTTY_ERR_INVALID_VALUE,
        };

        #[derive(serde::Deserialize)]
        struct QtyJson {
            value: f64,
            unit_id: u32,
        }

        match serde_json::from_str::<QtyJson>(json_str) {
            Ok(q) => match UnitId::from_u32(q.unit_id) {
                Some(unit) => {
                    // SAFETY: `out` is not null (checked above).
                    unsafe { *out = QttyQuantity::new(q.value, unit) };
                    QTTY_OK
                }
                None => QTTY_ERR_UNKNOWN_UNIT,
            },
            Err(_) => QTTY_ERR_INVALID_VALUE,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_value_roundtrip() {
        let src = QttyQuantity::new(123.456, UnitId::Meter);
        let mut ptr: *mut c_char = std::ptr::null_mut();

        let status = unsafe { qtty_quantity_to_json_value(src, &mut ptr) };
        assert_eq!(status, QTTY_OK);
        assert!(!ptr.is_null());

        let mut out = QttyQuantity::default();
        let status2 = unsafe { qtty_quantity_from_json_value(UnitId::Meter, ptr, &mut out) };
        unsafe { qtty_string_free(ptr) };

        assert_eq!(status2, QTTY_OK);
        assert!((out.value - 123.456).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Meter);
    }

    #[test]
    fn test_to_json_roundtrip() {
        let src = QttyQuantity::new(1.5, UnitId::Kilometer);
        let mut ptr: *mut c_char = std::ptr::null_mut();

        let status = unsafe { qtty_quantity_to_json(src, &mut ptr) };
        assert_eq!(status, QTTY_OK);
        assert!(!ptr.is_null());

        let mut out = QttyQuantity::default();
        let status2 = unsafe { qtty_quantity_from_json(ptr, &mut out) };
        unsafe { qtty_string_free(ptr) };

        assert_eq!(status2, QTTY_OK);
        assert!((out.value - 1.5).abs() < 1e-12);
        assert_eq!(out.unit, UnitId::Kilometer);
    }

    #[test]
    fn test_from_json_value_invalid_input() {
        let bad_json = std::ffi::CString::new("not a number").unwrap();
        let mut out = QttyQuantity::default();

        let status =
            unsafe { qtty_quantity_from_json_value(UnitId::Meter, bad_json.as_ptr(), &mut out) };
        assert_eq!(status, QTTY_ERR_INVALID_VALUE);
    }

    #[test]
    fn test_string_free_null_is_noop() {
        // Must not crash
        unsafe { qtty_string_free(std::ptr::null_mut()) };
    }
}
