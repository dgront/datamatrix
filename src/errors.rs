use thiserror::Error;

/// Custom error type for DataMatrix operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Row or column labels count does not match number of rows / columns
    #[error("The number of labels {expected} does not match the count anticipated from the data matrix {actual}")]
    IncorrectMatrixLabels { expected: usize, actual: usize},

    /// Line does not have enough columns.
    #[error("Line {line} does not contain enough columns (need at least {needed}): '{content}'")]
    NotEnoughColumns { line: usize, needed: usize, content: String},

    /// More than one column found when expecting single column input.
    #[error("Line {line} has too many columns when expecting single value: '{content}'")]
    TooManyColumns { line: usize, content: String},

    /// Parsing error at a line.
    #[error("Invalid value at line {line}: '{content}'")]
    ParseError { line: usize, content: String},

    /// Generic I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
