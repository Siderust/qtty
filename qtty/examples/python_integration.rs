//! Example demonstrating how `Quantity` fields work in `#[pyclass]` structures
//! when the `qtty-core` crate is built with the `pyo3` feature.

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "pyo3")]
use qtty::Degrees;

#[cfg(feature = "pyo3")]
#[pyclass]
pub struct DegreeRange {
    #[pyo3(get, set)]
    pub min: Degrees,
    #[pyo3(get, set)]
    pub max: Degrees,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DegreeRange {
    #[new]
    fn new(min: f64, max: f64) -> Self {
        Self {
            min: Degrees::new(min),
            max: Degrees::new(max),
        }
    }

    fn span(&self) -> f64 {
        (self.max - self.min).value()
    }
}

/// A minimal pyo3 module initializer to export `DegreeRange` when building as a Python extension.
#[cfg(feature = "pyo3")]
#[pymodule]
fn qtty_examples(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DegreeRange>()?;
    Ok(())
}

fn main() {
    #[cfg(feature = "pyo3")]
    {
        Python::attach(|py| {
            let range = DegreeRange::new(0.0, 180.0);
            let obj = Py::new(py, range).expect("create DegreeRange");
            let span: f64 = obj
                .bind(py)
                .call_method0("span")
                .and_then(|v| v.extract())
                .expect("call DegreeRange.span()");
            println!("span = {span}");
        })
    }

    #[cfg(not(feature = "pyo3"))]
    {
        eprintln!("This example requires `--features pyo3`.");
    }
}
