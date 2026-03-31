use super::shared::{catch_panic, catch_panic_or, decode_unit, out_ptr};
use crate::registry;
use crate::types::{DimensionId, QttyStatus, UnitId};
use core::ffi::c_char;

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
        let mut out = match out_ptr(out) {
            Ok(out) => out,
            Err(err) => return err,
        };
        let unit = match decode_unit(unit_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };

        match registry::dimension(unit) {
            Some(dim) => {
                unsafe { *out.as_mut() = dim };
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
pub unsafe extern "C" fn qtty_units_compatible(a_id: u32, b_id: u32, out: *mut bool) -> QttyStatus {
    catch_panic!({
        let mut out = match out_ptr(out) {
            Ok(out) => out,
            Err(err) => return err,
        };
        let a = match decode_unit(a_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };
        let b = match decode_unit(b_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };

        unsafe { *out.as_mut() = registry::compatible(a, b) };
        QttyStatus::Ok
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
/// not recognized. The pointer points to static memory; the caller must not
/// free or modify it.
#[no_mangle]
pub extern "C" fn qtty_unit_name(unit_id: u32) -> *const c_char {
    catch_panic_or!(core::ptr::null(), {
        match UnitId::from_u32(unit_id) {
            Some(unit) if registry::meta(unit).is_some() => unit.name_cstr(),
            _ => core::ptr::null(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let status = unsafe { qtty_unit_dimension(UnitId::Second as u32, &mut dim) };
        assert_eq!(status, QttyStatus::Ok);
        assert_eq!(dim, DimensionId::Time);
    }

    #[test]
    fn test_unit_dimension_rejects_invalid_id() {
        let mut dim = DimensionId::Length;
        let status = unsafe { qtty_unit_dimension(0, &mut dim) };
        assert_eq!(status, QttyStatus::UnknownUnit);

        let status = unsafe { qtty_unit_dimension(9999, &mut dim) };
        assert_eq!(status, QttyStatus::UnknownUnit);
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

        let status = unsafe {
            qtty_units_compatible(UnitId::Meter as u32, UnitId::Second as u32, &mut compat)
        };
        assert_eq!(status, QttyStatus::Ok);
        assert!(!compat);
    }

    #[test]
    fn test_units_compatible_rejects_invalid_id() {
        let mut compat = false;
        let status = unsafe { qtty_units_compatible(0, UnitId::Meter as u32, &mut compat) };
        assert_eq!(status, QttyStatus::UnknownUnit);

        let status = unsafe { qtty_units_compatible(UnitId::Meter as u32, 9999, &mut compat) };
        assert_eq!(status, QttyStatus::UnknownUnit);
    }
}
