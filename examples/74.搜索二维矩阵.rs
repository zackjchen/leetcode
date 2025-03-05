/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for nums in matrix {
            if nums.binary_search(&target).is_ok(){
                return true;
            }
        }
        false
    }
    
}
// @lc code=end

