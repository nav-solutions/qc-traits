use pyo3::prelude::*;

use crate::processing::{TimeCorrection, TimeCorrectionsDB};

use hifitime::{
    prelude::{Duration, Epoch, TimeScale},
    Polynomial,
};

#[pymodule]
fn qc_traits(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Epoch>()?;
    m.add_class::<Duration>()?;
    m.add_class::<Polynomial>()?;
    m.add_class::<TimeScale>()?;
    m.add_class::<TimeCorrection>()?;
    m.add_class::<TimeCorrectionsDB>()?;
    Ok(())
}
