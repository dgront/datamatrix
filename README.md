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
