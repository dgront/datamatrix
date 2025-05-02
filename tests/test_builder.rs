#[cfg(test)]
mod test_builder {
    use datamatrix::DataMatrixBuilder;

    #[test]
    fn from_three_columns() {
        let input = "./tests/test_files/three_columns_short.txt";
        let dm = DataMatrixBuilder::new()
            .symmetric(true)
            .from_file(input).expect(&format!("Cant read the input file: {}", input));
        assert_eq!(dm.nrows(), 3);
        assert_eq!(dm.get_by_label("Alice", "Bob"), Some(1.2));
        assert_eq!(dm.get_by_label("Bob", "Alice"), Some(1.2)); // symmetric
    }

    #[test]
    fn from_five_columns() {
        let input = "./tests/test_files/five_columns_short.txt";
        let dm = DataMatrixBuilder::new()
            .symmetric(true)
            .data_column(5)
            .index_columns(3, 4)
            .from_file(input).expect(&format!("Cant read the input file: {}", input));
        assert_eq!(dm.nrows(), 3);
        assert_eq!(dm.ncols(), 3);
        assert_eq!(dm.get_by_label("Alice", "Bob"), Some(1.5));
        assert_eq!(dm.get_by_label("Bob", "Alice"), Some(1.5)); // symmetric
        assert_eq!(dm.get_by_label("John", "Bob"), Some(2.2));
    }

    #[test]
    fn five_columns_csv() {
        let input = "./tests/test_files/cities_by_distance.csv";
        let dm = DataMatrixBuilder::new()
            .symmetric(true)
            .data_column(3)
            .separator(',')
            .skip_header(true)
            .index_columns(4, 5)
            .from_file(input).expect(&format!("Cant read the input file: {}", input));
        let err = dm.get_by_label("Tokyo","New York City").unwrap() -  10851.73;
        assert!(err.abs() < 0.0001);
        assert_eq!(dm.nrows(), 15);
    }

    #[test]
    fn from_single_column_labels() {
        let input = "./tests/test_files/single_column_short.txt";
        let labels = ["A", "B"].iter().map(|s|s.to_string()).collect();
        let dm = DataMatrixBuilder::new()
            .labels(labels)
            .data_column(1)
            .from_file(input).expect(&format!("Cant read the input file: {}", input));
        assert_eq!(dm.nrows(), 2);
        assert_eq!(dm.ncols(), 2);
        assert_eq!(dm.get_by_label("A", "B"), Some(2.2));
        assert_eq!(dm.get_by_label("B", "A"), Some(3.3)); // not symmetric!
    }
    #[test]
    fn from_data() -> Result<(), datamatrix::Error> {
        let data: [f64;9] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let dm = DataMatrixBuilder::new().from_data(&data)?;
        assert_eq!(dm.nrows(), 3);
        assert_eq!(dm.ncols(), 3);
        assert_eq!(dm.get_by_label("row-1", "col-2"), Some(1.0));
        assert_eq!(dm.get_by_label("row-2", "col-1"), Some(3.0)); // not symmetric!

        Ok(())
    }
}