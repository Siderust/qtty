//! Integration tests for qtty-ffi.
//!
//! These tests verify the FFI functions work correctly from a consumer's perspective.

use approx::assert_relative_eq;
use core::f64::consts::PI;
use qtty_ffi::{
    qtty_derived_convert, qtty_derived_from_json, qtty_derived_make, qtty_derived_to_json,
    qtty_ffi_version, qtty_quantity_convert, qtty_quantity_convert_value, qtty_quantity_format,
    qtty_quantity_from_json, qtty_quantity_from_json_value, qtty_quantity_make,
    qtty_quantity_to_json, qtty_quantity_to_json_value, qtty_string_free, qtty_unit_dimension,
    qtty_unit_is_valid, qtty_unit_name, qtty_units_compatible, DimensionId, QttyDerivedQuantity,
    QttyQuantity, UnitId, QTTY_ERR_BUFFER_TOO_SMALL, QTTY_ERR_INCOMPATIBLE_DIM,
    QTTY_ERR_INVALID_VALUE, QTTY_ERR_NULL_OUT, QTTY_FMT_LOWER_EXP, QTTY_FMT_UPPER_EXP, QTTY_OK,
};
use std::ffi::CStr;

// =============================================================================
// Unit Validation Tests
// =============================================================================

#[test]
fn test_all_units_are_valid() {
    let units = [
        UnitId::Meter,
        UnitId::Kilometer,
        UnitId::Second,
        UnitId::Minute,
        UnitId::Hour,
        UnitId::Day,
        UnitId::Radian,
        UnitId::Degree,
    ];

    for unit in units {
        assert!(qtty_unit_is_valid(unit), "Unit {:?} should be valid", unit);
    }
}

#[test]
fn test_unit_dimensions_are_correct() {
    let test_cases = [
        (UnitId::Meter, DimensionId::Length),
        (UnitId::Kilometer, DimensionId::Length),
        (UnitId::Second, DimensionId::Time),
        (UnitId::Minute, DimensionId::Time),
        (UnitId::Hour, DimensionId::Time),
        (UnitId::Day, DimensionId::Time),
        (UnitId::Radian, DimensionId::Angle),
        (UnitId::Degree, DimensionId::Angle),
    ];

    for (unit, expected_dim) in test_cases {
        let mut dim = DimensionId::Length;
        let status = unsafe { qtty_unit_dimension(unit, &mut dim) };
        assert_eq!(status, QTTY_OK, "Getting dimension for {:?} failed", unit);
        assert_eq!(dim, expected_dim, "Dimension mismatch for {:?}", unit);
    }
}

#[test]
fn test_compatible_units() {
    let compatible_pairs = [
        (UnitId::Meter, UnitId::Kilometer),
        (UnitId::Second, UnitId::Minute),
        (UnitId::Second, UnitId::Hour),
        (UnitId::Second, UnitId::Day),
        (UnitId::Minute, UnitId::Hour),
        (UnitId::Radian, UnitId::Degree),
    ];

    for (a, b) in compatible_pairs {
        let mut result = false;
        let status = unsafe { qtty_units_compatible(a, b, &mut result) };
        assert_eq!(status, QTTY_OK);
        assert!(result, "{:?} and {:?} should be compatible", a, b);
    }
}

#[test]
fn test_incompatible_units() {
    let incompatible_pairs = [
        (UnitId::Meter, UnitId::Second),
        (UnitId::Meter, UnitId::Radian),
        (UnitId::Second, UnitId::Degree),
        (UnitId::Hour, UnitId::Kilometer),
    ];

    for (a, b) in incompatible_pairs {
        let mut result = true;
        let status = unsafe { qtty_units_compatible(a, b, &mut result) };
        assert_eq!(status, QTTY_OK);
        assert!(!result, "{:?} and {:?} should be incompatible", a, b);
    }
}

// =============================================================================
// Known Conversion Tests
// =============================================================================

#[test]
fn test_conversion_1000_meters_to_1_kilometer() {
    let src = QttyQuantity::new(1000.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(dst.value, 1.0, epsilon = 1e-12);
    assert_eq!(dst.unit, UnitId::Kilometer);
}

#[test]
fn test_conversion_3600_seconds_to_1_hour() {
    let src = QttyQuantity::new(3600.0, UnitId::Second);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Hour, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(dst.value, 1.0, epsilon = 1e-12);
    assert_eq!(dst.unit, UnitId::Hour);
}

#[test]
fn test_conversion_180_degrees_to_pi_radians() {
    let src = QttyQuantity::new(180.0, UnitId::Degree);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Radian, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(dst.value, PI, epsilon = 1e-12);
    assert_eq!(dst.unit, UnitId::Radian);
}

#[test]
fn test_conversion_90_degrees_to_half_pi_radians() {
    let src = QttyQuantity::new(90.0, UnitId::Degree);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Radian, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(dst.value, PI / 2.0, epsilon = 1e-12);
}

#[test]
fn test_conversion_1_day_to_24_hours() {
    let src = QttyQuantity::new(1.0, UnitId::Day);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Hour, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(dst.value, 24.0, epsilon = 1e-12);
}

#[test]
fn test_conversion_1_hour_to_60_minutes() {
    let src = QttyQuantity::new(1.0, UnitId::Hour);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Minute, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(dst.value, 60.0, epsilon = 1e-12);
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[test]
fn test_incompatible_conversion_returns_error() {
    let src = QttyQuantity::new(100.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Second, &mut dst) };

    assert_eq!(status, QTTY_ERR_INCOMPATIBLE_DIM);
}

#[test]
fn test_null_out_pointer_returns_error() {
    let src = QttyQuantity::new(100.0, UnitId::Meter);

    // SAFETY: We're intentionally passing a null pointer to test error handling
    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer, std::ptr::null_mut()) };

    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_null_dimension_out_pointer() {
    let status = unsafe { qtty_unit_dimension(UnitId::Meter, std::ptr::null_mut()) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_null_compatible_out_pointer() {
    let status =
        unsafe { qtty_units_compatible(UnitId::Meter, UnitId::Kilometer, std::ptr::null_mut()) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_null_make_out_pointer() {
    let status = unsafe { qtty_quantity_make(100.0, UnitId::Meter, std::ptr::null_mut()) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_null_convert_value_out_pointer() {
    let status = unsafe {
        qtty_quantity_convert_value(
            100.0,
            UnitId::Meter,
            UnitId::Kilometer,
            std::ptr::null_mut(),
        )
    };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

// =============================================================================
// Layout Tests
// =============================================================================

#[test]
fn test_qtty_quantity_size() {
    // QttyQuantity should be 16 bytes: f64 (8) + u32 (4) + padding (4)
    assert_eq!(std::mem::size_of::<QttyQuantity>(), 16);
}

#[test]
fn test_qtty_quantity_alignment() {
    // QttyQuantity should be aligned to 8 bytes (alignment of f64)
    assert_eq!(std::mem::align_of::<QttyQuantity>(), 8);
}

#[test]
fn test_unit_id_size() {
    // UnitId should be 4 bytes (u32)
    assert_eq!(std::mem::size_of::<UnitId>(), 4);
}

#[test]
fn test_dimension_id_size() {
    // DimensionId should be 4 bytes (u32)
    assert_eq!(std::mem::size_of::<DimensionId>(), 4);
}

// =============================================================================
// Unit Name Tests
// =============================================================================

#[test]
fn test_unit_names() {
    let test_cases = [
        (UnitId::Meter, "Meter"),
        (UnitId::Kilometer, "Kilometer"),
        (UnitId::Second, "Second"),
        (UnitId::Minute, "Minute"),
        (UnitId::Hour, "Hour"),
        (UnitId::Day, "Day"),
        (UnitId::Radian, "Radian"),
        (UnitId::Degree, "Degree"),
    ];

    for (unit, expected_name) in test_cases {
        let name_ptr = qtty_unit_name(unit);
        assert!(
            !name_ptr.is_null(),
            "Name for {:?} should not be null",
            unit
        );

        // SAFETY: We verified the pointer is not null and it points to static memory
        let name = unsafe { CStr::from_ptr(name_ptr) };
        assert_eq!(
            name.to_str().unwrap(),
            expected_name,
            "Name mismatch for {:?}",
            unit
        );
    }
}

// =============================================================================
// Version Test
// =============================================================================

#[test]
fn test_ffi_version() {
    assert_eq!(qtty_ffi_version(), 1);
}

// =============================================================================
// Rust Integration Tests
// =============================================================================

#[test]
fn test_rust_helpers_meters_to_kilometers() {
    use qtty::length::{Kilometers, Meters};

    let meters = Meters::new(1000.0);
    let ffi: QttyQuantity = meters.into();

    assert_relative_eq!(ffi.value, 1000.0);
    assert_eq!(ffi.unit, UnitId::Meter);

    let km: Kilometers = ffi.try_into().unwrap();
    assert_relative_eq!(km.value(), 1.0, epsilon = 1e-12);
}

#[test]
fn test_rust_helpers_hours_to_seconds() {
    use qtty::time::{Hours, Seconds};

    let hours = Hours::new(2.0);
    let ffi: QttyQuantity = hours.into();

    assert_relative_eq!(ffi.value, 2.0);
    assert_eq!(ffi.unit, UnitId::Hour);

    let secs: Seconds = ffi.try_into().unwrap();
    assert_relative_eq!(secs.value(), 7200.0, epsilon = 1e-12);
}

#[test]
fn test_rust_helpers_degrees_to_radians() {
    use qtty::angular::{Degrees, Radians};

    let degrees = Degrees::new(360.0);
    let ffi: QttyQuantity = degrees.into();

    let radians: Radians = ffi.try_into().unwrap();
    assert_relative_eq!(radians.value(), 2.0 * PI, epsilon = 1e-12);
}

#[test]
fn test_rust_helpers_incompatible_fails() {
    use qtty::length::Meters;
    use qtty::time::Seconds;

    let meters = Meters::new(100.0);
    let ffi: QttyQuantity = meters.into();

    let result: Result<Seconds, i32> = ffi.try_into();
    assert_eq!(result, Err(QTTY_ERR_INCOMPATIBLE_DIM));
}

// =============================================================================
// Special Value Tests
// =============================================================================

#[test]
fn test_nan_values_propagate() {
    let src = QttyQuantity::new(f64::NAN, UnitId::Meter);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert!(dst.value.is_nan());
}

#[test]
fn test_infinity_values_propagate() {
    let src = QttyQuantity::new(f64::INFINITY, UnitId::Second);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Hour, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert!(dst.value.is_infinite());
    assert!(dst.value.is_sign_positive());
}

#[test]
fn test_negative_infinity_values_propagate() {
    let src = QttyQuantity::new(f64::NEG_INFINITY, UnitId::Second);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Hour, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert!(dst.value.is_infinite());
    assert!(dst.value.is_sign_negative());
}

#[test]
fn test_zero_values() {
    let src = QttyQuantity::new(0.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(dst.value, 0.0);
}

#[test]
fn test_negative_values() {
    let src = QttyQuantity::new(-1000.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();

    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer, &mut dst) };

    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(dst.value, -1.0, epsilon = 1e-12);
}

// =============================================================================
// QttyQuantity method tests
// =============================================================================

#[test]
fn test_qtty_quantity_compatible() {
    let q1 = QttyQuantity::new(100.0, UnitId::Meter);
    let q2 = QttyQuantity::new(1.0, UnitId::Kilometer);
    let q3 = QttyQuantity::new(10.0, UnitId::Second);

    assert!(q1.compatible(&q2));
    assert!(!q1.compatible(&q3));
}

#[test]
fn test_qtty_quantity_dimension() {
    let q = QttyQuantity::new(100.0, UnitId::Meter);
    assert_eq!(q.dimension(), Some(DimensionId::Length));

    let q2 = QttyQuantity::new(10.0, UnitId::Second);
    assert_eq!(q2.dimension(), Some(DimensionId::Time));
}

#[test]
fn test_qtty_quantity_convert_to() {
    let q = QttyQuantity::new(1000.0, UnitId::Meter);
    let converted = q.convert_to(UnitId::Kilometer);
    assert!(converted.is_some());
    let conv = converted.unwrap();
    assert_relative_eq!(conv.value, 1.0, epsilon = 1e-12);
    assert_eq!(conv.unit, UnitId::Kilometer);
}

#[test]
fn test_qtty_quantity_convert_to_incompatible() {
    let q = QttyQuantity::new(1000.0, UnitId::Meter);
    let converted = q.convert_to(UnitId::Second);
    assert!(converted.is_none());
}

#[test]
fn test_qtty_quantity_add() {
    let q1 = QttyQuantity::new(100.0, UnitId::Meter);
    let q2 = QttyQuantity::new(0.5, UnitId::Kilometer);
    let result = q1.add(&q2);
    assert!(result.is_some());
    let sum = result.unwrap();
    assert_relative_eq!(sum.value, 600.0, epsilon = 1e-10); // 100 + 500
    assert_eq!(sum.unit, UnitId::Meter);
}

#[test]
fn test_qtty_quantity_add_incompatible() {
    let q1 = QttyQuantity::new(100.0, UnitId::Meter);
    let q2 = QttyQuantity::new(10.0, UnitId::Second);
    assert!(q1.add(&q2).is_none());
}

#[test]
fn test_qtty_quantity_sub() {
    let q1 = QttyQuantity::new(1.0, UnitId::Kilometer);
    let q2 = QttyQuantity::new(500.0, UnitId::Meter);
    let result = q1.sub(&q2);
    assert!(result.is_some());
    let diff = result.unwrap();
    assert_relative_eq!(diff.value, 0.5, epsilon = 1e-10); // 1000 - 500 = 500m = 0.5km
    assert_eq!(diff.unit, UnitId::Kilometer);
}

#[test]
fn test_qtty_quantity_sub_incompatible() {
    let q1 = QttyQuantity::new(100.0, UnitId::Meter);
    let q2 = QttyQuantity::new(10.0, UnitId::Second);
    assert!(q1.sub(&q2).is_none());
}

#[test]
fn test_qtty_quantity_mul_scalar() {
    let q = QttyQuantity::new(100.0, UnitId::Meter);
    let result = q.mul_scalar(2.5);
    assert_relative_eq!(result.value, 250.0);
    assert_eq!(result.unit, UnitId::Meter);
}

#[test]
fn test_qtty_quantity_div_scalar() {
    let q = QttyQuantity::new(100.0, UnitId::Meter);
    let result = q.div_scalar(4.0);
    assert_relative_eq!(result.value, 25.0);
    assert_eq!(result.unit, UnitId::Meter);
}

#[test]
fn test_qtty_quantity_div_scalar_zero() {
    let q = QttyQuantity::new(100.0, UnitId::Meter);
    let result = q.div_scalar(0.0);
    assert!(result.value.is_infinite());
}

// =============================================================================
// qtty_quantity_format tests
// =============================================================================

#[test]
fn test_quantity_format_default_decimal() {
    let qty = QttyQuantity::new(100.0, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, -1, 0, buf.as_mut_ptr(), 64) };
    assert!(status > 0);
    let s = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .unwrap();
    assert!(s.contains("100") && s.contains('m'));
}

#[test]
fn test_quantity_format_with_precision() {
    let qty = QttyQuantity::new(PI, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, 2, 0, buf.as_mut_ptr(), 64) };
    assert!(status > 0);
    let s = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .unwrap();
    assert_eq!(s, "3.14 m");
}

#[test]
fn test_quantity_format_lower_exp() {
    let qty = QttyQuantity::new(1234.0, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, -1, QTTY_FMT_LOWER_EXP, buf.as_mut_ptr(), 64) };
    assert!(status > 0);
    let s = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .unwrap();
    assert!(s.contains('e'), "Expected lowercase 'e' in '{}'", s);
    assert!(s.contains('m'));
}

#[test]
fn test_quantity_format_upper_exp() {
    let qty = QttyQuantity::new(1234.0, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, -1, QTTY_FMT_UPPER_EXP, buf.as_mut_ptr(), 64) };
    assert!(status > 0);
    let s = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .unwrap();
    assert!(s.contains('E'), "Expected uppercase 'E' in '{}'", s);
    assert!(s.contains('m'));
}

#[test]
fn test_quantity_format_upper_exp_with_precision() {
    let qty = QttyQuantity::new(1234.0, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, 2, QTTY_FMT_UPPER_EXP, buf.as_mut_ptr(), 64) };
    assert!(status > 0);
    let s = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .unwrap();
    assert!(s.contains('E'), "Expected uppercase 'E' in '{}'", s);
}

#[test]
fn test_quantity_format_lower_exp_with_precision() {
    let qty = QttyQuantity::new(1234.0, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, 3, QTTY_FMT_LOWER_EXP, buf.as_mut_ptr(), 64) };
    assert!(status > 0);
    let s = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .unwrap();
    assert!(s.contains('e'), "Expected lowercase 'e' in '{}'", s);
}

#[test]
fn test_quantity_format_null_buf_returns_err() {
    let qty = QttyQuantity::new(1.0, UnitId::Meter);
    let status = unsafe { qtty_quantity_format(qty, -1, 0, std::ptr::null_mut(), 64) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_quantity_format_zero_buf_len_returns_err() {
    let qty = QttyQuantity::new(1.0, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, -1, 0, buf.as_mut_ptr(), 0) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_quantity_format_buffer_too_small() {
    let qty = QttyQuantity::new(1234567890.0, UnitId::Meter);
    let mut buf = [0i8; 4]; // way too small
    let status = unsafe { qtty_quantity_format(qty, -1, 0, buf.as_mut_ptr(), 4) };
    assert_eq!(status, QTTY_ERR_BUFFER_TOO_SMALL);
}

// =============================================================================
// qtty_string_free tests
// =============================================================================

#[test]
fn test_string_free_null_is_safe() {
    // Passing null must not crash
    unsafe { qtty_string_free(std::ptr::null_mut()) };
}

#[test]
fn test_string_free_after_to_json_value() {
    let src = QttyQuantity::new(42.0, UnitId::Meter);
    let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
    let status = unsafe { qtty_quantity_to_json_value(src, &mut ptr) };
    assert_eq!(status, QTTY_OK);
    assert!(!ptr.is_null());
    // Must not crash or leak
    unsafe { qtty_string_free(ptr) };
}

// =============================================================================
// qtty_quantity_to_json / qtty_quantity_from_json tests
// =============================================================================

#[test]
fn test_quantity_to_json_roundtrip() {
    let src = QttyQuantity::new(99.5, UnitId::Kilometer);
    let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
    let status = unsafe { qtty_quantity_to_json(src, &mut ptr) };
    assert_eq!(status, QTTY_OK);
    assert!(!ptr.is_null());

    let json_str = unsafe { std::ffi::CStr::from_ptr(ptr) }
        .to_str()
        .unwrap()
        .to_owned();
    unsafe { qtty_string_free(ptr) };

    // Parse it back
    let json_cstr = std::ffi::CString::new(json_str).unwrap();
    let mut out = QttyQuantity::default();
    let status2 = unsafe { qtty_quantity_from_json(json_cstr.as_ptr(), &mut out) };
    assert_eq!(status2, QTTY_OK);
    assert_relative_eq!(out.value, 99.5);
    assert_eq!(out.unit, UnitId::Kilometer);
}

#[test]
fn test_quantity_to_json_null_out() {
    let src = QttyQuantity::new(1.0, UnitId::Meter);
    let status = unsafe { qtty_quantity_to_json(src, std::ptr::null_mut()) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_quantity_from_json_invalid_json() {
    let bad = b"not_json\0";
    let mut out = QttyQuantity::default();
    let status = unsafe { qtty_quantity_from_json(bad.as_ptr() as *const _, &mut out) };
    assert_eq!(status, QTTY_ERR_INVALID_VALUE);
}

#[test]
fn test_quantity_from_json_null_ptrs() {
    let mut out = QttyQuantity::default();
    let status = unsafe { qtty_quantity_from_json(std::ptr::null(), &mut out) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

// =============================================================================
// qtty_quantity_from_json_value tests
// =============================================================================

#[test]
fn test_quantity_from_json_value_valid() {
    let json = b"123.4\0";
    let mut out = QttyQuantity::default();
    let status = unsafe {
        qtty_quantity_from_json_value(UnitId::Meter, json.as_ptr() as *const _, &mut out)
    };
    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(out.value, 123.4);
    assert_eq!(out.unit, UnitId::Meter);
}

#[test]
fn test_quantity_from_json_value_invalid_number() {
    let json = b"not_a_number\0";
    let mut out = QttyQuantity::default();
    let status = unsafe {
        qtty_quantity_from_json_value(UnitId::Meter, json.as_ptr() as *const _, &mut out)
    };
    assert_eq!(status, QTTY_ERR_INVALID_VALUE);
}

#[test]
fn test_quantity_from_json_value_null_json() {
    let mut out = QttyQuantity::default();
    let status =
        unsafe { qtty_quantity_from_json_value(UnitId::Meter, std::ptr::null(), &mut out) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

// =============================================================================
// qtty_derived_make tests
// =============================================================================

#[test]
fn test_derived_make_basic() {
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

// =============================================================================
// qtty_derived_convert tests
// =============================================================================

#[test]
fn test_derived_convert_m_per_s_to_km_per_h() {
    let mut src = QttyDerivedQuantity::default();
    let make_status = unsafe { qtty_derived_make(100.0, UnitId::Meter, UnitId::Second, &mut src) };
    assert_eq!(make_status, QTTY_OK);

    let mut out = QttyDerivedQuantity::default();
    let status = unsafe { qtty_derived_convert(src, UnitId::Kilometer, UnitId::Hour, &mut out) };
    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(out.value, 360.0, epsilon = 1e-9);
    assert_eq!(out.numerator, UnitId::Kilometer);
    assert_eq!(out.denominator, UnitId::Hour);
}

#[test]
fn test_derived_convert_null_out() {
    let src = QttyDerivedQuantity::default();
    let status =
        unsafe { qtty_derived_convert(src, UnitId::Kilometer, UnitId::Hour, std::ptr::null_mut()) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_derived_convert_incompatible_dimension() {
    let mut src = QttyDerivedQuantity::default();
    unsafe { qtty_derived_make(100.0, UnitId::Meter, UnitId::Second, &mut src) };

    // Meter / Second cannot be converted to Kilogram / Second (numerator dim mismatch)
    let mut out = QttyDerivedQuantity::default();
    let status = unsafe { qtty_derived_convert(src, UnitId::Gram, UnitId::Second, &mut out) };
    assert_eq!(status, QTTY_ERR_INCOMPATIBLE_DIM);
}

// =============================================================================
// qtty_derived_to_json / qtty_derived_from_json tests
// =============================================================================

#[test]
fn test_derived_to_json_and_free() {
    let mut src = QttyDerivedQuantity::default();
    unsafe { qtty_derived_make(60.0, UnitId::Meter, UnitId::Second, &mut src) };

    let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
    let status = unsafe { qtty_derived_to_json(src, &mut ptr) };
    assert_eq!(status, QTTY_OK);
    assert!(!ptr.is_null());

    let s = unsafe { std::ffi::CStr::from_ptr(ptr) }
        .to_str()
        .unwrap()
        .to_owned();
    unsafe { qtty_string_free(ptr) };

    assert!(s.contains("60") || s.contains("6e"));
}

#[test]
fn test_derived_to_json_null_out() {
    let src = QttyDerivedQuantity::default();
    let status = unsafe { qtty_derived_to_json(src, std::ptr::null_mut()) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}

#[test]
fn test_derived_from_json_roundtrip() {
    let mut src = QttyDerivedQuantity::default();
    unsafe { qtty_derived_make(42.0, UnitId::Kilometer, UnitId::Hour, &mut src) };

    let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut();
    unsafe { qtty_derived_to_json(src, &mut ptr) };
    let json_str = unsafe { std::ffi::CStr::from_ptr(ptr) }
        .to_str()
        .unwrap()
        .to_owned();
    unsafe { qtty_string_free(ptr) };

    let json_cstr = std::ffi::CString::new(json_str).unwrap();
    let mut out = QttyDerivedQuantity::default();
    let status = unsafe { qtty_derived_from_json(json_cstr.as_ptr(), &mut out) };
    assert_eq!(status, QTTY_OK);
    assert_relative_eq!(out.value, 42.0);
    assert_eq!(out.numerator, UnitId::Kilometer);
    assert_eq!(out.denominator, UnitId::Hour);
}

#[test]
fn test_derived_from_json_invalid_json() {
    let bad = b"not_json\0";
    let mut out = QttyDerivedQuantity::default();
    let status = unsafe { qtty_derived_from_json(bad.as_ptr() as *const _, &mut out) };
    assert_eq!(status, QTTY_ERR_INVALID_VALUE);
}

#[test]
fn test_derived_from_json_null_ptrs() {
    let mut out = QttyDerivedQuantity::default();
    let status = unsafe { qtty_derived_from_json(std::ptr::null(), &mut out) };
    assert_eq!(status, QTTY_ERR_NULL_OUT);
}
