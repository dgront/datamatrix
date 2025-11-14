use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use data_matrix::DataMatrixBuilder;
use crate::PyDataMatrix;

#[pyclass(name = "DataMatrixBuilder")]
#[derive(Debug, Clone)]
pub struct PyDataMatrixBuilder {
    inner: DataMatrixBuilder,
}

#[pymethods]
impl PyDataMatrixBuilder {
    #[new]
    pub fn new() -> Self {
        PyDataMatrixBuilder { inner: DataMatrixBuilder::new() }
    }

    pub fn label_columns(&self, row: usize, col: usize) -> Self {
        let new = self.inner.clone().label_columns(row, col);
        PyDataMatrixBuilder { inner: new }
    }

    pub fn data_column(&self, val: usize) -> Self {
        let new = self.inner.clone().data_column(val);
        PyDataMatrixBuilder { inner: new }
    }

    pub fn index_columns(&self, row_idx: usize, col_idx: usize) -> Self {
        let new = self.inner.clone().index_columns(row_idx, col_idx);
        PyDataMatrixBuilder { inner: new }
    }

    pub fn separator(&self, sep: char) -> Self {
        let new = self.inner.clone().separator(sep);
        PyDataMatrixBuilder { inner: new }
    }

    pub fn skip_header(&self, if_header: bool) -> Self {
        let new = self.inner.clone().skip_header(if_header);
        PyDataMatrixBuilder { inner: new }
    }

    pub fn symmetric(&self, if_symmetric: bool) -> Self {
        let new = self.inner.clone().symmetric(if_symmetric);
        PyDataMatrixBuilder { inner: new }
    }

    pub fn labels(&self, labels: Vec<String>) -> Self {
        let new = self.inner.clone().labels(labels);
        PyDataMatrixBuilder { inner: new }
    }

    pub fn from_data(&self, data: Vec<f64>) -> PyResult<PyDataMatrix> {
        let dm = self.inner.clone().from_data(&data)
            .map_err(|msg| PyErr::new::<PyValueError, _>(msg.to_string()));
        Ok(PyDataMatrix::from_datamatrix(dm?))
    }

    pub fn from_file(&self, filename: &str) -> PyResult<PyDataMatrix> {

        let dm = self.inner.clone().from_file(filename)
            .map_err(|msg| PyErr::new::<PyValueError, _>(msg.to_string()));
        Ok(PyDataMatrix::from_datamatrix(dm?))
    }
}
