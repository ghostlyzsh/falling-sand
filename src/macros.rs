#[macro_export]
macro_rules! index_matrix {
    // x, y
    ($col:expr, $row:expr) => {
        (($row * MATRIX_COLS) + $col) as usize
    };
}
