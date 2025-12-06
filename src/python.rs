use pyo3::prelude::*;

use crate::processing::{TimeCorrection, TimeCorrectionsDB};

#[pymodule]
fn qc_traits(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TimeCorrection>()?;
    m.add_class::<TimeCorrectionsDB>()?;
    Ok(())
}
