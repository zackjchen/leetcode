/*
 * @lc app=leetcode.cn id=37 lang=rust
 *
 * [37] 解数独
 */

// @lc code=start
impl Solution {

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {

        fn is_valid(row: usize, col: usize, val: char, board: & Vec<Vec<char>>) -> bool{
            // row行检查
            if board[row][col] != '.' {
                return true;
            }
            for i in 0..board.len() {
                if board[row][i] == val {
                    return false;
                }
            }
            // col列检查
            for i in 0..board.len() {
                if board[i][col] == val {
                    return false;
                }
                
            }

            // 3x3小方格检查
            let start_row = (row / 3) * 3;
            let start_col = (col / 3) * 3;
            for i in start_row..start_row + 3 {
                for j in start_col..start_col + 3 {
                    if board[i][j] == val {
                        return false;
                    }
                }
            }

            return true;
        }

        fn backtrace(board: &mut Vec<Vec<char>>) -> bool{
            // 结束条件

            for i in 0..9 {
                for j in 0..9 {
                    if board[i][j] != '.'{
                        continue;
                    }
                    for val in '1'..='9' {
                        if is_valid(i, j, val, board) {
                            board[i][j] = val;
                            if backtrace(board){
                                return true;
                            }
                            board[i][j] = '.';
                        }
                    }
                    return false;
                }
            }
            true
        }

        backtrace(board);
    }
}
// @lc code=end

