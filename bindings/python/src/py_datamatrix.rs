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

