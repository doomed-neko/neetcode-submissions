impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut sqrs = [0u16; 9];
        for row_index in 0..9 {
            for col_index in 0..9 {
                let ch = board[row_index][col_index];
                if ch == '.' {
                    continue;
                }
                let col = ch as u16;
                let bit = 1 << col;
                let sqr_index = (row_index / 3) * 3 + col_index / 3;
                if rows[row_index] & bit != 0
                    || cols[col_index] & bit != 0
                    || sqrs[sqr_index] & bit != 0
                {
                    return false;
                }
                rows[row_index] |= bit;
                cols[col_index] |= bit;
                sqrs[sqr_index] |= bit;
            }
        }
        true
    }
}
