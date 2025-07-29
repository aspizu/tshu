mod commands;
mod enc;
mod templates;

use pyo3::prelude::*;

use crate::commands::_execute_command;

#[pymodule]
fn tshu(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_execute_command, m)?)?;
    Ok(())
}
