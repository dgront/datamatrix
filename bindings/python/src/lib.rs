use pyo3::prelude::*;

mod py_datamatrix;
pub use py_datamatrix::*;

/// Python module definition
#[pymodule]
fn datamatrix(m: &Bound<'_, PyModule>) -> PyResult<()> {

    m.add_class::<PyDataMatrix>()?;

    m.add_function(wrap_pyfunction!(read_matrix, m)?)?;
    m.add_function(wrap_pyfunction!(read_column, m)?)?;
    m.add_function(wrap_pyfunction!(read_matrix_indexed, m)?)?;

    Ok(())
}
