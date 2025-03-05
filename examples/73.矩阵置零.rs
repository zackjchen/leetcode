/*
 * @lc app=leetcode.cn id=73 lang=rust
 *
 * [73] 矩阵置零
 */

// @lc code=start
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row = vec![1;matrix.len()];
        let mut col = vec![1;matrix[0].len()];
        for i in 0..matrix.len(){
            println!("matrix[0].len():{}",matrix[0].len());
            for j in 0..matrix[0].len(){
                println!("i={},j={}",i,j);
                if matrix[i][j] == 0{
                    row[i] = 0;
                    col[j] = 0;
                }
            }
        }
        for i in 0..row.len(){
            for j in 0..col.len(){
                if row[i] == 0 || col[j] == 0{
                    matrix[i][j] = 0;
                };
            }
        }
    }
}
// @lc code=end

