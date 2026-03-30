//! Extern "C" API for FFI consumers.
//!
//! This module exposes `#[no_mangle] pub extern "C"` functions that form the stable C ABI
//! for `qtty-ffi`. These functions can be called from C/C++ code or any language with C FFI support.
//!
//! # Safety
//!
//! All functions in this module:
//! - Never panic across FFI boundaries (all panics are caught and converted to `InternalPanic`)
//! - Validate all input pointers before use
//! - Accept unit and dimension IDs as raw `uint32_t` values and validate them before dispatch
//! - Return [`QttyStatus`] to indicate success or failure
//!
//! # ABI conventions (vNext)
//!
//! - Every fallible exported function returns `QttyStatus` and writes results via out-parameters.
//! - Unit and dimension inputs are raw `u32` IDs validated before dispatch; named constants
//!   in the generated header provide the canonical values for C callers.
//! - `InternalPanic` is reserved exclusively for caught Rust panics and is never used
//!   for domain errors like `UnknownUnit`.

use crate::registry;
use crate::types::{
    DimensionId, QttyDerivedQuantity, QttyQuantity, QttyStatus, UnitId, QTTY_ERR_INCOMPATIBLE_DIM,
    QTTY_ERR_UNKNOWN_UNIT, QTTY_FMT_LOWER_EXP, QTTY_FMT_UPPER_EXP,
};
use core::ffi::c_char;
use std::ffi::{CStr, CString};

// =============================================================================
// Helper macros to catch panics
// =============================================================================

/// Catches any panic and returns `InternalPanic` instead of unwinding across FFI.
/// Use this for functions that return `QttyStatus`.
macro_rules! catch_panic {
    ($body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(result) => result,
            Err(_) => QttyStatus::InternalPanic,
        }
    }};
}

/// Catches any panic and returns the given fallback for non-QttyStatus return types.
macro_rules! catch_panic_or {
    ($fallback:expr, $body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(result) => result,
            Err(_) => $fallback,
        }
    }};
}

// =============================================================================
// Internal helpers: validated ID → typed enum
// =============================================================================

/// Decode a raw `u32` to `UnitId`, returning `UnknownUnit` if invalid.
#[inline]
fn decode_unit(raw: u32) -> Result<UnitId, QttyStatus> {
    UnitId::from_u32(raw).ok_or(QttyStatus::UnknownUnit)
}

// =============================================================================
// Unit Validation / Info Functions
// =============================================================================

/// Checks if a raw unit ID is valid (recognized by the registry).
///
/// # Arguments
///
/// * `unit_id` - Raw `uint32_t` unit identifier
///
/// # Returns
///
/// `true` if the unit is valid, `false` otherwise.
#[no_mangle]
pub extern "C" fn qtty_unit_is_valid(unit_id: u32) -> bool {
    catch_panic_or!(false, {
        UnitId::from_u32(unit_id).and_then(registry::meta).is_some()
    })
}

/// Gets the dimension of a unit.
///
/// # Arguments
///
/// * `unit_id` - Raw `uint32_t` unit identifier
/// * `out`     - Pointer to store the [`DimensionId`]
///
/// # Returns
///
/// * [`QttyStatus::Ok`] on success
/// * [`QttyStatus::NullOut`] if `out` is null
/// * [`QttyStatus::UnknownUnit`] if the unit ID is not recognized
///
/// # Safety
///
/// `out` must be a valid, writable pointer to `DimensionId`, or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_unit_dimension(unit_id: u32, out: *mut DimensionId) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let unit = match decode_unit(unit_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        match registry::dimension(unit) {
            Some(dim) => {
                unsafe { *out = dim };
                QttyStatus::Ok
            }
            None => QttyStatus::UnknownUnit,
        }
    })
}

/// Checks if two units are compatible (same dimension).
///
/// # Arguments
///
/// * `a_id` - Raw `uint32_t` unit identifier for the first unit
/// * `b_id` - Raw `uint32_t` unit identifier for the second unit
/// * `out`  - Pointer to store the result (`bool`)
///
/// # Returns
///
/// * [`QttyStatus::Ok`] on success
/// * [`QttyStatus::NullOut`] if `out` is null
/// * [`QttyStatus::UnknownUnit`] if either unit ID is not recognized
///
/// # Safety
///
/// `out` must be a valid, writable pointer to `bool`, or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_units_compatible(
    a_id: u32,
    b_id: u32,
    out: *mut bool,
) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let a = match decode_unit(a_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        let b = match decode_unit(b_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        unsafe { *out = registry::compatible(a, b) };
        QttyStatus::Ok
    })
}

// =============================================================================
// Quantity Construction and Conversion Functions
// =============================================================================

/// Creates a new quantity with the given value and unit.
///
/// # Arguments
///
/// * `value`   - The numeric value
/// * `unit_id` - Raw `uint32_t` unit identifier
/// * `out`     - Pointer to store the resulting [`QttyQuantity`]
///
/// # Returns
///
/// * [`QttyStatus::Ok`] on success
/// * [`QttyStatus::NullOut`] if `out` is null
/// * [`QttyStatus::UnknownUnit`] if the unit ID is not recognized
///
/// # Safety
///
/// `out` must be a valid, writable pointer to [`QttyQuantity`], or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_make(
    value: f64,
    unit_id: u32,
    out: *mut QttyQuantity,
) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let unit = match decode_unit(unit_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        unsafe { *out = QttyQuantity::new(value, unit) };
        QttyStatus::Ok
    })
}

/// Converts a quantity to a different unit.
///
/// # Arguments
///
/// * `src`         - The source quantity
/// * `dst_unit_id` - Raw `uint32_t` target unit identifier
/// * `out`         - Pointer to store the converted [`QttyQuantity`]
///
/// # Returns
///
/// * [`QttyStatus::Ok`] on success
/// * [`QttyStatus::NullOut`] if `out` is null
/// * [`QttyStatus::UnknownUnit`] if either unit ID is not recognized
/// * [`QttyStatus::IncompatibleDim`] if units have different dimensions
///
/// # Safety
///
/// `out` must be a valid, writable pointer to [`QttyQuantity`], or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_convert(
    src: QttyQuantity,
    dst_unit_id: u32,
    out: *mut QttyQuantity,
) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let dst_unit = match decode_unit(dst_unit_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        match registry::convert_value(src.value, src.unit, dst_unit) {
            Ok(v) => {
                unsafe { *out = QttyQuantity::new(v, dst_unit) };
                QttyStatus::Ok
            }
            Err(QTTY_ERR_UNKNOWN_UNIT) => QttyStatus::UnknownUnit,
            Err(QTTY_ERR_INCOMPATIBLE_DIM) => QttyStatus::IncompatibleDim,
            Err(_) => QttyStatus::UnknownUnit,
        }
    })
}

/// Converts a raw value from one unit to another.
///
/// # Arguments
///
/// * `value`       - The numeric value to convert
/// * `src_unit_id` - Raw `uint32_t` source unit identifier
/// * `dst_unit_id` - Raw `uint32_t` target unit identifier
/// * `out_value`   - Pointer to store the converted `f64`
///
/// # Returns
///
/// * [`QttyStatus::Ok`] on success
/// * [`QttyStatus::NullOut`] if `out_value` is null
/// * [`QttyStatus::UnknownUnit`] if either unit ID is not recognized
/// * [`QttyStatus::IncompatibleDim`] if units have different dimensions
///
/// # Safety
///
/// `out_value` must be a valid, writable pointer to `f64`, or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_convert_value(
    value: f64,
    src_unit_id: u32,
    dst_unit_id: u32,
    out_value: *mut f64,
) -> QttyStatus {
    catch_panic!({
        if out_value.is_null() {
            return QttyStatus::NullOut;
        }
        let src_unit = match decode_unit(src_unit_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        let dst_unit = match decode_unit(dst_unit_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        match registry::convert_value(value, src_unit, dst_unit) {
            Ok(converted) => {
                unsafe { *out_value = converted };
                QttyStatus::Ok
            }
            Err(QTTY_ERR_UNKNOWN_UNIT) => QttyStatus::UnknownUnit,
            Err(QTTY_ERR_INCOMPATIBLE_DIM) => QttyStatus::IncompatibleDim,
            Err(_) => QttyStatus::UnknownUnit,
        }
    })
}

/// Gets the name of a unit as a NUL-terminated C string.
///
/// # Arguments
///
/// * `unit_id` - Raw `uint32_t` unit identifier
///
/// # Returns
///
/// A pointer to a static, NUL-terminated C string, or null if the unit ID is
/// not recognized.  The pointer points to static memory; the caller must not
/// free or modify it.
#[no_mangle]
pub extern "C" fn qtty_unit_name(unit_id: u32) -> *const c_char {
    catch_panic_or!(core::ptr::null(), {
        match UnitId::from_u32(unit_id) {
            Some(u) if registry::meta(u).is_some() => u.name_cstr(),
            _ => core::ptr::null(),
        }
    })
}

// =============================================================================
// String Formatting
// =============================================================================

/// Formats a quantity as a human-readable string into a caller-provided buffer.
///
/// Produces strings like `"1234.57 m"`, `"1.23e3 km"`, or `"1.23E3 km"`.
///
/// # Arguments
///
/// * `qty`       - The quantity to format.
/// * `precision` - Decimal digits after the point; `-1` for default (shortest).
/// * `flags`     - `QTTY_FMT_DEFAULT` (0), `QTTY_FMT_LOWER_EXP` (1), or
///   `QTTY_FMT_UPPER_EXP` (2).
/// * `buf`       - Caller-allocated output buffer (non-null).
/// * `buf_len`   - Size of `buf` in bytes (must include space for NUL).
///
/// # Returns
///
/// * `QttyStatus::Ok` and the buffer is filled on success.
/// * `QttyStatus::NullOut` if `buf` is null.
/// * `QttyStatus::UnknownUnit` if the quantity's unit is not recognized.
/// * `QttyStatus::BufferTooSmall` if `buf_len` is insufficient.
///
/// # Safety
///
/// `buf` must point to a writable allocation of at least `buf_len` bytes.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_format(
    qty: QttyQuantity,
    precision: i32,
    flags: u32,
    buf: *mut c_char,
    buf_len: usize,
) -> QttyStatus {
    catch_panic!({
        if buf.is_null() || buf_len == 0 {
            return QttyStatus::NullOut;
        }
        if crate::registry::meta(qty.unit).is_none() {
            return QttyStatus::UnknownUnit;
        }

        let symbol = qty.unit.symbol();
        let formatted = match flags {
            QTTY_FMT_LOWER_EXP => {
                if precision >= 0 {
                    format!("{:.prec$e} {}", qty.value, symbol, prec = precision as usize)
                } else {
                    format!("{:e} {}", qty.value, symbol)
                }
            }
            QTTY_FMT_UPPER_EXP => {
                if precision >= 0 {
                    format!("{:.prec$E} {}", qty.value, symbol, prec = precision as usize)
                } else {
                    format!("{:E} {}", qty.value, symbol)
                }
            }
            _ => {
                if precision >= 0 {
                    format!("{:.prec$} {}", qty.value, symbol, prec = precision as usize)
                } else {
                    format!("{} {}", qty.value, symbol)
                }
            }
        };

        let bytes = formatted.as_bytes();
        let needed = bytes.len() + 1;
        if buf_len < needed {
            return QttyStatus::BufferTooSmall;
        }

        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, buf, bytes.len());
            *buf.add(bytes.len()) = 0;
        }
        QttyStatus::Ok
    })
}

// =============================================================================
// Heap string management
// =============================================================================

/// Frees a string previously allocated by one of the `qtty_*_to_json*` functions.
///
/// Passing a null pointer is safe (no-op).
///
/// # Safety
///
/// The pointer must have been returned by a `qtty_*_to_json*` function and must
/// not have been freed previously.
#[no_mangle]
pub unsafe extern "C" fn qtty_string_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe { let _ = CString::from_raw(s); }
}

// =============================================================================
// JSON Serialization / Deserialization
// =============================================================================

/// Serializes a quantity's value as a plain JSON number string (e.g. `"123.45"`).
///
/// The returned string must be freed with [`qtty_string_free`].
///
/// # Safety
///
/// `out` must be a valid, writable pointer to `*mut c_char`, or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_to_json_value(
    src: QttyQuantity,
    out: *mut *mut c_char,
) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let s = serde_json::to_string(&src.value).unwrap_or_default();
        let c = CString::new(s).unwrap_or_default();
        unsafe { *out = c.into_raw() };
        QttyStatus::Ok
    })
}

/// Deserializes a quantity from a plain JSON numeric string with an explicit unit ID.
///
/// # Safety
///
/// `json` must be a valid NUL-terminated C string; `out` must be a valid,
/// writable pointer to [`QttyQuantity`].
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_from_json_value(
    unit_id: u32,
    json: *const c_char,
    out: *mut QttyQuantity,
) -> QttyStatus {
    catch_panic!({
        if json.is_null() || out.is_null() {
            return QttyStatus::NullOut;
        }
        let unit = match decode_unit(unit_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        let cstr = unsafe { CStr::from_ptr(json) };
        let s = match cstr.to_str() {
            Ok(v) => v,
            Err(_) => return QttyStatus::InvalidValue,
        };
        let v: f64 = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(_) => return QttyStatus::InvalidValue,
        };
        unsafe { *out = QttyQuantity::new(v, unit) };
        QttyStatus::Ok
    })
}

/// Serializes a quantity to a full JSON object: `{"value":123.45,"unit":"Meter"}`.
///
/// The returned string must be freed with [`qtty_string_free`].
///
/// # Safety
///
/// `out` must be a valid, writable pointer to `*mut c_char`, or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_to_json(
    src: QttyQuantity,
    out: *mut *mut c_char,
) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let s = match serde_json::to_string(&src) {
            Ok(s) => s,
            Err(_) => return QttyStatus::InvalidValue,
        };
        let c = CString::new(s).unwrap_or_default();
        unsafe { *out = c.into_raw() };
        QttyStatus::Ok
    })
}

/// Deserializes a quantity from a JSON object: `{"value":123.45,"unit":"Meter"}`.
///
/// # Safety
///
/// `json` must be a valid NUL-terminated C string; `out` must be a valid,
/// writable pointer to [`QttyQuantity`].
#[no_mangle]
pub unsafe extern "C" fn qtty_quantity_from_json(
    json: *const c_char,
    out: *mut QttyQuantity,
) -> QttyStatus {
    catch_panic!({
        if json.is_null() || out.is_null() {
            return QttyStatus::NullOut;
        }
        let cstr = unsafe { CStr::from_ptr(json) };
        let s = match cstr.to_str() {
            Ok(v) => v,
            Err(_) => return QttyStatus::InvalidValue,
        };
        let qty: QttyQuantity = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(_) => return QttyStatus::InvalidValue,
        };
        if registry::meta(qty.unit).is_none() {
            return QttyStatus::UnknownUnit;
        }
        unsafe { *out = qty };
        QttyStatus::Ok
    })
}

// =============================================================================
// Derived Quantity (Compound Unit) Functions
// =============================================================================

/// Creates a new derived quantity (compound unit like m/s).
///
/// # Arguments
///
/// * `value`         - The numeric value
/// * `numerator_id`  - Raw `uint32_t` numerator unit identifier
/// * `denominator_id`- Raw `uint32_t` denominator unit identifier
/// * `out`           - Pointer to store the resulting [`QttyDerivedQuantity`]
///
/// # Safety
///
/// `out` must be a valid, writable pointer to [`QttyDerivedQuantity`], or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_derived_make(
    value: f64,
    numerator_id: u32,
    denominator_id: u32,
    out: *mut QttyDerivedQuantity,
) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let num = match decode_unit(numerator_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        let den = match decode_unit(denominator_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        unsafe { *out = QttyDerivedQuantity::new(value, num, den) };
        QttyStatus::Ok
    })
}

/// Converts a derived quantity to different units.
///
/// # Safety
///
/// `out` must be a valid, writable pointer to [`QttyDerivedQuantity`], or null.
#[no_mangle]
pub unsafe extern "C" fn qtty_derived_convert(
    src: QttyDerivedQuantity,
    target_num_id: u32,
    target_den_id: u32,
    out: *mut QttyDerivedQuantity,
) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let target_num = match decode_unit(target_num_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        let target_den = match decode_unit(target_den_id) {
            Ok(u) => u,
            Err(e) => return e,
        };
        match src.convert_to(target_num, target_den) {
            Some(converted) => {
                unsafe { *out = converted };
                QttyStatus::Ok
            }
            None => {
                if registry::meta(src.numerator).is_none()
                    || registry::meta(src.denominator).is_none()
                {
                    QttyStatus::UnknownUnit
                } else {
                    QttyStatus::IncompatibleDim
                }
            }
        }
    })
}

/// Serializes a derived quantity to a JSON object.
///
/// The returned string must be freed with [`qtty_string_free`].
///
/// # Safety
///
/// `out` must be a valid, writable pointer to `*mut c_char`.
#[no_mangle]
pub unsafe extern "C" fn qtty_derived_to_json(
    src: QttyDerivedQuantity,
    out: *mut *mut c_char,
) -> QttyStatus {
    catch_panic!({
        if out.is_null() {
            return QttyStatus::NullOut;
        }
        let s = match serde_json::to_string(&src) {
            Ok(s) => s,
            Err(_) => return QttyStatus::InvalidValue,
        };
        let c = CString::new(s).unwrap_or_default();
        unsafe { *out = c.into_raw() };
        QttyStatus::Ok
    })
}

/// Deserializes a derived quantity from a JSON object.
///
/// # Safety
///
/// `json` must be a valid NUL-terminated C string; `out` must be a valid,
/// writable pointer to [`QttyDerivedQuantity`].
#[no_mangle]
pub unsafe extern "C" fn qtty_derived_from_json(
    json: *const c_char,
    out: *mut QttyDerivedQuantity,
) -> QttyStatus {
    catch_panic!({
        if json.is_null() || out.is_null() {
            return QttyStatus::NullOut;
        }
        let cstr = unsafe { CStr::from_ptr(json) };
        let s = match cstr.to_str() {
            Ok(v) => v,
            Err(_) => return QttyStatus::InvalidValue,
        };
        let qty: QttyDerivedQuantity = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(_) => return QttyStatus::InvalidValue,
        };
        if registry::meta(qty.numerator).is_none() || registry::meta(qty.denominator).is_none() {
            return QttyStatus::UnknownUnit;
        }
        unsafe { *out = qty };
        QttyStatus::Ok
    })
}

// =============================================================================
// Version Info
// =============================================================================

/// Returns the FFI ABI version (major*10000 + minor*100 + patch).
///
/// Current version: 0.5.0 → 500
#[allow(clippy::erasing_op, clippy::identity_op)]
#[no_mangle]
pub extern "C" fn qtty_ffi_version() -> u32 {
    0 * 10000 + 5 * 100 + 0 // 0.5.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::QTTY_FMT_DEFAULT;
    use approx::assert_relative_eq;
    use core::f64::consts::PI;

    // ── Unit validation ───────────────────────────────────────────────────────

    #[test]
    fn test_unit_is_valid() {
        assert!(qtty_unit_is_valid(UnitId::Meter as u32));
        assert!(qtty_unit_is_valid(UnitId::Second as u32));
        assert!(qtty_unit_is_valid(UnitId::Radian as u32));
    }

    #[test]
    fn test_unit_is_valid_rejects_unknown_id() {
        assert!(!qtty_unit_is_valid(0));
        assert!(!qtty_unit_is_valid(9999));
        assert!(!qtty_unit_is_valid(99999));
        assert!(!qtty_unit_is_valid(u32::MAX));
    }

    #[test]
    fn test_unit_dimension() {
        let mut dim = DimensionId::Length;
        let status = unsafe { qtty_unit_dimension(UnitId::Meter as u32, &mut dim) };
        assert_eq!(status, QttyStatus::Ok);
        assert_eq!(dim, DimensionId::Length);

        let status2 = unsafe { qtty_unit_dimension(UnitId::Second as u32, &mut dim) };
        assert_eq!(status2, QttyStatus::Ok);
        assert_eq!(dim, DimensionId::Time);
    }

    #[test]
    fn test_unit_dimension_rejects_invalid_id() {
        let mut dim = DimensionId::Length;
        let status = unsafe { qtty_unit_dimension(0, &mut dim) };
        assert_eq!(status, QttyStatus::UnknownUnit);

        let status2 = unsafe { qtty_unit_dimension(9999, &mut dim) };
        assert_eq!(status2, QttyStatus::UnknownUnit);
    }

    #[test]
    fn test_unit_dimension_null_out() {
        let status = unsafe { qtty_unit_dimension(UnitId::Meter as u32, core::ptr::null_mut()) };
        assert_eq!(status, QttyStatus::NullOut);
    }

    #[test]
    fn test_units_compatible() {
        let mut compat = false;
        let status = unsafe {
            qtty_units_compatible(UnitId::Meter as u32, UnitId::Kilometer as u32, &mut compat)
        };
        assert_eq!(status, QttyStatus::Ok);
        assert!(compat);

        let status2 = unsafe {
            qtty_units_compatible(UnitId::Meter as u32, UnitId::Second as u32, &mut compat)
        };
        assert_eq!(status2, QttyStatus::Ok);
        assert!(!compat);
    }

    #[test]
    fn test_units_compatible_rejects_invalid_id() {
        let mut compat = false;
        let status = unsafe { qtty_units_compatible(0, UnitId::Meter as u32, &mut compat) };
        assert_eq!(status, QttyStatus::UnknownUnit);

        let status2 = unsafe { qtty_units_compatible(UnitId::Meter as u32, 9999, &mut compat) };
        assert_eq!(status2, QttyStatus::UnknownUnit);
    }

    // ── Quantity make / convert ───────────────────────────────────────────────

    #[test]
    fn test_quantity_make_valid() {
        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_make(42.0, UnitId::Meter as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert_eq!(out.value, 42.0);
        assert_eq!(out.unit, UnitId::Meter);
    }

    #[test]
    fn test_quantity_make_invalid_unit() {
        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_make(1.0, 0, &mut out) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }

    #[test]
    fn test_quantity_make_null_out() {
        let status = unsafe { qtty_quantity_make(1.0, UnitId::Meter as u32, core::ptr::null_mut()) };
        assert_eq!(status, QttyStatus::NullOut);
    }

    #[test]
    fn test_known_conversion_meters_to_kilometers() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(1000.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert_relative_eq!(out.value, 1.0, epsilon = 1e-12);
        assert_eq!(out.unit, UnitId::Kilometer);
    }

    #[test]
    fn test_known_conversion_seconds_to_hours() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(3600.0, UnitId::Second);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Hour as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert_relative_eq!(out.value, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_known_conversion_degrees_to_radians() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(180.0, UnitId::Degree);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Radian as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert_relative_eq!(out.value, PI, epsilon = 1e-12);
    }

    #[test]
    fn test_incompatible_conversion_fails() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(100.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Second as u32, &mut out) };
        assert_eq!(status, QttyStatus::IncompatibleDim);
    }

    #[test]
    fn test_convert_invalid_dst_unit() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(100.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_convert(src, 0, &mut out) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }

    #[test]
    fn test_convert_null_out() {
        let src = QttyQuantity::new(100.0, UnitId::Meter);
        let status =
            unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, core::ptr::null_mut()) };
        assert_eq!(status, QttyStatus::NullOut);
    }

    #[test]
    fn test_convert_value_invalid_src_unit() {
        let mut out = 0.0f64;
        let status =
            unsafe { qtty_quantity_convert_value(1.0, 9999, UnitId::Meter as u32, &mut out) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }

    #[test]
    fn test_convert_value_invalid_dst_unit() {
        let mut out = 0.0f64;
        let status =
            unsafe { qtty_quantity_convert_value(1.0, UnitId::Meter as u32, 9999, &mut out) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }

    // ── Format ───────────────────────────────────────────────────────────────

    #[test]
    fn test_format_default_notation() {
        let qty = QttyQuantity::new(1234.5, UnitId::Meter);
        let mut buf = [0u8; 64];
        let status = unsafe {
            qtty_quantity_format(qty, 2, QTTY_FMT_DEFAULT, buf.as_mut_ptr() as *mut c_char, 64)
        };
        assert_eq!(status, QttyStatus::Ok);
    }

    #[test]
    fn test_format_buffer_too_small() {
        let qty = QttyQuantity::new(1234.5, UnitId::Meter);
        let mut buf = [0u8; 2]; // definitely too small
        let status = unsafe {
            qtty_quantity_format(qty, 2, QTTY_FMT_DEFAULT, buf.as_mut_ptr() as *mut c_char, 2)
        };
        assert_eq!(status, QttyStatus::BufferTooSmall);
    }

    // ── Derived quantity ──────────────────────────────────────────────────────

    #[test]
    fn test_derived_make_valid() {
        let mut out = QttyDerivedQuantity::default();
        let status = unsafe {
            qtty_derived_make(100.0, UnitId::Meter as u32, UnitId::Second as u32, &mut out)
        };
        assert_eq!(status, QttyStatus::Ok);
        assert_eq!(out.value, 100.0);
    }

    #[test]
    fn test_derived_make_invalid_numerator() {
        let mut out = QttyDerivedQuantity::default();
        let status = unsafe { qtty_derived_make(1.0, 0, UnitId::Second as u32, &mut out) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }

    #[test]
    fn test_derived_make_invalid_denominator() {
        let mut out = QttyDerivedQuantity::default();
        let status = unsafe { qtty_derived_make(1.0, UnitId::Meter as u32, 0, &mut out) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }

    #[test]
    fn test_derived_convert_incompatible_dim() {
        let mut out = QttyDerivedQuantity::default();
        let src = QttyDerivedQuantity::new(100.0, UnitId::Meter, UnitId::Second);
        // kg/h is incompatible with m/s
        let status = unsafe {
            qtty_derived_convert(
                src,
                UnitId::Kilogram as u32,
                UnitId::Hour as u32,
                &mut out,
            )
        };
        assert_eq!(status, QttyStatus::IncompatibleDim);
    }

    // ── String free ──────────────────────────────────────────────────────────

    #[test]
    fn test_string_free_null_is_safe() {
        unsafe { qtty_string_free(core::ptr::null_mut()) };
    }

    // ── Version ──────────────────────────────────────────────────────────────

    #[test]
    fn test_version() {
        // 0.5.0 → 500
        assert_eq!(qtty_ffi_version(), 500);
    }
}
