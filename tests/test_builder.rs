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
}