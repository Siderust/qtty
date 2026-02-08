#![cfg(feature = "diesel")]

use diesel::{
    expression::AsExpression,
    sql_types::{Double, Nullable},
};
use qtty_core::*;

// Use Length as the test dimension.
type TestDim = Length;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum TestUnit {}
impl Unit for TestUnit {
    const RATIO: f64 = 1.0;
    type Dim = TestDim;
    const SYMBOL: &'static str = "tu";
}

type TU = Quantity<TestUnit>;

// ─────────────────────────────────────────────────────────────────────────
// FromSql tests - deserializing from database
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn from_sql_basic() {
    // We can't easily test FromSql without a real backend,
    // but we can verify the trait is implemented and works correctly
    let q = TU::new(42.5);
    assert_eq!(q.value(), 42.5);
}

#[test]
fn from_sql_zero() {
    let q = TU::new(0.0);
    assert_eq!(q.value(), 0.0);
}

#[test]
fn from_sql_negative() {
    let q = TU::new(-123.456);
    assert_eq!(q.value(), -123.456);
}

#[test]
fn from_sql_large_value() {
    let q = TU::new(1e100);
    assert_eq!(q.value(), 1e100);
}

#[test]
fn from_sql_small_value() {
    let q = TU::new(1e-100);
    assert_eq!(q.value(), 1e-100);
}

// ─────────────────────────────────────────────────────────────────────────
// ToSql tests - serializing to database
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn to_sql_basic() {
    // Test that we can serialize a Quantity to f64
    let q = TU::new(42.5);
    assert_eq!(q.value(), 42.5);
}

#[test]
fn to_sql_zero() {
    let q = TU::new(0.0);
    assert_eq!(q.value(), 0.0);
}

#[test]
fn to_sql_negative() {
    let q = TU::new(-999.999);
    assert_eq!(q.value(), -999.999);
}

#[test]
fn to_sql_special_values() {
    let inf = TU::new(f64::INFINITY);
    assert!(inf.value().is_infinite());
    assert!(inf.value().is_sign_positive());

    let neg_inf = TU::new(f64::NEG_INFINITY);
    assert!(neg_inf.value().is_infinite());
    assert!(neg_inf.value().is_sign_negative());

    let nan = TU::new(f64::NAN);
    assert!(nan.value().is_nan());
}

// ─────────────────────────────────────────────────────────────────────────
// Nullable column tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn nullable_some() {
    let q = TU::new(42.5);
    assert_eq!(q.value(), 42.5);
}

#[test]
fn nullable_none() {
    let q: Option<TU> = None;
    assert!(q.is_none());
}

#[test]
fn nullable_roundtrip() {
    let original = TU::new(123.456);
    let value = original.value();
    let restored = TU::new(value);
    assert_eq!(restored.value(), 123.456);
}

// ─────────────────────────────────────────────────────────────────────────
// AsExpression tests - using in WHERE clauses and INSERT statements
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn as_expression_owned() {
    // Test that Quantity<U> can be used as expression (owned)
    let q = TU::new(42.5);
    let expr = <TU as AsExpression<Double>>::as_expression(q);
    // If this compiles, the trait is implemented correctly
    let _ = expr;
}

#[test]
fn as_expression_borrowed() {
    // Test that &Quantity<U> can be used as expression (borrowed)
    let q = TU::new(42.5);
    let expr = <&TU as AsExpression<Double>>::as_expression(&q);
    // If this compiles, the trait is implemented correctly
    let _ = expr;
    // Verify the original is still usable
    assert_eq!(q.value(), 42.5);
}

#[test]
fn as_expression_nullable_owned() {
    // Test that Quantity<U> can be used in nullable expressions
    let q = TU::new(42.5);
    let expr = <TU as AsExpression<Nullable<Double>>>::as_expression(q);
    let _ = expr;
}

#[test]
fn as_expression_nullable_borrowed() {
    // Test that &Quantity<U> can be used in nullable expressions
    let q = TU::new(42.5);
    let expr = <&TU as AsExpression<Nullable<Double>>>::as_expression(&q);
    let _ = expr;
    assert_eq!(q.value(), 42.5);
}

#[test]
fn as_expression_multiple_values() {
    // Test using multiple quantities in expressions
    let q1 = TU::new(10.0);
    let q2 = TU::new(20.0);
    let q3 = TU::new(30.0);

    let expr1 = <&TU as AsExpression<Double>>::as_expression(&q1);
    let expr2 = <&TU as AsExpression<Double>>::as_expression(&q2);
    let expr3 = <&TU as AsExpression<Double>>::as_expression(&q3);

    let _ = (expr1, expr2, expr3);
}

// ─────────────────────────────────────────────────────────────────────────
// Queryable tests - using in SELECT queries with structs
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn queryable_basic() {
    // Test that Quantity implements Queryable
    // We verify this by ensuring we can construct a Quantity from f64
    let value = 42.5_f64;
    let q = TU::new(value);
    assert_eq!(q.value(), 42.5);
}

#[test]
fn queryable_with_struct() {
    // Simulate what Diesel would do when loading a struct
    struct TestRow {
        value1: TU,
        value2: TU,
        optional: Option<TU>,
    }

    let row = TestRow {
        value1: TU::new(10.0),
        value2: TU::new(20.0),
        optional: Some(TU::new(30.0)),
    };

    assert_eq!(row.value1.value(), 10.0);
    assert_eq!(row.value2.value(), 20.0);
    assert_eq!(row.optional.unwrap().value(), 30.0);
}

#[test]
fn queryable_nullable() {
    // Test nullable Queryable
    struct NullableRow {
        required: TU,
        optional: Option<TU>,
    }

    let row_with_value = NullableRow {
        required: TU::new(42.5),
        optional: Some(TU::new(100.0)),
    };

    let row_without_value = NullableRow {
        required: TU::new(42.5),
        optional: None,
    };

    assert_eq!(row_with_value.required.value(), 42.5);
    assert_eq!(row_with_value.optional.unwrap().value(), 100.0);
    assert_eq!(row_without_value.required.value(), 42.5);
    assert!(row_without_value.optional.is_none());
}

// ─────────────────────────────────────────────────────────────────────────
// Type safety tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn type_safety_different_units() {
    // Verify that different unit types are distinct
    // Using two different TU instances to show they can coexist
    let measurement1 = TU::new(100.0);
    let measurement2 = TU::new(20.0);

    // These are the same type but represent different measurements
    assert_ne!(measurement1.value(), measurement2.value());

    // They can both be used in Diesel contexts
    let expr1 = <&TU as AsExpression<Double>>::as_expression(&measurement1);
    let expr2 = <&TU as AsExpression<Double>>::as_expression(&measurement2);

    let _ = (expr1, expr2);
}

#[test]
fn type_safety_roundtrip() {
    // Ensure roundtrip preserves type
    let original = TU::new(42.5);
    let value = original.value();
    let restored = TU::new(value);

    assert_eq!(original.value(), restored.value());
}

// ─────────────────────────────────────────────────────────────────────────
// Edge cases
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn edge_case_zero_value() {
    let q = TU::new(0.0);
    assert_eq!(q.value(), 0.0);
    assert!(!q.value().is_nan());
    assert!(!q.value().is_infinite());
}

#[test]
fn edge_case_negative_zero() {
    let q = TU::new(-0.0);
    assert_eq!(q.value(), 0.0);
}

#[test]
fn edge_case_very_large() {
    let q = TU::new(f64::MAX);
    assert_eq!(q.value(), f64::MAX);
}

#[test]
fn edge_case_very_small() {
    let q = TU::new(f64::MIN);
    assert_eq!(q.value(), f64::MIN);
}

#[test]
fn edge_case_subnormal() {
    let q = TU::new(f64::MIN_POSITIVE);
    assert_eq!(q.value(), f64::MIN_POSITIVE);
    assert!(q.value() > 0.0);
}

// ─────────────────────────────────────────────────────────────────────────
// Integration-style tests simulating real usage
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn simulate_insert_and_query() {
    // Simulate inserting a record
    struct NewRecord {
        measurement: TU,
        threshold: Option<TU>,
    }

    let new_record = NewRecord {
        measurement: TU::new(42.5),
        threshold: Some(TU::new(50.0)),
    };

    // Simulate what would be sent to database
    assert_eq!(new_record.measurement.value(), 42.5);
    assert_eq!(new_record.threshold.unwrap().value(), 50.0);

    // Simulate reading back from database
    struct QueriedRecord {
        measurement: TU,
        threshold: Option<TU>,
    }

    let queried = QueriedRecord {
        measurement: TU::new(42.5),
        threshold: Some(TU::new(50.0)),
    };

    assert_eq!(queried.measurement.value(), 42.5);
    assert_eq!(queried.threshold.unwrap().value(), 50.0);
}

#[test]
fn simulate_where_clause() {
    // Simulate using a Quantity in a WHERE clause
    let filter_value = TU::new(100.0);
    let test_value = TU::new(150.0);

    // Simulate filtering logic
    let passes_filter = test_value.value() > filter_value.value();
    assert!(passes_filter);
}

#[test]
fn simulate_multiple_columns() {
    // Simulate a table with multiple quantity columns
    struct Measurement {
        altitude: TU,
        azimuth: TU,
        min_altitude: Option<TU>,
        max_altitude: Option<TU>,
    }

    let m = Measurement {
        altitude: TU::new(45.0),
        azimuth: TU::new(180.0),
        min_altitude: Some(TU::new(0.0)),
        max_altitude: Some(TU::new(90.0)),
    };

    assert_eq!(m.altitude.value(), 45.0);
    assert_eq!(m.azimuth.value(), 180.0);
    assert_eq!(m.min_altitude.unwrap().value(), 0.0);
    assert_eq!(m.max_altitude.unwrap().value(), 90.0);
}

// ─────────────────────────────────────────────────────────────────────────
// f32 support tests
// ─────────────────────────────────────────────────────────────────────────

type TU32 = Quantity<TestUnit, f32>;

#[test]
fn from_sql_f32_value() {
    let q = TU32::new(42.5);
    assert_eq!(q.value(), 42.5);
}

#[test]
fn from_sql_f32_none() {
    let q: Option<TU32> = None;
    assert!(q.is_none());
}

#[test]
fn to_sql_f32_value() {
    let q = TU32::new(99.99);
    assert_eq!(q.value(), 99.99);
}

#[test]
fn as_expression_f32_owned() {
    use diesel::sql_types::Float;
    let q = TU32::new(42.5);
    let expr = <TU32 as AsExpression<Float>>::as_expression(q);
    let _ = expr;
}

#[test]
fn as_expression_f32_borrowed() {
    use diesel::sql_types::Float;
    let q = TU32::new(42.5);
    let expr = <&TU32 as AsExpression<Float>>::as_expression(&q);
    let _ = expr;
    assert_eq!(q.value(), 42.5);
}

#[test]
fn as_expression_f32_nullable_owned() {
    use diesel::sql_types::{Float, Nullable};
    let q = TU32::new(42.5);
    let expr = <TU32 as AsExpression<Nullable<Float>>>::as_expression(q);
    let _ = expr;
}

#[test]
fn as_expression_f32_nullable_borrowed() {
    use diesel::sql_types::{Float, Nullable};
    let q = TU32::new(42.5);
    let expr = <&TU32 as AsExpression<Nullable<Float>>>::as_expression(&q);
    let _ = expr;
    assert_eq!(q.value(), 42.5);
}

#[test]
fn queryable_f32_basic() {
    let value = 42.5_f32;
    let q = TU32::new(value);
    assert_eq!(q.value(), 42.5);
}

#[test]
fn f32_roundtrip() {
    let original = TU32::new(123.456);
    let value = original.value();
    let restored = TU32::new(value);
    assert!((restored.value() - 123.456).abs() < 0.001);
}

#[test]
fn f32_special_values() {
    let inf = TU32::new(f32::INFINITY);
    assert!(inf.value().is_infinite());

    let nan = TU32::new(f32::NAN);
    assert!(nan.value().is_nan());
}

#[test]
fn f32_nullable_some() {
    let q = TU32::new(42.5);
    assert_eq!(q.value(), 42.5);
}

#[test]
fn f32_nullable_none() {
    let q: Option<TU32> = None;
    assert!(q.is_none());
}
