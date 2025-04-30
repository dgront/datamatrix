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
    
def read_three_columns():
    dmatrix = (DataMatrixBuilder()
               .label_columns(1,2)
               .symmetric(True)
               .data_column(3)
               .from_file("../../../tests/test_files/three_columns_short.txt"))
    assert dmatrix.nrows() == 3
    assert dmatrix.ncols() == 3
    assert dmatrix.get_by_label("Alice", "Bob") == 1.2

if __name__ == "__main__":
    read_five_column()
    read_three_columns()
