impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut sqrs: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        for (row_index, row) in board.iter().enumerate() {
            for (col_index, &col) in row.iter().enumerate() {
                if col == '.' {
                    continue;
                }
                let sqr_index = (row_index / 3) * 3 + col_index / 3;
                if !rows[row_index].insert(col)
                    || !cols[col_index].insert(col)
                    || !sqrs[sqr_index].insert(col)
                {
                    return false;
                }
            }
        }
        true
    }
}
