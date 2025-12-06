use pyo3::prelude::*;
// use crate::prelude::*;

#[pymodule]
fn gnss(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<Epoch>()?;
    Ok(())
}
