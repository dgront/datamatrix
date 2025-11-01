//! # datamatrix
//!
//! `datamatrix` provides a simple and efficient implementation of a two-dimensional matrix
//! of numeric values (`f64`), with support for labeled rows and columns.
//!
//! It is particularly useful for handling datasets where elements are naturally accessed
//! by meaningful names rather than numeric indices. In addition to direct matrix construction,
//! the crate offers utilities to read matrices from structured text files.
//!
//! ## Features
//!
//! - Storage of 2D numeric data with row and column labels.
//! - Indexing by position or by label.
//! - Construction of matrices from raw data.
//! - Reading matrices from the following text file formats:
//!   - Three-column format (row label, column label, value).
//!   - Single column of values (for square matrices).
//!   - Indexed format with explicit (row, column) indices and labels.
//! - Optional symmetric filling (automatically populating (i, j) and (j, i)).
//!
//! ## Core Structures
//!
//! - [`DataMatrix`]: Represents a dense 2D matrix with labeled rows and columns.
//!
//! ## Reading Matrices
//!
//! Use [`DataMatrixBuilder`] to create a [`DataMatrix`] from a given file of data. This builder can read:
//! - a file with three columns (row label, column label, value).
//! - a flat list of values in a single column, forming a square matrix.
//! - a matrix from a file providing explicit indices along with labels;
//!   five columns are used in that case: row label, column label, row index, column index, value
//! - `.csv` and `.tsv` file formats are supported as well.
//!
//! ## Error Handling
//!
//! All I/O operations and parsing procedures return a custom [`Error`] type, which provides
//! detailed feedback about issues encountered during file reading or parsing.
//!
//! ## Examples
//!
//! ### Read a three-column file
//!
//! Text file with space-separated columns:
//! ```text
//! # Comment lines are allowed
//! Alice Bob 1.2
//! Bob John 2.4
//! ```
//!
//! ```rust
//! use datamatrix::{DataMatrixBuilder, DataMatrix, Error};
//!
//! # fn main() -> Result<(), Error> {
//! # let path = "./tests/test_files/three_columns_short.txt";
//! let matrix = DataMatrixBuilder::new()
//!     .label_columns(1, 2)
//!     .data_column(3)
//!     .symmetric(true)
//!     .from_file(path)?;
//!
//! let value = matrix.get_by_label("Alice", "Bob");
//! println!("{:?}", value);
//! # assert_eq!(value, Some(1.2));
//! let value_again = matrix.get(0, 1);
//! println!("{:?}", value);
//! # assert_eq!(value, Some(1.2));
//! # Ok(())
//! # }
//! ```
//!
//! By default, DataMatrixBuilder expects labels to be in the first two columns and the data in the third.
//! The code above can be therefore shortened to:
//! ```rust
//! # use datamatrix::{DataMatrixBuilder, DataMatrix, Error};
//! # fn main() -> Result<(), Error> {
//! # let path = "./tests/test_files/three_columns_short.txt";
//! let matrix = DataMatrixBuilder::new().symmetric(true).from_file(path)?;
//! # let value = matrix.get_by_label("Alice", "Bob");
//! # assert_eq!(value, Some(1.2));
//! # Ok(())
//! # }
//! ```
//!
//! `DataMatrix` allows also to look up the label of a given row or column and vice versa:
//! ```rust
//! use datamatrix::{DataMatrixBuilder, DataMatrix, Error};
//!
//! # fn main() -> Result<(), Error> {
//! # let path = "./tests/test_files/three_columns_short.txt";
//! let matrix = DataMatrixBuilder::new().symmetric(true).from_file(path)?;
//! let row_label = matrix.row_label(1);
//! assert_eq!(row_label, "Bob");
//! let row_index = matrix.row_index(row_label);
//! assert_eq!(row_index, Some(1));
//! # Ok(())
//! # }
//! ```
//!
//! ## License
//!
//! This project is licensed under the Apache 2.0 license.

mod errors;
mod datamatrix_builder;

pub use datamatrix_builder::DataMatrixBuilder;
pub use crate::errors::Error;
use crate::Error::IncorrectMatrixLabels;

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
    pub fn new(data: Vec<Vec<f64>>, row_labels: Vec<String>, col_labels: Vec<String>) -> Result<Self, Error> {
        if data.len() != row_labels.len() {
            return  Err(IncorrectMatrixLabels{ expected: row_labels.len(), actual: data.len()})
        }
        if data.is_empty() || data[0].len() != col_labels.len() {
            return  Err(IncorrectMatrixLabels{ expected: col_labels.len(), actual: data[0].len()})
        }

        Ok(Self { data, row_labels, col_labels })
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
    pub fn row_index(&self, label: &str) -> Option<usize> { self.row_labels.iter().position(|r| r == label) }

    /// Returns the label of a column by its index.
    pub fn col_index(&self, label: &str) -> Option<usize> { self.col_labels.iter().position(|r| r == label) }

    /// Returns the label of a row by its index.
    pub fn row_label(&self, index: usize) -> &String { &self.row_labels[index] }

    /// Returns the label of a column by its index.
    pub fn col_label(&self, index: usize) -> &String { &self.col_labels[index] }

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
