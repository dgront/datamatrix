use flate2::read;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::path::Path;

use crate::{DataMatrix, Error};

/// A builder for loading labeled matrices from plain text, CSV, or TSV files.
///
/// [`DataMatrixBuilder`] provides flexible configuration for how files are parsed:
/// - specify which columns contain row labels, column labels, and values,
/// - optionally specify explicit row and column indices (for 5-column formats),
/// - control the separator (space, comma, tab, etc.),
/// - skip header lines,
/// - control whether the matrix should be symmetric.
///
/// # Supported formats
/// - **Three-column format**: row label, column label, value
/// - **Five-column format**: row label, column label, row index, column index, value
/// - **Single-column format**: raw values for a square matrix (handled separately); requires labels provided by a user
///   with `DataMatrixBuilder::labels()`.
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
/// use data_matrix::{DataMatrixBuilder, Error};
/// # fn main() -> Result<(), Error> {
/// # let input_fname = "./tests/test_files/five_columns_short.txt";
/// let matrix = DataMatrixBuilder::new()
///     .label_columns(0, 1)    // columns 0 and 1: row and column labels
///     .index_columns(2, 3)    // columns 2 and 3: row and column indices
///     .data_column(4)         // column 4: value
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
/// use data_matrix::{DataMatrixBuilder, Error};
/// # fn main() -> Result<(), Error> {
/// # let input_fname = "./tests/test_files/three_columns_short.txt";
///
/// let matrix = DataMatrixBuilder::new()
///     .label_columns(0, 1)    // columns 0 and 1: row and column labels
///     .data_column(2)         // column 2: value
///     .separator(' ')         // whitespace separator
///     .symmetric(true)        // make symmetric
///     .skip_header(false)     // this is the default behaviour
///     .from_file(input_fname)?;
/// # assert_eq!(matrix.ncols(), 3);
/// # assert_eq!(matrix.nrows(), 3);
/// # Ok(())
/// # }
/// ```
///
/// # Notes
/// - Columns are indexed starting **from 0**
/// - field separator must be a single character (with an exception for `' '`, see below); if not given, the value will be inferred from the file extension,
///     e.g. `'\t'` for `.tsv`
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
    separator: Option<char>,
    symmetric: bool,
    skip_header: bool,
    labels: Option<Vec<String>>,
}

#[allow(clippy::new_without_default)]
impl DataMatrixBuilder {
    /// Creates just a new builder.
    ///
    /// Now use its methods to set up column indexes (e.g. [`label_columns()`](DataMatrixBuilder::label_columns)), then provide some data (e.g. [`from_file()`](DataMatrixBuilder::from_file))
    pub fn new() -> Self {
        Self {
            row_label_col: 0,
            col_label_col: 1,
            data_col: 2,
            row_idx_col: None,
            col_idx_col: None,
            separator: None,
            symmetric: false,
            skip_header: false,
            labels: None,
        }
    }

    /// Specifies which columns contain the row and column labels.
    ///
    /// Column indices are **0-based** (i.e., the first column is 0).
    ///
    /// # Arguments
    /// * `row` — Column number for row labels.
    /// * `col` — Column number for column labels.
    ///
    /// # Example
    /// ```rust
    /// use data_matrix::DataMatrixBuilder;
    /// let mut builder = DataMatrixBuilder::new();
    /// builder.label_columns(1, 2);
    /// ```
    pub fn label_columns(mut self, row: usize, col: usize) -> Self {
        self.row_label_col = row;
        self.col_label_col = col;
        self
    }

    /// Provides labels for the case when the input data is a single column.
    pub fn labels<I, S>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    /// Specifies which column contains the numeric value.
    ///
    /// Column index is **0-based**.
    pub fn data_column(mut self, val: usize) -> Self {
        self.data_col = val;
        self
    }

    /// Specifies which columns provide explicit row and column indices.
    ///
    /// Column indices are **0-based**.
    ///
    /// # Arguments
    /// * `row_idx` — Column number for the row index.
    /// * `col_idx` — Column number for the column index.
    ///
    /// # Example
    /// ```rust
    /// use data_matrix::DataMatrixBuilder;
    /// let mut builder = DataMatrixBuilder::new();
    /// builder.index_columns(3, 4);
    /// ```
    pub fn index_columns(mut self, row_idx: usize, col_idx: usize) -> Self {
        self.row_idx_col = Some(row_idx);
        self.col_idx_col = Some(col_idx);
        self
    }

    /// Sets the character used to separate fields in the input file.
    ///
    /// Common choices: `' '`, `','`, `'\t'`.
    pub fn separator(mut self, sep: char) -> Self {
        self.separator = Some(sep);
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

    /// Creates a new [`DataMatrix`] from a given 1D vector of data.
    ///
    /// This method is devised to turn a 1D column of numbers into a **square** (usually symmetrix)
    /// 2D [`DataMatrix`] object.
    /// Labels should be provided with [`labels()`](DataMatrixBuilder::labels) method,
    /// otherwise they will be automatically generated as `"row-{}", i + 1` and `col-{}", i + 1`
    /// for rows and columns, respectively.
    ///
    /// # Examples
    /// Creates a square matrix with automatically generated labels:
    ///
    /// ```rust
    /// use data_matrix::{DataMatrixBuilder, Error};
    /// # fn main() -> Result<(), Error> {
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let matrix = DataMatrixBuilder::new().from_data(&data).unwrap();
    /// assert_eq!(matrix.ncols(), 3);
    /// assert_eq!(matrix.get(0,0).unwrap(), 1.0);
    /// assert_eq!(matrix.row_label(0), "row-1");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Creates a square symmetric matrix with user-defined labels:
    ///
    /// ```rust
    /// use data_matrix::{DataMatrixBuilder, Error};
    /// # fn main() -> Result<(), Error> {
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let labels = ["data-1", "data-2", "data-3"];
    /// let matrix = DataMatrixBuilder::new().labels(labels).from_data(&data).unwrap();
    /// assert_eq!(matrix.ncols(), 3);
    /// assert_eq!(matrix.get(0,0).unwrap(), 1.0);
    /// assert_eq!(matrix.row_label(0), "data-1");
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn from_data(self, data: &[f64]) -> Result<DataMatrix, Error> {
        let len = data.len();
        let n = (len as f64).sqrt() as usize;
        if n * n != len {
            return Err(Error::WrongNumberOfData { n_data: len });
        }

        let (row_labels, col_labels) = match &self.labels {
            Some(given) => (given.clone(), given.clone()),
            None => {
                let rows = (0..n).map(|i| format!("row-{}", i + 1)).collect();
                let cols = (0..n).map(|i| format!("col-{}", i + 1)).collect();
                (rows, cols)
            }
        };

        let mut matrix = Vec::with_capacity(n);
        for i in 0..n {
            let start = i * n;
            let end = start + n;
            matrix.push(data[start..end].to_vec());
        }

        DataMatrix::new(matrix, row_labels, col_labels)
    }

    /// Loads the matrix from the given file path according to the current builder settings.
    pub fn from_file<P: AsRef<Path>>(self, filename: P) -> Result<DataMatrix, Error> {
        if let Some(ref labels) = self.labels {
            return self.read_one_column(filename, self.data_col, labels.clone());
        }

        let mut row_indexer = Indexer::new();
        let mut col_indexer = Indexer::new();

        let separator = match self.separator {
            None => guess_separator(&filename),
            Some(c) => c,
        };

        let lines = parse_plain(filename, separator, self.skip_header)?;
        // ---------- Build the label_to_index map if we have explicit entry indexing
        if let (Some(r_idx), Some(c_idx)) = (self.row_idx_col, self.col_idx_col) {
            for (line_no, parts) in lines.iter().enumerate() {
                let row_idx: usize = parts[r_idx].parse().map_err(|_| Error::ParseError {
                    line: line_no,
                    content: parts[r_idx].to_string(),
                })?;
                let col_idx: usize = parts[c_idx].parse().map_err(|_| Error::ParseError {
                    line: line_no,
                    content: parts[c_idx].to_string(),
                })?;
                row_indexer.add_explicit(&parts[self.row_label_col], row_idx);
                if self.symmetric {
                    row_indexer.add_explicit(&parts[self.col_label_col], col_idx);
                } else {
                    col_indexer.add_explicit(&parts[self.col_label_col], col_idx);
                }
            }
        } else {
            // ---------- Build the label_to_index map if we don't have explicit entry indexing
            for parts in &lines {
                row_indexer.add(&parts[self.row_label_col]);
                if self.symmetric {
                    row_indexer.add(&parts[self.col_label_col]);
                } else {
                    col_indexer.add(&parts[self.col_label_col]);
                }
            }
        }

        if self.symmetric {
            col_indexer = row_indexer.clone();
        }
        let mut data = vec![vec![0.0; col_indexer.max_index()]; row_indexer.max_index()];
        let row_labels = row_indexer.to_vec();
        let col_labels = col_indexer.to_vec();

        for (line_no, parts) in lines.into_iter().enumerate() {
            let i_row = row_indexer.index(&parts[self.row_label_col]);
            let j_col = col_indexer.index(&parts[self.col_label_col]);
            let value: f64 = parts[self.data_col]
                .parse()
                .map_err(|_| Error::ParseError {
                    line: line_no,
                    content: parts[self.data_col].to_string()
                })?;
            data[i_row][j_col] = value;
            if self.symmetric {
                data[j_col][i_row] = value;
            }
        }

        DataMatrix::new(data, row_labels, col_labels)
    }

    fn read_one_column<P: AsRef<Path>>(
        &self,
        filename: P,
        column: usize,
        labels: Vec<String>,
    ) -> Result<DataMatrix, Error> {
        let rows = parse_plain(filename, ' ', self.skip_header)?;
        let col_idx = column;

        let mut values = Vec::new();

        for (line_num, parts) in rows.into_iter().enumerate() {
            if col_idx >= parts.len() {
                return Err(Error::NotEnoughColumns {
                    line: line_num + 1,
                    needed: col_idx + 1,
                    content: format!("{:?}", parts),
                });
            }

            let value: f64 = parts[col_idx].parse().map_err(|_| Error::ParseError {
                line: line_num + 1,
                content: parts[col_idx].clone(),
            })?;

            values.push(value);
        }

        let n = labels.len();
        if n * n != values.len() {
            return Err(Error::ParseError {
                line: 0,
                content: format!(
                    "Expected {}² = {} values, but found {}",
                    n,
                    n * n,
                    values.len()
                ),
            });
        }

        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            let start = i * n;
            let end = start + n;
            data.push(values[start..end].to_vec());
        }

        DataMatrix::new(data, labels.clone(), labels)
    }
}

fn parse_plain<P: AsRef<Path>>(
    filename: P,
    separator: char,
    skip_header: bool,
) -> std::io::Result<Vec<Vec<String>>> {
    // --- read the file, possibly gzipped
    let reader = open_file(filename)?;

    let mut first_passed = false;
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        // skip the first line if this is a header
        if !first_passed && skip_header {
            first_passed = true;
            continue;
        }
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
struct Indexer {
    label_to_index: HashMap<String, usize>,
}

impl Indexer {
    fn new() -> Self {
        Self {
            label_to_index: HashMap::new(),
        }
    }

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
        *self
            .label_to_index
            .get(label)
            .expect("Label not found in indexer")
    }

    fn max_index(&self) -> usize {
        self.label_to_index.len()
    }

    fn to_vec(&self) -> Vec<String> {
        let mut result = vec!["".to_string(); self.label_to_index.len()];
        for (label, &idx) in &self.label_to_index {
            result[idx] = label.clone();
        }
        result
    }
}

/// Guess a field separator from the filename extension.
///
/// Supported (case-insensitive):
/// - `csv` → `,`
/// - `tsv`, `tab` → `\t`
/// - `psv` (pipe-separated) → `|`
/// - `ssv` (semicolon-separated) → `;`
///
/// Also handles compressed files like `data.csv.gz` (peels one layer).
///
/// By default returns ` ` (a space character) if the separator cannot be determined.
///
/// # Examples
/// ```rust,ignore
/// use std::path::Path;
///
/// assert_eq!(guess_separator("data.csv"), ',');
/// assert_eq!(guess_separator("data.TSV"), '\t');
/// assert_eq!(guess_separator("table.tab"), '\t');
/// assert_eq!(guess_separator("log.psv"), '|');
/// assert_eq!(guess_separator("semi.ssv"), ';');
/// assert_eq!(guess_separator("archive.csv.gz"), ','); // compressed
/// ```
fn guess_separator<P: AsRef<Path>>(path: P) -> char {
    let path = path.as_ref();

    // Get the likely data extension, handling a single compression suffix.
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => {
            let ext = ext.to_ascii_lowercase();
            match ext.as_str() {
                // Peel one compression layer and check the previous extension
                "gz" | "bz2" | "xz" | "zst" | "zip" => {
                    // file_stem() of "...csv.gz" is "....csv"
                    path.file_stem()
                        .and_then(|s| Path::new(s).extension())
                        .and_then(|e| e.to_str())
                        .map(|e| e.to_ascii_lowercase())
                        .unwrap_or_default()
                }
                other => other.to_string(),
            }
        }
        None => String::new(),
    };

    match ext.as_str() {
        "dat" => ' ',
        "csv" => ',',
        "tsv" | "tab" => '\t',
        "psv" => '|',
        "ssv" => ';',
        _ => ' ',
    }
}

/// This function can open a regular file or a gzipped one, as determined by the extension
/// of the input file name. A boxed reader to the content is returned.
///
/// The code has been copied from bioshell-io::utils
fn open_file<P: AsRef<Path>>(file_path: P) -> io::Result<Box<dyn BufRead>> {
    let path = file_path.as_ref();

    if path.as_os_str().is_empty() {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Couldn't open file: empty path",
        ));
    }
    let file = File::open(path)?;

    if file_path.as_ref().extension() == Some(OsStr::new("gz")) {
        Ok(Box::new(BufReader::with_capacity(
            128 * 1024,
            read::GzDecoder::new(file),
        )))
    } else {
        Ok(Box::new(BufReader::with_capacity(128 * 1024, file)))
    }
}
