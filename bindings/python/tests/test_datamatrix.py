from datamatrix import DataMatrixBuilder


def read_five_column():
    dmatrix = (DataMatrixBuilder()
               .label_columns(1,2)
               .data_column(5)
               .index_columns(3,4)
               .symmetric(True)
               .from_file("../../../tests/test_files/five_columns_short.txt"))
    assert dmatrix.nrows() == 3
    assert dmatrix.ncols() == 3
    assert dmatrix.get_by_label("Bob", "Alice") == 1.5
    assert dmatrix.get_by_label("Alice", "Bob") == 1.5
    assert dmatrix.get_by_label("John", "Bob") == 2.2
    assert dmatrix.row_index("Alice") == 0
    assert dmatrix.row_index("Bob") == 1
    assert dmatrix.col_label(0) == "Alice"
    assert dmatrix.col_label(1) == "Bob"
    
    
    
def read_three_columns():
    dmatrix = (DataMatrixBuilder()
               .label_columns(1,2)
               .symmetric(True)
               .data_column(3)
               .from_file("../../../tests/test_files/three_columns_short.txt"))
    assert dmatrix.nrows() == 3
    assert dmatrix.ncols() == 3
    assert dmatrix.get_by_label("Alice", "Bob") == 1.2


def read_single_column():
    dmatrix = (DataMatrixBuilder()
               .data_column(1)
               .labels(["A","B"])
               .from_file("../../../tests/test_files/single_column_short.txt"))
    assert dmatrix.nrows() == 2
    
    
def read_cities_by_distance():
    dmatrix = (DataMatrixBuilder()
               .label_columns(1,2)
               .symmetric(True)
               .data_column(3)
               .index_columns(4,5)
               .separator(',')
               .skip_header(True)
               .from_file("../../../tests/test_files/cities_by_distance.csv"))
    assert dmatrix.nrows() == 15
    assert dmatrix.ncols() == 15
    
    tt_distance = dmatrix.get_by_label("Tokyo","Toronto")
    assert abs(tt_distance-10351.69) < 0.0001
    
    expected_cities = ["Tokyo", "Seoul", "Beijing", "Bangkok", "Singapore", "Paris", "Madrid", "Rome",
          "Berlin", "Warsaw", "Los Angeles", "Miami", "Chicago", "New York City", "Toronto"]
    actual_cities = dmatrix.row_labels()
    for (ci, cj) in zip(actual_cities, expected_cities):
        assert ci == cj
    


def random_data():
    data = [0, 1, 2, 3, 4, 5, 6, 7, 8]
    dmatrix = DataMatrixBuilder().from_data(data)
    assert dmatrix.nrows() == 3
    data_obj = dmatrix.data()
    assert data_obj == [[0,1,2], [3, 4, 5], [6, 7, 8]]
    
    
if __name__ == "__main__":
    read_five_column()
    read_three_columns()
    read_cities_by_distance()
    read_single_column()
    random_data()
    