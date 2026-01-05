/*
 * @lc app=leetcode.cn id=52 lang=rust
 *
 * [52] N 皇后 II
 */

// @lc code=start
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut count = 0;
        let mut chessboard = vec![vec!['.'; n as usize]; n as usize];

        fn is_valid(row: usize, col: usize, chessboard: &Vec<Vec<char>>) -> bool {
            let row = row as i32 - 1;
            for i in 0..row {
                if chessboard[i as usize][col] == 'Q' {
                    return false;
                }
            }

            let mut i = row as i32 - 1;
            let mut j = col as i32 - 1;
            while i >= 0 && j >= 0 {
                if chessboard[i as usize][j as usize] == 'Q' {
                    return false;
                }
                i -= 1;
                j -= 1;
            }

            i = row as i32 - 1;
            j = col as i32 + 1;
            while j  < chessboard.len() as i32 && i >= 0 {
                if chessboard[i as usize][j as usize] == 'Q' {
                    return false;
                }
                i -= 1;
                j += 1;
            }

            true
        }

        fn backtrace(count: &mut i32, row: usize, chessboard: &mut Vec<Vec<char>>) {
            if row == chessboard.len() {
                *count += 1;
                return;
            }

            for col in 0..chessboard.len() {
                if is_valid(row, col, chessboard) {
                    println!("Placing Q at row {}, col {}", row, col);
                    chessboard[row][col] = 'Q';
                    backtrace(count, row + 1, chessboard);
                    chessboard[row][col] = '.';
                }
            }
        }

        backtrace(&mut count, 0, &mut chessboard);
        count

    }
}
// @lc code=end

