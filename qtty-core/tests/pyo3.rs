#![cfg(feature = "pyo3")]

use pyo3::prelude::*;
use pyo3::types::PyFloat;
use qtty_core::*;

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum DoubleTestUnit {}
impl Unit for DoubleTestUnit {
    const RATIO: f64 = 2.0;
    type Dim = TestDim;
    const SYMBOL: &'static str = "dtu";
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum HalfTestUnit {}
impl Unit for HalfTestUnit {
    const RATIO: f64 = 0.5;
    type Dim = TestDim;
    const SYMBOL: &'static str = "htu";
}

type TU = Quantity<TestUnit>;
type Dtu = Quantity<DoubleTestUnit>;

/// Helper to initialize Python once for tests
fn with_py<F, R>(f: F) -> R
where
    F: for<'py> FnOnce(Python<'py>) -> R,
{
    Python::initialize();
    Python::attach(f)
}

#[test]
fn into_pyobject_basic() {
    with_py(|py| {
        let q = TU::new(42.5);
        let py_obj = q.into_pyobject(py).unwrap();
        let value: f64 = py_obj.extract().unwrap();
        assert_eq!(value, 42.5);
    });
}

#[test]
fn into_pyobject_negative() {
    with_py(|py| {
        let q = TU::new(-123.456);
        let py_obj = q.into_pyobject(py).unwrap();
        let value: f64 = py_obj.extract().unwrap();
        assert_eq!(value, -123.456);
    });
}

#[test]
fn into_pyobject_zero() {
    with_py(|py| {
        let q = TU::new(0.0);
        let py_obj = q.into_pyobject(py).unwrap();
        let value: f64 = py_obj.extract().unwrap();
        assert_eq!(value, 0.0);
    });
}

#[test]
fn into_pyobject_infinity() {
    with_py(|py| {
        let q = TU::new(f64::INFINITY);
        let py_obj = q.into_pyobject(py).unwrap();
        let value: f64 = py_obj.extract().unwrap();
        assert!(value.is_infinite() && value.is_sign_positive());
    });
}

#[test]
fn into_pyobject_neg_infinity() {
    with_py(|py| {
        let q = TU::new(f64::NEG_INFINITY);
        let py_obj = q.into_pyobject(py).unwrap();
        let value: f64 = py_obj.extract().unwrap();
        assert!(value.is_infinite() && value.is_sign_negative());
    });
}

#[test]
fn into_pyobject_nan() {
    with_py(|py| {
        let q = TU::new(f64::NAN);
        let py_obj = q.into_pyobject(py).unwrap();
        let value: f64 = py_obj.extract().unwrap();
        assert!(value.is_nan());
    });
}

#[test]
fn into_pyobject_very_large() {
    with_py(|py| {
        let q = TU::new(1e100);
        let py_obj = q.into_pyobject(py).unwrap();
        let value: f64 = py_obj.extract().unwrap();
        assert_eq!(value, 1e100);
    });
}

#[test]
fn into_pyobject_very_small() {
    with_py(|py| {
        let q = TU::new(1e-100);
        let py_obj = q.into_pyobject(py).unwrap();
        let value: f64 = py_obj.extract().unwrap();
        assert_eq!(value, 1e-100);
    });
}

#[test]
fn from_pyobject_basic() {
    with_py(|py| {
        let py_float = PyFloat::new(py, 42.5);
        let q: TU = py_float.extract().unwrap();
        assert_eq!(q.value(), 42.5);
    });
}

#[test]
fn from_pyobject_negative() {
    with_py(|py| {
        let py_float = PyFloat::new(py, -123.456);
        let q: TU = py_float.extract().unwrap();
        assert_eq!(q.value(), -123.456);
    });
}

#[test]
fn from_pyobject_zero() {
    with_py(|py| {
        let py_float = PyFloat::new(py, 0.0);
        let q: TU = py_float.extract().unwrap();
        assert_eq!(q.value(), 0.0);
    });
}

#[test]
fn from_pyobject_infinity() {
    with_py(|py| {
        let py_float = PyFloat::new(py, f64::INFINITY);
        let q: TU = py_float.extract().unwrap();
        assert!(q.value().is_infinite() && q.value().is_sign_positive());
    });
}

#[test]
fn from_pyobject_neg_infinity() {
    with_py(|py| {
        let py_float = PyFloat::new(py, f64::NEG_INFINITY);
        let q: TU = py_float.extract().unwrap();
        assert!(q.value().is_infinite() && q.value().is_sign_negative());
    });
}

#[test]
fn from_pyobject_nan() {
    with_py(|py| {
        let py_float = PyFloat::new(py, f64::NAN);
        let q: TU = py_float.extract().unwrap();
        assert!(q.value().is_nan());
    });
}

#[test]
fn from_pyobject_very_large() {
    with_py(|py| {
        let py_float = PyFloat::new(py, 1e100);
        let q: TU = py_float.extract().unwrap();
        assert_eq!(q.value(), 1e100);
    });
}

#[test]
fn from_pyobject_very_small() {
    with_py(|py| {
        let py_float = PyFloat::new(py, 1e-100);
        let q: TU = py_float.extract().unwrap();
        assert_eq!(q.value(), 1e-100);
    });
}

#[test]
fn roundtrip_basic() {
    with_py(|py| {
        let original = TU::new(123.456);
        let py_obj = original.into_pyobject(py).unwrap();
        let restored: TU = py_obj.extract().unwrap();
        assert!((restored.value() - original.value()).abs() < 1e-12);
    });
}

#[test]
fn roundtrip_negative() {
    with_py(|py| {
        let original = TU::new(-987.654);
        let py_obj = original.into_pyobject(py).unwrap();
        let restored: TU = py_obj.extract().unwrap();
        assert!((restored.value() - original.value()).abs() < 1e-12);
    });
}

#[test]
fn roundtrip_zero() {
    with_py(|py| {
        let original = TU::new(0.0);
        let py_obj = original.into_pyobject(py).unwrap();
        let restored: TU = py_obj.extract().unwrap();
        assert_eq!(restored.value(), original.value());
    });
}

#[test]
fn roundtrip_very_large() {
    with_py(|py| {
        let original = TU::new(1e100);
        let py_obj = original.into_pyobject(py).unwrap();
        let restored: TU = py_obj.extract().unwrap();
        assert_eq!(restored.value(), original.value());
    });
}

#[test]
fn roundtrip_very_small() {
    with_py(|py| {
        let original = TU::new(1e-100);
        let py_obj = original.into_pyobject(py).unwrap();
        let restored: TU = py_obj.extract().unwrap();
        assert_eq!(restored.value(), original.value());
    });
}

#[test]
fn roundtrip_different_unit_types() {
    with_py(|py| {
        // Test with DoubleTestUnit
        let original_dtu = Dtu::new(42.5);
        let py_obj = original_dtu.into_pyobject(py).unwrap();
        let restored_dtu: Dtu = py_obj.extract().unwrap();
        assert!((restored_dtu.value() - original_dtu.value()).abs() < 1e-12);

        // Test with HalfTestUnit
        let original_htu = Quantity::<HalfTestUnit>::new(99.9);
        let py_obj = original_htu.into_pyobject(py).unwrap();
        let restored_htu: Quantity<HalfTestUnit> = py_obj.extract().unwrap();
        assert!((restored_htu.value() - original_htu.value()).abs() < 1e-12);
    });
}

#[test]
fn roundtrip_preserves_precision() {
    with_py(|py| {
        // Test that precision is preserved through conversion
        let original = TU::new(core::f64::consts::PI);
        let py_obj = original.into_pyobject(py).unwrap();
        let restored: TU = py_obj.extract().unwrap();
        assert_eq!(restored.value(), original.value());
    });
}

#[test]
fn into_pyobject_returns_pyfloat() {
    with_py(|py| {
        let q = TU::new(42.5);
        let py_obj = q.into_pyobject(py).unwrap();
        // Verify it's actually a PyFloat
        assert!(py_obj.is_instance_of::<PyFloat>());
    });
}

#[test]
fn from_pyobject_from_int() {
    with_py(|py| {
        // Python integers can be converted to f64
        let py_int = py.eval(c"42", None, None).unwrap();
        let q: TU = py_int.extract().unwrap();
        assert_eq!(q.value(), 42.0);
    });
}

#[test]
fn from_pyobject_from_calculation() {
    with_py(|py| {
        // Extract from a Python calculation result
        let py_result = py.eval(c"2.5 * 10.0", None, None).unwrap();
        let q: TU = py_result.extract().unwrap();
        assert_eq!(q.value(), 25.0);
    });
}

#[test]
fn from_pyobject_error_on_string() {
    with_py(|py| {
        // Should fail to extract from a string
        let py_str = py.eval(c"'not a number'", None, None).unwrap();
        let result: PyResult<TU> = py_str.extract();
        assert!(result.is_err());
    });
}

#[test]
fn from_pyobject_error_on_none() {
    with_py(|py| {
        // Should fail to extract from None
        let py_none = py.eval(c"None", None, None).unwrap();
        let result: PyResult<TU> = py_none.extract();
        assert!(result.is_err());
    });
}

#[test]
fn from_pyobject_error_on_list() {
    with_py(|py| {
        // Should fail to extract from a list
        let py_list = py.eval(c"[1, 2, 3]", None, None).unwrap();
        let result: PyResult<TU> = py_list.extract();
        assert!(result.is_err());
    });
}
