#![cfg(feature = "tiberius")]

use qtty_core::*;
use tiberius::{ColumnData, FromSql, ToSql};

#[derive(Debug)]
pub enum TestDim {}
impl Dimension for TestDim {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum TestUnit {}
impl Unit for TestUnit {
    const RATIO: f64 = 1.0;
    type Dim = TestDim;
    const SYMBOL: &'static str = "tu";
}

type TU = Quantity<TestUnit>;

// Helper function for roundtrip testing
fn test_roundtrip(original: TU) {
    let column_data = original.to_sql();
    match column_data {
        ColumnData::F64(Some(val)) => {
            let column_data_copy = ColumnData::F64(Some(val));
            let restored = TU::from_sql(&column_data_copy).unwrap();
            assert!(restored.is_some());
            assert!((restored.unwrap().value() - original.value()).abs() < 1e-12);
        }
        _ => panic!("Expected ColumnData::F64"),
    }
}

#[test]
fn to_sql_converts_to_f64() {
    let q = TU::new(42.5);
    let column_data = q.to_sql();
    match column_data {
        ColumnData::F64(Some(val)) => assert_eq!(val, 42.5),
        _ => panic!("Expected ColumnData::F64(Some(42.5))"),
    }
}

#[test]
fn to_sql_handles_negative_values() {
    let q = TU::new(-123.456);
    let column_data = q.to_sql();
    match column_data {
        ColumnData::F64(Some(val)) => assert_eq!(val, -123.456),
        _ => panic!("Expected ColumnData::F64(Some(-123.456))"),
    }
}

#[test]
fn to_sql_handles_zero() {
    let q = TU::new(0.0);
    let column_data = q.to_sql();
    match column_data {
        ColumnData::F64(Some(val)) => assert_eq!(val, 0.0),
        _ => panic!("Expected ColumnData::F64(Some(0.0))"),
    }
}

#[test]
fn from_sql_f64_value() {
    let column_data = ColumnData::F64(Some(42.5));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), 42.5);
}

#[test]
fn from_sql_f32_value() {
    let column_data = ColumnData::F32(Some(42.5f32));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    // Use 1e-6 tolerance for f32 since it has less precision than f64
    assert!((result.unwrap().value() - 42.5).abs() < 1e-6);
}

#[test]
fn from_sql_i32_value() {
    let column_data = ColumnData::I32(Some(42));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), 42.0);
}

#[test]
fn from_sql_i64_value() {
    let column_data = ColumnData::I64(Some(12345));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), 12345.0);
}

#[test]
fn from_sql_f64_none() {
    let column_data = ColumnData::F64(None);
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_none());
}

#[test]
fn from_sql_f32_none() {
    let column_data = ColumnData::F32(None);
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_none());
}

#[test]
fn from_sql_i32_none() {
    let column_data = ColumnData::I32(None);
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_none());
}

#[test]
fn from_sql_i64_none() {
    let column_data = ColumnData::I64(None);
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_none());
}

#[test]
fn from_sql_negative_i32() {
    let column_data = ColumnData::I32(Some(-42));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), -42.0);
}

#[test]
fn from_sql_negative_i64() {
    let column_data = ColumnData::I64(Some(-12345));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), -12345.0);
}

#[test]
fn roundtrip_to_sql_from_sql() {
    test_roundtrip(TU::new(123.456));
}

#[test]
fn roundtrip_with_large_values() {
    test_roundtrip(TU::new(1e100));
}

#[test]
fn roundtrip_with_small_values() {
    test_roundtrip(TU::new(1e-100));
}

#[test]
fn roundtrip_with_zero() {
    test_roundtrip(TU::new(0.0));
}

// ─────────────────────────────────────────────────────────────────────────────
// Additional FromSql variants for coverage
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn from_sql_i16_value() {
    let column_data = ColumnData::I16(Some(42));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), 42.0);
}

#[test]
fn from_sql_i16_none() {
    let column_data = ColumnData::I16(None);
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_none());
}

#[test]
fn from_sql_u8_value() {
    let column_data = ColumnData::U8(Some(255));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), 255.0);
}

#[test]
fn from_sql_u8_none() {
    let column_data = ColumnData::U8(None);
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_none());
}

#[test]
fn from_sql_default_arm() {
    // Using Bit variant which is not handled -> should return None via default arm
    let column_data = ColumnData::Bit(Some(true));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_none());
}

#[test]
fn from_sql_negative_i16() {
    let column_data = ColumnData::I16(Some(-100));
    let result = TU::from_sql(&column_data).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), -100.0);
}
