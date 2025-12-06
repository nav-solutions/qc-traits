use pyo3::prelude::*;
use crate::processing::*;

#[pymodule]
fn gnss(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TimeCorrection>()?;
    Ok(())
}
