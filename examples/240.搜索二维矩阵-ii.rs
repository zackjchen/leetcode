/*
 * @lc app=leetcode.cn id=240 lang=rust
 *
 * [240] 搜索二维矩阵 II
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut x = 0;
        let mut y = matrix[0].len() as i32 - 1;
        while x < matrix.len() && y >= 0 && matrix[x][y as usize] != target {
            if matrix[x][y as usize] > target {
                y -= 1;
            } else {
                x += 1;
            }
        }
        if x >= matrix.len() || y < 0 {
            false
        }else {
            true
        }
    }
}
// @lc code=end

