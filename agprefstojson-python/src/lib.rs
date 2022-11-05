use pyo3::exceptions::*;
use pyo3::{pyfunction, pymodule, PyResult};

#[pyfunction]
pub fn encode(input: &str) -> PyResult<String> {
    Ok(serde_json::to_string_pretty(
        &agprefs::Agpref::from_str(input).map_err(|e| PySyntaxError::new_err(format!("{}", e)))?,
    )
    .map_err(|e| PyValueError::new_err(format!("{}", e)))?)
}

#[pyfunction]
pub fn decode(input: &str) -> PyResult<String> {
    Ok(serde_json::from_str::<agprefs::Agpref>(input)
        .map_err(|e| PySyntaxError::new_err(format!("{}", e)))?
        .to_str()
        .map_err(|e| PyValueError::new_err(format!("{}", e)))?)
}

#[pymodule]
fn agprefstojson(_py: pyo3::Python, m: &pyo3::prelude::PyModule) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(encode, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(decode, m)?)?;
    Ok(())
}
