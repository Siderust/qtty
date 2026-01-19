//! PyO3 trait implementations for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `pyo3` feature. It provides `IntoPyObject` and `FromPyObject`
//! implementations that convert `Quantity<U>` to/from Python floats.

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

impl<'py, U: Unit> pyo3::conversion::IntoPyObject<'py> for Quantity<U> {
    type Target = pyo3::types::PyFloat;
    type Output = pyo3::Bound<'py, pyo3::types::PyFloat>;
    type Error = core::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(pyo3::types::PyFloat::new(
            py,
            <Self as QuantityRepr>::value(&self),
        ))
    }
}

impl<'a, 'py, U: Unit> pyo3::conversion::FromPyObject<'a, 'py> for Quantity<U> {
    type Error = pyo3::PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let value = <f64 as pyo3::conversion::FromPyObject<'a, 'py>>::extract(obj)?;
        Ok(<Self as QuantityRepr>::from_value(value))
    }
}
