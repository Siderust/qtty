use pyo3::prelude::*;
use qtty_ffi::UnitId;

fn main() {
    Python::attach(|py| {
        // This requires UnitId's PyClass impl to come from the same PyO3
        // dependency instance used by this downstream crate.
        let unit = Py::new(py, UnitId::Meter).expect("create UnitId Python object");
        let _ = unit.bind(py);
    });
}
