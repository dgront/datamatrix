# datamatrix

**datamatrix** provides a lightweight Rust implementation of labeled numerical matrices.
It allows reading tabular data from text files (CSV, TSV, space-separated, etc.), attaching
labels to rows and columns, and performing label-based lookup or symmetric completion.

---

## Features

- Simple and expressive **builder API** for constructing matrices:
  ```rust
  use datamatrix::DataMatrixBuilder;

  let dm = DataMatrixBuilder::new()
      .index_columns(0, 1)          // 0-based column indices for row and column labels
      .data_column(2)               // numeric data column
      .separator(',')               // optional; inferred from file extension if omitted
      .symmetric(true)              // fill both (i,j) and (j,i)
      .from_file("data/example.csv")?;

  println!("Matrix shape: {} × {}", dm.nrows(), dm.ncols());
  println!("Value at (A,B): {:?}", dm.get_by_label("A", "B"));
  ```

  - By-label and by-index access to numerical values:
  ```rust
  let val_ab = dm.get_by_label("Alice", "Bob");
  let val_ij = dm.get(0, 1);
  ```

  - Reads compressed input:
  ```rust
  let dm = DataMatrixBuilder::new()
      .from_file("test_files/cities_by_distance.csv.gz")?;
  ```


## Installation
```toml
[dependencies]
datamatrix = "0.2"
```

## Example
The following input file:

| gene | sample | value |
| ---- | ------ | ----- |
| G1   | S1     | 0.81  |
| G1   | S2     | 0.93  |
| G2   | S1     | 0.72  |
| G2   | S2     | 1.00  |

can be processed as follows:
```rust
let m = DataMatrixBuilder::new()
    .index_columns(0, 1)
    .data_column(2)
    .from_file("gene_expression.csv")?;
println!("{:?}", m.get_by_label("G1", "S2"))
```

single column, three-column and five-column input files are supported. Alternatively, a `DataMatrix` struct can be created from raw data.


## Python library
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

## License
Licensed under Apache License, Version 2.0 (LICENSE-APACHE https://www.apache.org/licenses/LICENSE-2.0)
