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

    pub fn row_index(&self, label: &str) -> Option<usize> { self.inner.row_index(label) }

    pub fn col_index(&self, label: &str) -> Option<usize> { self.inner.col_index(label) }

    pub fn row_label(&self, index: usize) -> &String{ self.inner.row_label(index)}

    pub fn col_label(&self, index: usize) -> &String { self.inner.col_label(index) }

    pub fn row_labels(&self) -> Vec<String> { self.inner.row_labels().to_vec() }

    pub fn col_labels(&self) -> Vec<String> { self.inner.col_labels().to_vec() }

    pub fn get_by_label(&self, row_label: &str, col_label: &str) -> Option<f64> {
        self.inner.get_by_label(row_label, col_label)
    }

    pub fn data(&self) -> Vec<Vec<f64>> {
        self.inner.data().clone()
    }
}

