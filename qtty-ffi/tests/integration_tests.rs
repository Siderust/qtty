// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Vallés Puig, Ramon

//! Integration tests for qtty-ffi (vNext ABI).
//!
//! All FFI function calls use raw `u32` unit IDs and check `QttyStatus` return values.

use approx::assert_relative_eq;
use core::f64::consts::PI;
use qtty_ffi::{
    qtty_derived_convert, qtty_derived_make, qtty_ffi_version, qtty_quantity_convert,
    qtty_quantity_convert_value, qtty_quantity_format, qtty_quantity_make, qtty_unit_dimension,
    qtty_unit_is_valid, qtty_unit_name, qtty_units_compatible, DimensionId, QttyDerivedQuantity,
    QttyQuantity, QttyStatus, UnitId, QTTY_FMT_LOWER_EXP, QTTY_FMT_UPPER_EXP,
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
        assert!(
            qtty_unit_is_valid(unit as u32),
            "Unit {:?} should be valid",
            unit
        );
    }
}

#[test]
fn test_invalid_unit_ids_are_rejected() {
    for id in [0u32, 1, 9999, 99999, u32::MAX] {
        assert!(!qtty_unit_is_valid(id), "ID {} should be invalid", id);
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
        let status = unsafe { qtty_unit_dimension(unit as u32, &mut dim) };
        assert_eq!(
            status,
            QttyStatus::Ok,
            "Getting dimension for {:?} failed",
            unit
        );
        assert_eq!(dim, expected_dim, "Dimension mismatch for {:?}", unit);
    }
}

#[test]
fn test_unit_dimension_rejects_invalid_id() {
    let mut dim = DimensionId::Length;
    assert_eq!(
        unsafe { qtty_unit_dimension(0, &mut dim) },
        QttyStatus::UnknownUnit
    );
    assert_eq!(
        unsafe { qtty_unit_dimension(9999, &mut dim) },
        QttyStatus::UnknownUnit
    );
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
        let status = unsafe { qtty_units_compatible(a as u32, b as u32, &mut result) };
        assert_eq!(status, QttyStatus::Ok);
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
        let status = unsafe { qtty_units_compatible(a as u32, b as u32, &mut result) };
        assert_eq!(status, QttyStatus::Ok);
        assert!(!result, "{:?} and {:?} should be incompatible", a, b);
    }
}

#[test]
fn test_units_compatible_rejects_invalid_id() {
    let mut result = false;
    assert_eq!(
        unsafe { qtty_units_compatible(0, UnitId::Meter as u32, &mut result) },
        QttyStatus::UnknownUnit
    );
    assert_eq!(
        unsafe { qtty_units_compatible(UnitId::Meter as u32, 9999, &mut result) },
        QttyStatus::UnknownUnit
    );
}

// =============================================================================
// Known Conversion Tests
// =============================================================================

#[test]
fn test_conversion_1000_meters_to_1_kilometer() {
    let src = QttyQuantity::new(1000.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(dst.value, 1.0, epsilon = 1e-12);
    assert_eq!(dst.unit, UnitId::Kilometer as u32);
}

#[test]
fn test_conversion_3600_seconds_to_1_hour() {
    let src = QttyQuantity::new(3600.0, UnitId::Second);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Hour as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(dst.value, 1.0, epsilon = 1e-12);
    assert_eq!(dst.unit, UnitId::Hour as u32);
}

#[test]
fn test_conversion_180_degrees_to_pi_radians() {
    let src = QttyQuantity::new(180.0, UnitId::Degree);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Radian as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(dst.value, PI, epsilon = 1e-12);
    assert_eq!(dst.unit, UnitId::Radian as u32);
}

#[test]
fn test_conversion_90_degrees_to_half_pi_radians() {
    let src = QttyQuantity::new(90.0, UnitId::Degree);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Radian as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(dst.value, PI / 2.0, epsilon = 1e-12);
}

#[test]
fn test_conversion_1_day_to_24_hours() {
    let src = QttyQuantity::new(1.0, UnitId::Day);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Hour as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(dst.value, 24.0, epsilon = 1e-12);
}

#[test]
fn test_conversion_1_hour_to_60_minutes() {
    let src = QttyQuantity::new(1.0, UnitId::Hour);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Minute as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(dst.value, 60.0, epsilon = 1e-12);
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[test]
fn test_incompatible_conversion_returns_error() {
    let src = QttyQuantity::new(100.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Second as u32, &mut dst) };
    assert_eq!(status, QttyStatus::IncompatibleDim);
}

#[test]
fn test_convert_invalid_dst_unit_id() {
    let src = QttyQuantity::new(100.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, 0, &mut dst) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_null_out_pointer_returns_error() {
    let src = QttyQuantity::new(100.0, UnitId::Meter);
    let status =
        unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, std::ptr::null_mut()) };
    assert_eq!(status, QttyStatus::NullOut);
}

#[test]
fn test_null_dimension_out_pointer() {
    let status = unsafe { qtty_unit_dimension(UnitId::Meter as u32, std::ptr::null_mut()) };
    assert_eq!(status, QttyStatus::NullOut);
}

#[test]
fn test_null_compatible_out_pointer() {
    let status = unsafe {
        qtty_units_compatible(
            UnitId::Meter as u32,
            UnitId::Kilometer as u32,
            std::ptr::null_mut(),
        )
    };
    assert_eq!(status, QttyStatus::NullOut);
}

#[test]
fn test_null_make_out_pointer() {
    let status = unsafe { qtty_quantity_make(100.0, UnitId::Meter as u32, std::ptr::null_mut()) };
    assert_eq!(status, QttyStatus::NullOut);
}

#[test]
fn test_null_convert_value_out_pointer() {
    let status = unsafe {
        qtty_quantity_convert_value(
            100.0,
            UnitId::Meter as u32,
            UnitId::Kilometer as u32,
            std::ptr::null_mut(),
        )
    };
    assert_eq!(status, QttyStatus::NullOut);
}

#[test]
fn test_convert_value_invalid_src_unit() {
    let mut out = 0.0f64;
    let status = unsafe { qtty_quantity_convert_value(1.0, 9999, UnitId::Meter as u32, &mut out) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_convert_value_invalid_dst_unit() {
    let mut out = 0.0f64;
    let status = unsafe { qtty_quantity_convert_value(1.0, UnitId::Meter as u32, 9999, &mut out) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

// =============================================================================
// Layout Tests
// =============================================================================

#[test]
fn test_qtty_quantity_size() {
    assert_eq!(std::mem::size_of::<QttyQuantity>(), 16);
}

#[test]
fn test_qtty_quantity_alignment() {
    assert_eq!(std::mem::align_of::<QttyQuantity>(), 8);
}

#[test]
fn test_unit_id_size() {
    assert_eq!(std::mem::size_of::<UnitId>(), 4);
}

#[test]
fn test_dimension_id_size() {
    assert_eq!(std::mem::size_of::<DimensionId>(), 4);
}

#[test]
fn test_qtty_status_size() {
    assert_eq!(std::mem::size_of::<QttyStatus>(), 4);
    assert_eq!(std::mem::align_of::<QttyStatus>(), 4);
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
        let name_ptr = qtty_unit_name(unit as u32);
        assert!(
            !name_ptr.is_null(),
            "Name for {:?} should not be null",
            unit
        );
        let name = unsafe { CStr::from_ptr(name_ptr) };
        assert_eq!(
            name.to_str().unwrap(),
            expected_name,
            "Name mismatch for {:?}",
            unit
        );
    }
}

#[test]
fn test_unit_name_invalid_id_returns_null() {
    let ptr = qtty_unit_name(0);
    assert!(ptr.is_null());
    let ptr2 = qtty_unit_name(9999);
    assert!(ptr2.is_null());
}

// =============================================================================
// Version Test
// =============================================================================

#[test]
fn test_ffi_version() {
    // 0.5.0 → 500
    assert_eq!(qtty_ffi_version(), 500);
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
    assert_eq!(ffi.unit, UnitId::Meter as u32);
    let km: Kilometers = ffi.try_into().unwrap();
    assert_relative_eq!(km.value(), 1.0, epsilon = 1e-12);
}

#[test]
fn test_rust_helpers_hours_to_seconds() {
    use qtty::time::{Hours, Seconds};
    let hours = Hours::new(2.0);
    let ffi: QttyQuantity = hours.into();
    assert_relative_eq!(ffi.value, 2.0);
    assert_eq!(ffi.unit, UnitId::Hour as u32);
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

// =============================================================================
// Special Value Tests
// =============================================================================

#[test]
fn test_nan_values_propagate() {
    let src = QttyQuantity::new(f64::NAN, UnitId::Meter);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert!(dst.value.is_nan());
}

#[test]
fn test_infinity_values_propagate() {
    let src = QttyQuantity::new(f64::INFINITY, UnitId::Second);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Hour as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert!(dst.value.is_infinite() && dst.value.is_sign_positive());
}

#[test]
fn test_negative_infinity_values_propagate() {
    let src = QttyQuantity::new(f64::NEG_INFINITY, UnitId::Second);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Hour as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert!(dst.value.is_infinite() && dst.value.is_sign_negative());
}

#[test]
fn test_zero_values() {
    let src = QttyQuantity::new(0.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(dst.value, 0.0);
}

#[test]
fn test_negative_values() {
    let src = QttyQuantity::new(-1000.0, UnitId::Meter);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, &mut dst) };
    assert_eq!(status, QttyStatus::Ok);
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
    assert_eq!(conv.unit, UnitId::Kilometer as u32);
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
    assert_relative_eq!(sum.value, 600.0, epsilon = 1e-10);
    assert_eq!(sum.unit, UnitId::Meter as u32);
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
    assert_relative_eq!(diff.value, 0.5, epsilon = 1e-10);
    assert_eq!(diff.unit, UnitId::Kilometer as u32);
}

#[test]
fn test_qtty_quantity_mul_scalar() {
    let q = QttyQuantity::new(100.0, UnitId::Meter);
    let result = q.mul_scalar(2.5);
    assert_relative_eq!(result.value, 250.0);
    assert_eq!(result.unit, UnitId::Meter as u32);
}

#[test]
fn test_qtty_quantity_div_scalar() {
    let q = QttyQuantity::new(100.0, UnitId::Meter);
    let result = q.div_scalar(4.0);
    assert_relative_eq!(result.value, 25.0);
    assert_eq!(result.unit, UnitId::Meter as u32);
}

// =============================================================================
// qtty_quantity_format tests
// =============================================================================

#[test]
fn test_quantity_format_default_decimal() {
    let qty = QttyQuantity::new(100.0, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, -1, 0, buf.as_mut_ptr(), 64) };
    assert_eq!(status, QttyStatus::Ok);
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
    assert_eq!(status, QttyStatus::Ok);
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
    assert_eq!(status, QttyStatus::Ok);
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
    assert_eq!(status, QttyStatus::Ok);
    let s = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .unwrap();
    assert!(s.contains('E'), "Expected uppercase 'E' in '{}'", s);
    assert!(s.contains('m'));
}

#[test]
fn test_quantity_format_null_buf_returns_err() {
    let qty = QttyQuantity::new(1.0, UnitId::Meter);
    let status = unsafe { qtty_quantity_format(qty, -1, 0, std::ptr::null_mut(), 64) };
    assert_eq!(status, QttyStatus::NullOut);
}

#[test]
fn test_quantity_format_zero_buf_len_returns_err() {
    let qty = QttyQuantity::new(1.0, UnitId::Meter);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, -1, 0, buf.as_mut_ptr(), 0) };
    assert_eq!(status, QttyStatus::NullOut);
}

#[test]
fn test_quantity_format_buffer_too_small() {
    let qty = QttyQuantity::new(1234567890.0, UnitId::Meter);
    let mut buf = [0i8; 4];
    let status = unsafe { qtty_quantity_format(qty, -1, 0, buf.as_mut_ptr(), 4) };
    assert_eq!(status, QttyStatus::BufferTooSmall);
}

// =============================================================================
// qtty_derived_make tests
// =============================================================================

#[test]
fn test_derived_make_basic() {
    let mut out = QttyDerivedQuantity::default();
    let status =
        unsafe { qtty_derived_make(100.0, UnitId::Meter as u32, UnitId::Second as u32, &mut out) };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(out.value, 100.0);
    assert_eq!(out.numerator, UnitId::Meter as u32);
    assert_eq!(out.denominator, UnitId::Second as u32);
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
    let status = unsafe { qtty_derived_make(1.0, UnitId::Meter as u32, 9999, &mut out) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_derived_make_null_out() {
    let status = unsafe {
        qtty_derived_make(
            1.0,
            UnitId::Meter as u32,
            UnitId::Second as u32,
            std::ptr::null_mut(),
        )
    };
    assert_eq!(status, QttyStatus::NullOut);
}

// =============================================================================
// qtty_derived_convert tests
// =============================================================================

#[test]
fn test_derived_convert_m_per_s_to_km_per_h() {
    let mut src = QttyDerivedQuantity::default();
    let make_status =
        unsafe { qtty_derived_make(100.0, UnitId::Meter as u32, UnitId::Second as u32, &mut src) };
    assert_eq!(make_status, QttyStatus::Ok);

    let mut out = QttyDerivedQuantity::default();
    let status = unsafe {
        qtty_derived_convert(src, UnitId::Kilometer as u32, UnitId::Hour as u32, &mut out)
    };
    assert_eq!(status, QttyStatus::Ok);
    assert_relative_eq!(out.value, 360.0, epsilon = 1e-9);
    assert_eq!(out.numerator, UnitId::Kilometer as u32);
    assert_eq!(out.denominator, UnitId::Hour as u32);
}

#[test]
fn test_derived_convert_null_out() {
    let src = QttyDerivedQuantity::default();
    let status = unsafe {
        qtty_derived_convert(
            src,
            UnitId::Kilometer as u32,
            UnitId::Hour as u32,
            std::ptr::null_mut(),
        )
    };
    assert_eq!(status, QttyStatus::NullOut);
}

#[test]
fn test_derived_convert_incompatible_dimension() {
    let mut src = QttyDerivedQuantity::default();
    unsafe { qtty_derived_make(100.0, UnitId::Meter as u32, UnitId::Second as u32, &mut src) };

    let mut out = QttyDerivedQuantity::default();
    let status =
        unsafe { qtty_derived_convert(src, UnitId::Gram as u32, UnitId::Second as u32, &mut out) };
    assert_eq!(status, QttyStatus::IncompatibleDim);
}

#[test]
fn test_derived_convert_invalid_target_num() {
    let mut src = QttyDerivedQuantity::default();
    unsafe { qtty_derived_make(1.0, UnitId::Meter as u32, UnitId::Second as u32, &mut src) };
    let mut out = QttyDerivedQuantity::default();
    let status = unsafe { qtty_derived_convert(src, 0, UnitId::Hour as u32, &mut out) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_derived_convert_invalid_target_den() {
    let mut src = QttyDerivedQuantity::default();
    unsafe { qtty_derived_make(1.0, UnitId::Meter as u32, UnitId::Second as u32, &mut src) };
    let mut out = QttyDerivedQuantity::default();
    let status = unsafe { qtty_derived_convert(src, UnitId::Kilometer as u32, 9999, &mut out) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

// =============================================================================
// Invalid Discriminant Tests (ABI hardening)
// =============================================================================
// These tests construct `QttyQuantity` and `QttyDerivedQuantity` with raw
// invalid `u32` unit IDs — something only possible now that the fields are
// raw `u32` instead of `UnitId` enums.  Before the ABI hardening this would
// have been instant UB.

#[test]
fn test_convert_with_invalid_src_unit_in_struct() {
    // C caller could fill the struct with any u32
    let src = QttyQuantity::from_raw(100.0, 0xDEAD);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Kilometer as u32, &mut dst) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_convert_with_zero_src_unit_in_struct() {
    let src = QttyQuantity::from_raw(1.0, 0);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Meter as u32, &mut dst) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_convert_with_u32_max_src_unit_in_struct() {
    let src = QttyQuantity::from_raw(1.0, u32::MAX);
    let mut dst = QttyQuantity::default();
    let status = unsafe { qtty_quantity_convert(src, UnitId::Meter as u32, &mut dst) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_format_with_invalid_unit_in_struct() {
    let qty = QttyQuantity::from_raw(1.0, 0xBAD);
    let mut buf = [0i8; 64];
    let status = unsafe { qtty_quantity_format(qty, -1, 0, buf.as_mut_ptr(), 64) };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_derived_convert_with_invalid_src_numerator_in_struct() {
    let src = QttyDerivedQuantity::from_raw(100.0, 0xDEAD, UnitId::Second as u32);
    let mut out = QttyDerivedQuantity::default();
    let status = unsafe {
        qtty_derived_convert(src, UnitId::Kilometer as u32, UnitId::Hour as u32, &mut out)
    };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_derived_convert_with_invalid_src_denominator_in_struct() {
    let src = QttyDerivedQuantity::from_raw(100.0, UnitId::Meter as u32, 0xDEAD);
    let mut out = QttyDerivedQuantity::default();
    let status = unsafe {
        qtty_derived_convert(src, UnitId::Kilometer as u32, UnitId::Hour as u32, &mut out)
    };
    assert_eq!(status, QttyStatus::UnknownUnit);
}

#[test]
fn test_quantity_methods_with_invalid_unit() {
    let bad = QttyQuantity::from_raw(1.0, 0xDEAD);
    let good = QttyQuantity::new(1.0, UnitId::Meter);

    // All methods should gracefully return None/false instead of UB
    assert_eq!(bad.unit_id(), None);
    assert_eq!(bad.dimension(), None);
    assert!(!bad.compatible(&good));
    assert!(!good.compatible(&bad));
    assert!(bad.convert_to(UnitId::Meter).is_none());
    assert!(bad.add(&good).is_none());
    assert!(bad.sub(&good).is_none());
    assert!(good.add(&bad).is_none());
    assert!(good.sub(&bad).is_none());
}

#[test]
fn test_derived_quantity_methods_with_invalid_unit() {
    let bad = QttyDerivedQuantity::from_raw(1.0, 0xDEAD, UnitId::Second as u32);

    assert_eq!(bad.numerator_id(), None);
    assert!(bad.symbol().is_none());
    assert!(bad.convert_to(UnitId::Kilometer, UnitId::Hour).is_none());
}
