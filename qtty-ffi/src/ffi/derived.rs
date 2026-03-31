use super::shared::{catch_panic, decode_unit, out_ptr};
use crate::registry;
use crate::types::{QttyDerivedQuantity, QttyStatus};

/// Creates a new derived quantity (compound unit like m/s).
///
/// # Arguments
///
/// * `value`          - The numeric value
/// * `numerator_id`   - Raw `uint32_t` numerator unit identifier
/// * `denominator_id` - Raw `uint32_t` denominator unit identifier
/// * `out`            - Pointer to store the resulting [`QttyDerivedQuantity`]
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
        let mut out = match out_ptr(out) {
            Ok(out) => out,
            Err(err) => return err,
        };
        let numerator = match decode_unit(numerator_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };
        let denominator = match decode_unit(denominator_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };

        unsafe { *out.as_mut() = QttyDerivedQuantity::new(value, numerator, denominator) };
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
        let mut out = match out_ptr(out) {
            Ok(out) => out,
            Err(err) => return err,
        };
        let target_num = match decode_unit(target_num_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };
        let target_den = match decode_unit(target_den_id) {
            Ok(unit) => unit,
            Err(err) => return err,
        };

        match src.convert_to(target_num, target_den) {
            Some(converted) => {
                unsafe { *out.as_mut() = converted };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::UnitId;

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
        let status = unsafe {
            qtty_derived_convert(src, UnitId::Kilogram as u32, UnitId::Hour as u32, &mut out)
        };
        assert_eq!(status, QttyStatus::IncompatibleDim);
    }
}
