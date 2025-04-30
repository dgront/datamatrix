use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::{DataMatrix, Error};

/// A builder for loading labeled matrices from plain text, CSV, or TSV files.
///
/// `DataMatrixBuilder` provides flexible configuration for how files are parsed:
/// - specify which columns contain row labels, column labels, and values,
/// - optionally specify explicit row and column indices (for 5-column formats),
/// - control the separator (space, comma, tab, etc.),
/// - skip header lines,
/// - control whether the matrix should be symmetric.
///
/// # Supported formats
/// - **Three-column format**: row label, column label, value
/// - **Five-column format**: row label, column label, row index, column index, value
/// - **Single-column format**: raw values for a square matrix (handled separately)
///
/// Lines starting with `#` are ignored as comments.
///
/// # Examples
///
/// ## Reading a 5-column file (e.g., `five_columns_short.txt`)
/// ```text
/// # Comment lines are allowed
/// Alice Bob 0 1 1.5
/// Bob John 1 2 2.2
/// ```
///
/// ```rust
/// use datamatrix::{DataMatrixBuilder, Error};
/// # fn main() -> Result<(), Error> {
/// # let input_fname = "./tests/test_files/five_columns_short.txt";
/// let matrix = DataMatrixBuilder::new()
///     .label_columns(1, 2)    // columns 1 and 2: row and column labels
///     .index_columns(3, 4)    // columns 3 and 4: row and column indices
///     .data_column(5)         // column 5: value
///     .separator(' ')         // whitespace separator
///     .symmetric(true)        // make symmetric
///     .from_file(input_fname)?;
/// # assert_eq!(matrix.ncols(), 3);
/// # assert_eq!(matrix.nrows(), 3);
/// # Ok(())
/// # }
/// ```
///
/// ## Reading a 3-column file (e.g., `three_columns_short.txt`)
/// ```text
/// # Comment lines are allowed
/// Alice Bob 1.2
/// Bob John 2.4
/// ```
///
/// ```rust
/// use datamatrix::{DataMatrixBuilder, Error};
/// # fn main() -> Result<(), Error> {
/// # let input_fname = "./tests/test_files/three_columns_short.txt";
///
/// let matrix = DataMatrixBuilder::new()
///     .label_columns(1, 2)    // columns 1 and 2: row and column labels
///     .data_column(3)         // column 3: value
///     .separator(' ')         // whitespace separator
///     .symmetric(true)        // make symmetric
///     .from_file(input_fname)?;
/// # assert_eq!(matrix.ncols(), 3);
/// # assert_eq!(matrix.nrows(), 3);
/// # Ok(())
/// # }
/// ```
///
/// # Notes
/// - Columns are indexed starting **from 1**
/// - `.separator(' ')`, `.separator(',')`, and `.separator('\\t')` are supported.
/// - when `' '` (a space) is used a separator, the builder splits by all white spaces, i.e.  `str.split_whitespace(&self)`
///   method is used
/// - `.symmetric(true)` ensures that if (i,j) is set, (j,i) will also be set automatically.
#[derive(Debug, Clone)]
pub struct DataMatrixBuilder {
    row_label_col: usize,
    col_label_col: usize,
    data_col: usize,
    row_idx_col: Option<usize>,
    col_idx_col: Option<usize>,
    separator: char,
    symmetric: bool,
    skip_header: bool,
}

impl DataMatrixBuilder {
    pub fn new() -> Self {
        Self {
            row_label_col: 0,
            col_label_col: 1,
            data_col: 2,
            row_idx_col: None,
            col_idx_col: None,
            separator: ' ',
            symmetric: false,
            skip_header: false,
        }
    }

    /// Specifies which columns contain the row and column labels.
    ///
    /// Column indices are **1-based** (i.e., the first column is 1).
    ///
    /// # Arguments
    /// * `row` — Column number for row labels.
    /// * `col` — Column number for column labels.
    ///
    /// # Example
    /// ```rust
    /// use datamatrix::DataMatrixBuilder;
    /// let mut builder = DataMatrixBuilder::new();
    /// builder.label_columns(1, 2);
    /// ```
    pub fn label_columns(mut self, row: usize, col: usize) -> Self {
        self.row_label_col = row - 1;
        self.col_label_col = col - 1;
        self
    }

    /// Specifies which column contains the numeric value.
    ///
    /// Column index is **1-based**.
    pub fn data_column(mut self, val: usize) -> Self {
        self.data_col = val - 1;
        self
    }

    /// Specifies which columns provide explicit row and column indices.
    ///
    /// Column indices are **1-based**.
    ///
    /// # Arguments
    /// * `row_idx` — Column number for the row index.
    /// * `col_idx` — Column number for the column index.
    ///
    /// # Example
    /// ```rust
    /// use datamatrix::DataMatrixBuilder;
    /// let mut builder = DataMatrixBuilder::new();
    /// builder.index_columns(3, 4);
    /// ```
    pub fn index_columns(mut self, row_idx: usize, col_idx: usize) -> Self {
        self.row_idx_col = Some(row_idx - 1);
        self.col_idx_col = Some(col_idx - 1);
        self
    }

    /// Sets the character used to separate fields in the input file.
    ///
    /// Common choices: `' '`, `','`, `'\t'`.
    pub fn separator(mut self, sep: char) -> Self {
        self.separator = sep;
        self
    }

    /// If set to `true`, the first line of the file should be skipped as a header.
    pub fn skip_header(mut self, if_header: bool) -> Self {
        self.skip_header = if_header;
        self
    }

    /// Sets whether the matrix should be treated as symmetric.
    ///
    /// If enabled, for every entry `(row, col, value)`, the symmetric entry `(col, row, value)`
    /// is automatically added.
    pub fn symmetric(mut self, if_symmetric: bool) -> Self {
        self.symmetric = if_symmetric;
        self
    }

    /// Loads the matrix from the given file path according to the current builder settings.
    pub fn from_file<P: AsRef<Path>>(self, filename: P)-> Result<DataMatrix, Error> {

        let mut row_indexer = Indexer::new();
        let mut col_indexer = Indexer::new();

        let lines = parse_plain(filename, self.separator, self.skip_header)?;
        // ---------- Build the label_to_index map if we have explicit entry indexing
        if let (Some(r_idx), Some(c_idx)) = (self.row_idx_col, self.col_idx_col) {
            let mut line_no = 0;
            for parts in &lines {
                let row_idx: usize = parts[r_idx].parse().map_err(|_| Error::ParseError { line: line_no, content: format!("{}", parts[r_idx]) })?;
                let col_idx: usize = parts[c_idx].parse().map_err(|_| Error::ParseError { line: line_no, content: format!("{}", parts[c_idx]) })?;
                row_indexer.add_explicit(&parts[self.row_label_col], row_idx);
                if self.symmetric {
                    row_indexer.add_explicit(&parts[self.col_label_col], col_idx);
                } else {
                    col_indexer.add_explicit(&parts[self.col_label_col], col_idx);
                }
                line_no += 1;
            }
        } else {    // ---------- Build the label_to_index map if we don't have explicit entry indexing
            for parts in &lines {
                row_indexer.add(&parts[self.row_label_col]);
                if self.symmetric { row_indexer.add(&parts[self.col_label_col]); }
                else { col_indexer.add(&parts[self.col_label_col]); }
            }
        }

        if self.symmetric {
            col_indexer = row_indexer.clone();
        }
        let mut data = vec![vec![0.0; col_indexer.max_index()]; row_indexer.max_index()];
        let row_labels = row_indexer.to_vec();
        let col_labels = col_indexer.to_vec();

        let mut line_no = 0;
        for parts in lines {
            let i_row = row_indexer.index(&parts[self.row_label_col]);
            let j_col = col_indexer.index(&parts[self.col_label_col]);
            let value: f64 = parts[self.data_col].parse().map_err(|_| Error::ParseError { line: line_no, content: format!("{}", &parts[self.data_col]) })?;
            data[i_row][j_col] = value;
            if self.symmetric {
                data[j_col][i_row] = value;
            }
            line_no += 1;
        }

        DataMatrix::new(data, row_labels, col_labels)
    }
}


fn parse_plain<P: AsRef<Path>>(filename: P, separator: char, skip_header: bool) -> Result<Vec<Vec<String>>, Error> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let mut first_passed = false;
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        // skip the first line if this is a header
        if !first_passed && skip_header { first_passed=true; continue }
        let parts: Vec<String> = if separator == ' ' {
            line.split_whitespace().map(|s| s.to_string()).collect()
        } else {
            line.split(separator).map(|s| s.to_string()).collect()
        };
        lines.push(parts);
    }
    Ok(lines)
}

#[derive(Clone)]
struct Indexer { label_to_index: HashMap<String, usize>, }

impl Indexer {
    fn new() -> Self { Self { label_to_index: HashMap::new(), } }

    fn add(&mut self, label: &str) -> usize {
        if let Some(&idx) = self.label_to_index.get(label) {
            idx
        } else {
            let idx = self.label_to_index.len();
            self.label_to_index.insert(label.to_string(), idx);
            idx
        }
    }

    fn add_explicit(&mut self, label: &str, idx: usize) {
        self.label_to_index.entry(label.to_string()).or_insert(idx);
    }

    fn index(&self, label: &str) -> usize {
        *self.label_to_index.get(label).expect("Label not found in indexer")
    }

    fn max_index(&self) -> usize { self.label_to_index.len() }

    fn to_vec(&self) -> Vec<String> {
        let mut result = vec!["".to_string(); self.label_to_index.len()];
        for (label, &idx) in &self.label_to_index {
            result[idx] = label.clone();
        }
        result
    }
}