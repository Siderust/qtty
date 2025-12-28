//! PyO3 conversions for `Quantity` types (feature-gated).

use crate::{Quantity, Unit};
use pyo3::prelude::*;

trait QuantityRepr: Sized {
    fn value(&self) -> f64;
    fn from_value(value: f64) -> Self;
}

impl<U: Unit> QuantityRepr for Quantity<U> {
    #[inline]
    fn value(&self) -> f64 {
        (*self).value()
    }

    #[inline]
    fn from_value(value: f64) -> Self {
        Self::new(value)
    }
}

macro_rules! impl_py_for_quantity {
    () => {
        impl<'py, U: Unit> pyo3::conversion::IntoPyObject<'py> for Quantity<U> {
            type Target = pyo3::types::PyFloat;
            type Output = pyo3::Bound<'py, pyo3::types::PyFloat>;
            type Error = core::convert::Infallible;

            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                Ok(pyo3::types::PyFloat::new(py, <Self as QuantityRepr>::value(&self)))
            }
        }

        impl<'a, 'py, U: Unit> pyo3::conversion::FromPyObject<'a, 'py> for Quantity<U> {
            type Error = pyo3::PyErr;

            fn extract(obj: pyo3::Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
                let value = <f64 as pyo3::conversion::FromPyObject<'a, 'py>>::extract(obj)?;
                Ok(<Self as QuantityRepr>::from_value(value))
            }
        }
    };
}

impl_py_for_quantity!();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::angular::Degrees;
    use crate::length::{Meter, Meters};
    use crate::time::Second;
    use crate::velocity::Velocity;

    #[test]
    fn quantity_to_python_roundtrip() {
        Python::with_gil(|py| {
            let meters = Meters::new(42.5);
            let obj = meters.to_object(py);
            let extracted = obj.extract::<f64>(py).unwrap();
            assert_eq!(extracted, 42.5);
        });
    }

    #[test]
    fn quantity_from_python_roundtrip() {
        Python::with_gil(|py| {
            let obj = 12.25f64.to_object(py);
            let any = obj.as_ref(py);
            let meters = Meters::extract(any).unwrap();
            assert_eq!(meters.value(), 12.25);
        });
    }

    #[test]
    fn quantity_from_python_other_unit() {
        Python::with_gil(|py| {
            let obj = 180.0f64.to_object(py);
            let any = obj.as_ref(py);
            let degrees = Degrees::extract(any).unwrap();
            assert_eq!(degrees.value(), 180.0);
        });
    }

    #[test]
    fn quantity_from_python_per_unit_alias() {
        Python::with_gil(|py| {
            let obj = 3.5f64.to_object(py);
            let any = obj.as_ref(py);
            let velocity: Velocity<Meter, Second> = Velocity::extract(any).unwrap();
            assert_eq!(velocity.value(), 3.5);
        });
    }
}
