use pyo3::prelude::*;

use datamatrix::DataMatrix;

#[pyclass(name = "DataMatrix")]
#[derive(Clone)]
pub struct PyDataMatrix {
    inner: datamatrix::DataMatrix,
}

impl PyDataMatrix {
    pub fn from_datamatrix(datamatrix: DataMatrix) -> Self {
        PyDataMatrix { inner: datamatrix }
    }
}

#[pymethods]
impl PyDataMatrix {
    pub fn nrows(&self) -> usize {
        self.inner.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.inner.ncols()
    }

    pub fn get(&self, i: usize, j: usize) -> Option<f64> {
        self.inner.get(i, j)
    }

    pub fn get_by_label(&self, row_label: &str, col_label: &str) -> Option<f64> {
        self.inner.get_by_label(row_label, col_label)
    }
}

#[pyfunction]
pub fn read_matrix(filename: &str, col_i: usize, col_j: usize, col_val: usize, make_symmetric: bool) -> PyResult<PyDataMatrix> {
    let dm = datamatrix::read_matrix(filename, col_i, col_j, col_val, make_symmetric)
        .map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()));
    Ok(PyDataMatrix::from_datamatrix(dm?))
}

#[pyfunction]
pub fn read_matrix_indexed(filename: &str, row_labels: usize, col_labels: usize,
                           row_idx: usize, col_idx: usize, col_val: usize, make_symmetric: bool) -> PyResult<PyDataMatrix> {
    let dm = datamatrix::read_matrix_indexed(filename, row_labels, col_labels, row_idx, col_idx, col_val, make_symmetric)
        .map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()));
    Ok(PyDataMatrix::from_datamatrix(dm?))
}

#[pyfunction]
pub fn read_column(filename: &str) -> PyResult<PyDataMatrix> {
    let dm = datamatrix::read_column(filename)
        .map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()));
    Ok(PyDataMatrix::from_datamatrix(dm?))
}
