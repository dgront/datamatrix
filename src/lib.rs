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
//! - [`read_matrix()`]: Reads a matrix from a file with three columns (row label, column label, value).
//! - [`read_column()`]: Reads a flat list of values forming a square matrix.
//! - [`read_matrix_indexed()`]: Reads a matrix from a file providing explicit indices along with labels.
//!
//! ## Error Handling
//!
//! All I/O operations and parsing procedures return a custom [`Error`] type, which provides
//! detailed feedback about issues encountered during file reading or parsing.
//!
//! ## Example
//!
//! ```no_run
//! use datamatrix::{read_matrix, DataMatrix, Error};
//!
//! # fn main() -> Result<(), Error> {
//! let matrix = read_matrix(
//!     "./path/to/file.txt",
//!     0, 1, 2, // columns: row label, column label, value
//!     true    // make symmetric
//! )?;
//!
//! let value = matrix.get_by_label("Alice", "Bob");
//! println!("{:?}", value);
//! # Ok(())
//! # }
//! ```
//!
//! ## License
//!
//! This project is licensed under the Apache 2.0 license.

mod errors;
pub use crate::errors::Error;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
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
    /// Results in an error if the data shape does not match the labels.
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

    /// Returns the row labels.
    pub fn row_labels(&self) -> &[String] {
        &self.row_labels
    }

    /// Returns the column labels.
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

/// Reads a labeled matrix from a file that stores each value in three columns.
///
/// The columns aof the file should provide: row_label, column_label, value.
///
/// labels are mapped to internal row/column indices.  If `make_symmetric` is true,
/// both (i, j) and (j, i) are set when a single line is read
///
/// # Arguments
///
/// * `filename` — path to the input file.
/// * `col_i` — column index (0-based) for row labels.
/// * `col_j` — column index for column labels.
/// * `col_val` — column index for values.
/// * `make_symmetric` — whether to mirror the matrix across the diagonal.
///
/// # Expected file format
///
/// Text file with space-separated columns:
/// ```text
/// # Comment lines are allowed
/// alice bob 1.2
/// bob john 2.4
/// ```
///
/// # Example
///
/// ```rust
/// use datamatrix::{read_matrix, Error};
/// # fn main() -> Result<(), Error> {
/// # let file_path = "./tests/test_files/three_columns_short.txt";
/// let matrix = read_matrix(
///     file_path,
///     0, 1, 2, // columns: row-key, col-key, value
///     true     // make symmetric
/// )?;
///
/// assert_eq!(matrix.nrows(), 3);
/// assert_eq!(matrix.get_by_label("alice", "bob"), Some(1.2));
/// assert_eq!(matrix.get_by_label("bob", "alice"), Some(1.2)); // symmetric
/// # Ok(())
/// # }
/// ```
pub fn read_matrix<P: AsRef<Path>>(filename: P, col_i: usize, col_j: usize, col_val: usize, make_symmetric: bool) -> Result<DataMatrix, Error> {

    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut label_to_index: HashMap<String, usize> = HashMap::new();
    let mut entries = Vec::new();
    let mut current_index = 0;
    let max_col = col_i.max(col_j).max(col_val);

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') { continue; }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() <= max_col {
            return Err(Error::NotEnoughColumns { line: line_num + 1, needed: max_col + 1, content: line.to_string()});
        }

        let key_i = parts[col_i].to_string();
        let key_j = parts[col_j].to_string();
        let value: f64 = parts[col_val]
            .parse()
            .map_err(|_| Error::ParseError { line: line_num + 1, content: line.to_string()})?;

        let i = *label_to_index.entry(key_i.clone()).or_insert_with(|| {
            let idx = current_index;
            current_index += 1;
            idx
        });

        let j = *label_to_index.entry(key_j.clone()).or_insert_with(|| {
            let idx = current_index;
            current_index += 1;
            idx
        });

        entries.push((i, j, value));
        if make_symmetric && i != j {
            entries.push((j, i, value));
        }
    }

    let n = current_index;
    let mut data = vec![vec![0.0; n]; n];
    for (i, j, value) in entries {
        data[i][j] = value;
    }

    // Build label vectors ordered by index
    let mut labels = vec!["".to_string(); n];
    for (label, &index) in &label_to_index {
        labels[index] = label.clone();
    }

    DataMatrix::new(data, labels.clone(), labels)
}

/// Reads a flat list of values forming a square matrix (row-wise order),
/// and auto-generates row and column labels.
///
/// Skips empty lines and comment lines starting with '#'.
///
/// # Example file
/// ```text
/// # Single-column square matrix
/// 1.1
/// 2.2
/// 3.3
/// 4.4
/// ```
/// -> forms 2x2 matrix: [[1.1, 2.2], [3.3, 4.4]]
///
/// # Example
///
/// ```rust
/// use datamatrix::{read_column, Error};
/// # fn main() -> Result<(), Error> {
/// # let file_path = "./tests/test_files/single_columns_short.txt";
/// let matrix = read_column(file_path)?;
/// assert_eq!(matrix.nrows(), 2);
/// assert_eq!(matrix.get_by_label("row0", "col1"), Some(2.2));
/// assert_eq!(matrix.get_by_label("row1", "col1"), Some(4.4));
/// # Ok(())
/// # }
/// ```
pub fn read_column<P: AsRef<Path>>(filename: P) -> Result<DataMatrix, Error> {

    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut values = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 1 {
            return Err(Error::TooManyColumns {
                line: line_num + 1,
                content: line.to_string(),
            });
        }

        let value: f64 = parts[0]
            .parse()
            .map_err(|_| Error::ParseError {
                line: line_num + 1,
                content: line.to_string(),
            })?;
        values.push(value);
    }

    let total = values.len();
    let n = (total as f64).sqrt() as usize;

    if n * n != total {
        return Err(Error::ParseError {
            line: 0,
            content: format!("Total number of values ({}) is not a perfect square", total),
        });
    }

    let mut data = Vec::with_capacity(n);
    for row in 0..n {
        let start = row * n;
        let end = start + n;
        data.push(values[start..end].to_vec());
    }

    let row_labels: Vec<String> = (0..n).map(|i| format!("row{}", i)).collect();
    let col_labels: Vec<String> = (0..n).map(|i| format!("col{}", i)).collect();

    DataMatrix::new(data, row_labels, col_labels)
}

macro_rules! parse_token {
    ($parts:expr, $idx:expr, $ty:ty, $line_num:expr, $line_content:expr) => {
        $parts[$idx].parse::<$ty>().map_err(|_| crate::Error::ParseError {
            line: $line_num,
            content: $line_content.to_string(),
        })
    };
}

/// Reads a matrix from a file where each line provides both labels and indexes for each value
///
/// Indices are used to place values, labels are used for naming rows and columns, respectively.
///
/// If `make_symmetric` is true, both (i, j) and (j, i) are set.
///
/// # Example file
/// ```text
/// # row_label col_label row_idx col_idx value
/// Alice Bob 0 1 1.5
/// Bob John 1 2 2.2
/// ```
/// Creates a labeled 3x3 matrix with "Alice", "Bob", "John" as labels.
///
/// # Example
///
/// ```rust
/// use datamatrix::{read_matrix_indexed, Error};
/// # fn main() -> Result<(), Error> {
/// # let path = "./tests/test_files/five_columns_short.txt";
/// let matrix = read_matrix_indexed(
///     path,
///     0, 1, 2, 3, 4, // row_labels, col_labels, row_idx, col_idx, value
///     true          // make symmetric
/// )?;
///
///
/// assert_eq!(matrix.nrows(), 3);
/// assert_eq!(matrix.get_by_label("Alice", "Bob"), Some(1.5));
/// assert_eq!(matrix.get_by_label("Bob", "Alice"), Some(1.5)); // symmetric
/// # Ok(())
/// # }
/// ```
pub fn read_matrix_indexed<P: AsRef<Path>>(filename: P, row_labels: usize, col_labels: usize,
            row_idx: usize, col_idx: usize, values_idx: usize, make_symmetric: bool) -> Result<DataMatrix, Error> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut max_idx = 0;
    let mut entries = Vec::new();
    let max_col = *[col_labels, row_labels, col_idx, row_idx, values_idx].iter().max().unwrap();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') { continue; }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() <= max_col {
            return Err(Error::NotEnoughColumns { line: line_num + 1, needed: max_col + 1, content: line.to_string()});
        }

        let i = parse_token!(parts, row_idx, usize, line_num + 1, line)?;
        let j = parse_token!(parts, col_idx, usize, line_num + 1, line)?;
        let value = parse_token!(parts, values_idx, f64, line_num + 1, line)?;
        let row_label = parts[row_labels].to_string();
        let col_label = parts[col_labels].to_string();

        max_idx = max_idx.max(i).max(j);
        entries.push((i, j, row_label.clone(), col_label.clone(), value));

        if make_symmetric && i != j {
            entries.push((j, i, col_label, row_label, value));
        }
    }

    let n = max_idx + 1;
    let mut data = vec![vec![0.0; n]; n];

    // Prepare label arrays
    let mut row_labels_vec = vec!["".to_string(); n];
    let mut col_labels_vec = vec!["".to_string(); n];

    for (i, j, row_label, col_label, value) in entries {
        data[i][j] = value;
        row_labels_vec[i] = row_label;
        col_labels_vec[j] = col_label;
    }

    DataMatrix::new(data, row_labels_vec, col_labels_vec)
}


