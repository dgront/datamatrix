// #![cfg_attr(docsrs, feature(doc_cfg))]
// #![doc(html_root_url = "https://docs.rs/data-matrix/0.2.0")]

//! # Two dimensional array indexed by string labels
#![doc = include_str!("../README.rustdoc.md")]

mod datamatrix_builder;
mod errors;

pub use crate::errors::Error;
use crate::Error::IncorrectMatrixLabels;
pub use datamatrix_builder::DataMatrixBuilder;

/// A dense matrix of numeric values with labeled rows and columns.
#[derive(Debug, Clone)]
pub struct DataMatrix {
    /// Matrix data: values indexed by (row, column).
    data: Vec<Vec<f64>>,

    /// Row labels (index -> label).
    row_labels: Vec<String>,

    /// Column labels (index -> label).
    col_labels: Vec<String>,
}

impl DataMatrix {
    /// Creates a new DataMatrix from data and labels.
    ///
    /// Results in an error if the data shape does not match the labels. In daily work you might prefer
    /// to use [`DataMatrixBuilder`] to create a [`DataMatrix`] from a file or data.
    pub fn new(
        data: Vec<Vec<f64>>,
        row_labels: Vec<String>,
        col_labels: Vec<String>,
    ) -> Result<Self, Error> {
        if data.len() != row_labels.len() {
            return Err(IncorrectMatrixLabels {
                expected: row_labels.len(),
                actual: data.len(),
            });
        }
        if data.is_empty() || data[0].len() != col_labels.len() {
            return Err(IncorrectMatrixLabels {
                expected: col_labels.len(),
                actual: data[0].len(),
            });
        }

        Ok(Self {
            data,
            row_labels,
            col_labels,
        })
    }

    /// Returns the number of rows.
    pub fn nrows(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of columns.
    pub fn ncols(&self) -> usize {
        if let Some(first_row) = self.data.first() {
            first_row.len()
        } else {
            0
        }
    }

    /// Gets the matrix entry at (i, j).
    pub fn get(&self, i: usize, j: usize) -> Option<f64> {
        self.data.get(i).and_then(|row| row.get(j)).copied()
    }

    /// Gets the matrix entry by row and column label.
    pub fn get_by_label(&self, row_label: &str, col_label: &str) -> Option<f64> {
        let row_idx = self.row_labels.iter().position(|r| r == row_label)?;
        let col_idx = self.col_labels.iter().position(|c| c == col_label)?;
        self.get(row_idx, col_idx)
    }

    /// Returns the label of a row by its index.
    pub fn row_index(&self, label: &str) -> Option<usize> {
        self.row_labels.iter().position(|r| r == label)
    }

    /// Returns the label of a column by its index.
    pub fn col_index(&self, label: &str) -> Option<usize> {
        self.col_labels.iter().position(|r| r == label)
    }

    /// Returns the label of a row by its index.
    pub fn row_label(&self, index: usize) -> &String {
        &self.row_labels[index]
    }

    /// Returns the label of a column by its index.
    pub fn col_label(&self, index: usize) -> &String {
        &self.col_labels[index]
    }

    /// Returns the row labels.
    ///
    /// If the matrix is symmetric, the row labels are the same as the column labels.
    pub fn row_labels(&self) -> &[String] {
        &self.row_labels
    }

    /// Returns the column labels.
    ///
    /// If the matrix is symmetric, the row labels are the same as the column labels.
    pub fn col_labels(&self) -> &[String] {
        &self.col_labels
    }

    /// Access the raw matrix data.
    pub fn data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }

    /// Checks if the matrix is square.
    pub fn is_square(&self) -> bool {
        self.nrows() == self.ncols()
    }
}
