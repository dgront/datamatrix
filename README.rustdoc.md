
**datamatrix** provides a lightweight and efficient Rust implementation of a two-dimensional matrix of numeric values (f64) with *labeled rows and columns*. It is particularly suited for datasets where elements
are naturally accessed by meaningful names rather than numeric indices. In addition to in-memory construction,
the crate offers utilities to read matrices directly from structured text files.


## Features

- Storage of 2D numeric data with row and column labels.
- Indexing by position or by label.
- Simple and expressive **builder API** for constructing matrices:
- Reading from the following text file formats:
    - Three-column format: (row_label, column_label, value).
    - Single column of values: for square matrices.
    - Indexed format: explicit row/column indices with labels.

- Optional symmetric filling, automatically populating both (i, j) and (j, i) for symmetric data (e.g., distances or correlations).
- Transparent reading of compressed files (.gz, .bz2, .xz).


## Example
The following [`example_input.tsv`](https://github.com/dgront/datamatrix/blob/master/tests/test_files/example_input.tsv) input file with 3 columns:

| gene | sample | value |
| ---- | ------ | ----- |
| G1   | S1     | 0.81  |
| G1   | S2     | 0.93  |
| G2   | S1     | 0.72  |
| G2   | S2     | 1.00  |

can be loaded with the code given below:
```rust
use data_matrix::{DataMatrixBuilder, Error};
# fn main() -> Result<(), Error> {
let dm = DataMatrixBuilder::new()
      .label_columns(0, 1)          // 0-based column indexes for row and column labels
      .data_column(2)               // numeric data column
      .separator('\t')              // optional; inferred from file extension if omitted
      .symmetric(false)             // this is the default behaviour
      .skip_header(true)
      .from_file("./tests/test_files/example_input.tsv")?;
println!("Matrix shape: {} × {}", dm.nrows(), dm.ncols());
// access by labels
println!("Value at (G1,S1): {:?}", dm.get_by_label("G1", "S1"));
// access by indexes
println!("Value at [0,1]: {:?}",  dm.get(0, 1));
# Ok(()) }
```

By default, DataMatrixBuilder expects labels to be in the first two columns and the data in the third.
The code above can be therefore shortened to:

```rust
use data_matrix::{DataMatrixBuilder, Error};
# fn main() -> Result<(), Error> {
let matrix = DataMatrixBuilder::new().skip_header(true).from_file("./tests/test_files/example_input.tsv")?;
let value = matrix.get_by_label("G1", "S1");
# Ok(()) }
```
Single column, three-column and five-column input files are supported. Alternatively, a `DataMatrix` struct can be created from raw data.

## Installation
Add the following line to your `Cargo.toml` file an let `cargo` do the rest

```toml
[dependencies]
datamatrix = "0.2"
```

# Python package
The project provides also Python bindings to the datamatrix crate, which allows to use it in Python scripts as below:

```Python
from datamatrix import DataMatrixBuilder

dmatrix = (DataMatrixBuilder()
    .label_columns(0, 1)
    .data_column(4)
    .index_columns(2, 3)
    .symmetric(True)
    .from_file("../../../tests/test_files/five_columns_short.txt"))
assert dmatrix.ncols() == 3
assert dmatrix.get_by_label("Bob", "Alice") == 1.5
```

## Compilation
You need **maturin** to compile the datamatrix Python module, which runs in a virtual environment You can use the `requirements.txt` file provided in `./bindings/python` to ease the installation:

### Set up a virtual environment and install build deps

```bash
cd bindings/python

# Create & activate a virtual environment - only once
python3 -m venv .venv-maturin
# Activate the virtual environment - before compilation
# Linux/macOS:
source .venv-maturin/bin/activate
# (Windows PowerShell:)
# .venv\Scripts\Activate.ps1

# Upgrade pip and install maturin (and any other build deps)
pip install -U pip
pip install -r requirements.txt
```

### Build and install locally for development
This compiles the Rust extension and installs the Python package into the active venv:
```bash
maturin develop --release
# Test if it works
python -c "import datamatrix; print(datamatrix.__doc__[:60])"
```

### Build distribution wheels (for packaging/upload)
Build wheels into `target/wheels/`:
```bash
maturin build --release
```

## License
Licensed under Apache License, Version 2.0 (LICENSE-APACHE https://www.apache.org/licenses/LICENSE-2.0)
