import datamatrix

def read_one_column():
    dmatrix = datamatrix.read_column("../../../tests/test_files/single_columns_short.txt")
    assert dmatrix.nrows() == 2
    assert dmatrix.ncols() == 2
    assert dmatrix.get_by_label("row0", "col1") == 2.2
    
def read_matrix():
    dmatrix = datamatrix.read_matrix("../../../tests/test_files/three_columns_short.txt", 1, 2, 3, True)
    assert dmatrix.nrows() == 3
    assert dmatrix.ncols() == 3
    assert dmatrix.get_by_label("alice", "bob") == 1.2

if __name__ == "__main__":
    read_one_column()
    read_matrix()
