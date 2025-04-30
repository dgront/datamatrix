use pyo3::prelude::*;

mod py_datamatrix;
pub use py_datamatrix::*;

mod py_datamatrix_builder;
pub use py_datamatrix_builder::*;

/// Python module definition
#[pymodule]
fn datamatrix(m: &Bound<'_, PyModule>) -> PyResult<()> {

    m.add_class::<PyDataMatrix>()?;
    m.add_class::<PyDataMatrixBuilder>()?;

    Ok(())
}
