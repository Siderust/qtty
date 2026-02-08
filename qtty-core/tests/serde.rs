#![cfg(feature = "serde")]

use qtty_core::*;
use serde::{Deserialize, Serialize};

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

#[test]
fn serialize_quantity() {
    let q = TU::new(42.5);
    let json = serde_json::to_string(&q).unwrap();
    assert_eq!(json, "42.5");
}

#[test]
fn deserialize_quantity() {
    let json = "42.5";
    let q: TU = serde_json::from_str(json).unwrap();
    assert_eq!(q.value(), 42.5);
}

#[test]
fn serde_roundtrip() {
    let original = TU::new(123.456);
    let json = serde_json::to_string(&original).unwrap();
    let restored: TU = serde_json::from_str(&json).unwrap();
    assert!((restored.value() - original.value()).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────
// serde_with_unit module tests
// ─────────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Debug)]
struct TestStruct {
    #[serde(with = "crate::serde_with_unit")]
    distance: TU,
}

#[test]
fn serde_with_unit_serialize() {
    let data = TestStruct {
        distance: TU::new(42.5),
    };
    let json = serde_json::to_string(&data).unwrap();
    assert!(json.contains("\"value\""));
    assert!(json.contains("\"unit\""));
    assert!(json.contains("42.5"));
    assert!(json.contains("\"tu\""));
}

#[test]
fn serde_with_unit_deserialize() {
    let json = r#"{"distance":{"value":42.5,"unit":"tu"}}"#;
    let data: TestStruct = serde_json::from_str(json).unwrap();
    assert_eq!(data.distance.value(), 42.5);
}

#[test]
fn serde_with_unit_deserialize_no_unit_field() {
    // Should work without unit field for backwards compatibility
    let json = r#"{"distance":{"value":42.5}}"#;
    let data: TestStruct = serde_json::from_str(json).unwrap();
    assert_eq!(data.distance.value(), 42.5);
}

#[test]
fn serde_with_unit_deserialize_wrong_unit() {
    let json = r#"{"distance":{"value":42.5,"unit":"wrong"}}"#;
    let result: Result<TestStruct, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("unit mismatch") || err_msg.contains("expected"));
}

#[test]
fn serde_with_unit_deserialize_missing_value() {
    let json = r#"{"distance":{"unit":"tu"}}"#;
    let result: Result<TestStruct, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("missing field") || err_msg.contains("value"));
}

#[test]
fn serde_with_unit_deserialize_duplicate_value() {
    let json = r#"{"distance":{"value":42.5,"value":100.0,"unit":"tu"}}"#;
    let result: Result<TestStruct, _> = serde_json::from_str(json);
    // This should either error or use one of the values (implementation-dependent)
    // but we're testing that it doesn't panic
    let _ = result;
}

#[test]
fn serde_with_unit_deserialize_duplicate_unit() {
    let json = r#"{"distance":{"value":42.5,"unit":"tu","unit":"tu"}}"#;
    let result: Result<TestStruct, _> = serde_json::from_str(json);
    // Similar to above - just ensure no panic
    let _ = result;
}

#[test]
fn serde_with_unit_deserialize_invalid_format() {
    // Test the expecting() method by providing wrong format
    let json = r#"{"distance":"not_an_object"}"#;
    let result: Result<TestStruct, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn serde_with_unit_deserialize_array() {
    // Test the expecting() method with array format
    let json = r#"{"distance":[42.5, "tu"]}"#;
    let result: Result<TestStruct, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn serde_with_unit_roundtrip() {
    let original = TestStruct {
        distance: TU::new(123.456),
    };
    let json = serde_json::to_string(&original).unwrap();
    let restored: TestStruct = serde_json::from_str(&json).unwrap();
    assert!((restored.distance.value() - original.distance.value()).abs() < 1e-12);
}

#[test]
fn serde_with_unit_special_values() {
    // Note: JSON doesn't support Infinity and NaN natively.
    // serde_json serializes them as null, which can't be deserialized
    // back to f64. So we'll test with very large numbers instead.
    let test_large = TestStruct {
        distance: TU::new(1e100),
    };
    let json = serde_json::to_string(&test_large).unwrap();
    let restored: TestStruct = serde_json::from_str(&json).unwrap();
    assert!((restored.distance.value() - 1e100).abs() < 1e88);

    let test_small = TestStruct {
        distance: TU::new(-1e-100),
    };
    let json = serde_json::to_string(&test_small).unwrap();
    let restored: TestStruct = serde_json::from_str(&json).unwrap();
    assert!((restored.distance.value() + 1e-100).abs() < 1e-112);
}

// ─────────────────────────────────────────────────────────────────────────
// Additional serde edge cases
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn serde_negative_value() {
    let q = TU::new(-999.999);
    let json = serde_json::to_string(&q).unwrap();
    let restored: TU = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.value(), -999.999);
}

#[test]
fn serde_zero_value() {
    let q = TU::new(0.0);
    let json = serde_json::to_string(&q).unwrap();
    let restored: TU = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.value(), 0.0);
}

#[test]
fn serde_with_unit_negative() {
    let data = TestStruct {
        distance: TU::new(-42.5),
    };
    let json = serde_json::to_string(&data).unwrap();
    let restored: TestStruct = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.distance.value(), -42.5);
}

#[test]
fn serde_with_f32() {
    let q = Quantity::<TestUnit, f32>::new(42.5);
    let json = serde_json::to_string(&q).unwrap();
    let restored: Quantity<TestUnit, f32> = serde_json::from_str(&json).unwrap();
    assert!((restored.value() - 42.5).abs() < 0.01);
}

// ─────────────────────────────────────────────────────────────────────────────
// serde_scalar module tests
// ─────────────────────────────────────────────────────────────────────────────

mod serde_with_unit {
    use super::*;

    pub fn serialize<U, S, Ser>(
        quantity: &Quantity<U, S>,
        serializer: Ser,
    ) -> Result<Ser::Ok, Ser::Error>
    where
        U: Unit,
        S: qtty_core::scalar::Real,
        Ser: serde::Serializer,
    {
        qtty_core::serde_with_unit::serialize(quantity, serializer)
    }

    pub fn deserialize<'de, U, S, D>(deserializer: D) -> Result<Quantity<U, S>, D::Error>
    where
        U: Unit,
        S: qtty_core::scalar::Real,
        D: serde::Deserializer<'de>,
    {
        qtty_core::serde_with_unit::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ScalarStruct {
    #[serde(with = "qtty_core::serde_scalar")]
    distance: TU,
}

#[test]
fn serde_scalar_serialize() {
    let data = ScalarStruct {
        distance: TU::new(42.5),
    };
    let json = serde_json::to_string(&data).unwrap();
    assert!(json.contains("42.5"));
}

#[test]
fn serde_scalar_deserialize() {
    let json = r#"{"distance":42.5}"#;
    let data: ScalarStruct = serde_json::from_str(json).unwrap();
    assert_eq!(data.distance.value(), 42.5);
}

#[test]
fn serde_scalar_roundtrip() {
    let original = ScalarStruct {
        distance: TU::new(123.456),
    };
    let json = serde_json::to_string(&original).unwrap();
    let restored: ScalarStruct = serde_json::from_str(&json).unwrap();
    assert!((restored.distance.value() - 123.456).abs() < 1e-12);
}
