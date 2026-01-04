/*
 * @lc app=leetcode.cn id=51 lang=rust
 *
 * [51] N 皇后
 */

// @lc code=start
impl Solution {

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![];
        
        // 棋盘作为temp存储
        let mut chessboard = vec![vec!['.'; n as usize]; n as usize];

        fn backtrace(res: &mut Vec<Vec<String>>, row: usize, chessboard: &mut Vec<Vec<char>>) {
            if row == chessboard.len() {
                let mut temp = vec![];
                for i in 0..chessboard.len() {
                    let s: String = chessboard[i].iter().collect();
                    temp.push(s);
                }
                res.push(temp.clone());
                return;
            }

            for col in 0..chessboard.len() {
                if is_valid(col, row, chessboard) {
                    chessboard[row][col] = 'Q';
                    backtrace(res, row + 1, chessboard);
                    chessboard[row][col] = '.';
                }
            }

        }
        backtrace(&mut res, 0, &mut chessboard);

        res
    }

}
// @lc code=end


fn is_valid(col: usize, row: usize, chessboard: &Vec<Vec<char>>) -> bool {
    
    // 竖直方向有没有皇后
    let mut i = 0 as usize ;
    while i < row { 
        if chessboard[i][col] == 'Q' { 
            return false; 
        }
        i += 1;
    }
    let mut i = row as i32 - 1;
    let mut j = col as i32 - 1;

    // 135度方向有没有皇后
    while i >= 0 && j >= 0 {
        if chessboard[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }

    // 45度方向有没有皇后
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