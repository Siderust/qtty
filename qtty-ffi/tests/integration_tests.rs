//! Integration tests for qtty-ffi.
//!
//! These tests verify the FFI functions work correctly from a consumer's perspective.

use approx::assert_relative_eq;
use core::f64::consts::PI;
use qtty_ffi::{
    qtty_ffi_version, qtty_quantity_convert, qtty_quantity_convert_value, qtty_quantity_make,
    qtty_unit_dimension, qtty_unit_is_valid, qtty_unit_name, qtty_units_compatible, DimensionId,
    QttyQuantity, UnitId, QTTY_ERR_INCOMPATIBLE_DIM, QTTY_ERR_NULL_OUT, QTTY_OK,
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
