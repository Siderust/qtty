//! PyO3 trait implementations for `Quantity` types (feature-gated).
//!
//! This module is enabled by the `pyo3` feature. It provides `IntoPyObject` and `FromPyObject`
//! implementations that convert `Quantity<U, S>` to/from Python floats.

use crate::{Quantity, Unit};
use crate::scalar::Real;
use pyo3::prelude::*;

impl<'py, U: Unit, S: Real> pyo3::conversion::IntoPyObject<'py> for Quantity<U, S> {
    type Target = pyo3::types::PyFloat;
    type Output = pyo3::Bound<'py, pyo3::types::PyFloat>;
    type Error = core::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(pyo3::types::PyFloat::new(py, self.value().to_f64()))
    }
}

impl<'a, 'py, U: Unit, S: Real> pyo3::conversion::FromPyObject<'a, 'py> for Quantity<U, S> {
    type Error = pyo3::PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let value = <f64 as pyo3::conversion::FromPyObject<'a, 'py>>::extract(obj)?;
        Ok(Quantity::new(S::from_f64(value)))
    }
}
