use super::shared::{catch_panic, decode_unit, out_ptr};
use crate::registry;
use crate::types::{QttyQuantity, QttyStatus, UnitId, QTTY_FMT_LOWER_EXP, QTTY_FMT_UPPER_EXP};
use core::ffi::c_char;

fn map_convert_error(err: QttyStatus) -> QttyStatus {
    match err {
        QttyStatus::UnknownUnit => QttyStatus::UnknownUnit,
        QttyStatus::IncompatibleDim => QttyStatus::IncompatibleDim,
        _ => QttyStatus::UnknownUnit,
    }
}

fn format_quantity(qty: QttyQuantity, unit: UnitId, precision: i32, flags: u32) -> String {
    let symbol = unit.symbol();

    match flags {
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
        _ => {
            if precision >= 0 {
                format!("{:.prec$} {}", qty.value, symbol, prec = precision as usize)
            } else {
                format!("{} {}", qty.value, symbol)
            }
        }
    }
}

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
        let mut out = match out_ptr(out) {
            Ok(out) => out,
            Err(err) => return err,
        };
        let unit = match decode_unit(unit_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };

        unsafe { *out.as_mut() = QttyQuantity::new(value, unit) };
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
        let mut out = match out_ptr(out) {
            Ok(out) => out,
            Err(err) => return err,
        };
        let src_unit = match decode_unit(src.unit) {
            Ok(unit) => unit,
            Err(err) => return err,
        };
        let dst_unit = match decode_unit(dst_unit_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };

        match registry::convert_value(src.value, src_unit, dst_unit) {
            Ok(value) => {
                unsafe { *out.as_mut() = QttyQuantity::new(value, dst_unit) };
                QttyStatus::Ok
            }
            Err(err) => map_convert_error(err),
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
        let mut out_value = match out_ptr(out_value) {
            Ok(out) => out,
            Err(err) => return err,
        };
        let src_unit = match decode_unit(src_unit_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };
        let dst_unit = match decode_unit(dst_unit_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };

        match registry::convert_value(value, src_unit, dst_unit) {
            Ok(converted) => {
                unsafe { *out_value.as_mut() = converted };
                QttyStatus::Ok
            }
            Err(err) => map_convert_error(err),
        }
    })
}

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
        if buf_len == 0 {
            return QttyStatus::NullOut;
        }
        let buf = match out_ptr(buf) {
            Ok(buf) => buf,
            Err(err) => return err,
        };
        let unit = match decode_unit(qty.unit) {
            Ok(u) => u,
            Err(err) => return err,
        };
        if registry::meta(unit).is_none() {
            return QttyStatus::UnknownUnit;
        }

        let formatted = format_quantity(qty, unit, precision, flags);
        let bytes = formatted.as_bytes();
        let needed = bytes.len() + 1;
        if buf_len < needed {
            return QttyStatus::BufferTooSmall;
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                bytes.as_ptr() as *const c_char,
                buf.as_ptr(),
                bytes.len(),
            );
            *buf.as_ptr().add(bytes.len()) = 0;
        }
        QttyStatus::Ok
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::UnitId;
    use crate::types::QTTY_FMT_DEFAULT;
    use approx::assert_relative_eq;
    use core::f64::consts::PI;

    #[test]
    fn test_quantity_make_valid() {
        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_make(42.0, UnitId::Meter as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert_eq!(out.value, 42.0);
        assert_eq!(out.unit, UnitId::Meter as u32);
    }

    #[test]
    fn test_quantity_make_invalid_unit() {
        let mut out = QttyQuantity::default();
        let status = unsafe { qtty_quantity_make(1.0, 0, &mut out) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }

    #[test]
    fn test_quantity_make_null_out() {
        let status =
            unsafe { qtty_quantity_make(1.0, UnitId::Meter as u32, core::ptr::null_mut()) };
        assert_eq!(status, QttyStatus::NullOut);
    }

    #[test]
    fn test_known_conversion_meters_to_kilometers() {
        let mut out = QttyQuantity::default();
        let src = QttyQuantity::new(1000.0, UnitId::Meter);
        let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, &mut out) };
        assert_eq!(status, QttyStatus::Ok);
        assert_relative_eq!(out.value, 1.0, epsilon = 1e-12);
        assert_eq!(out.unit, UnitId::Kilometer as u32);
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

    #[test]
    fn test_format_default_notation() {
        let qty = QttyQuantity::new(1234.5, UnitId::Meter);
        let mut buf = [0u8; 64];
        let status = unsafe {
            qtty_quantity_format(
                qty,
                2,
                QTTY_FMT_DEFAULT,
                buf.as_mut_ptr() as *mut c_char,
                64,
            )
        };
        assert_eq!(status, QttyStatus::Ok);
    }

    #[test]
    fn test_format_buffer_too_small() {
        let qty = QttyQuantity::new(1234.5, UnitId::Meter);
        let mut buf = [0u8; 2];
        let status = unsafe {
            qtty_quantity_format(qty, 2, QTTY_FMT_DEFAULT, buf.as_mut_ptr() as *mut c_char, 2)
        };
        assert_eq!(status, QttyStatus::BufferTooSmall);
    }
}
