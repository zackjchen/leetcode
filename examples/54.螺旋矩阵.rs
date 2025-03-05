/*
 * @lc app=leetcode.cn id=54 lang=rust
 *
 * [54] 螺旋矩阵
 */

// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        let mut top = 0;
        let mut bottom = matrix.len() -1 ;
        let mut left = 0;
        let mut right = matrix[0].len() -1;
    
        while top<= bottom && left <= right  {
            println!("top={},bottom={},left={},right={}", top,bottom,left,right);
    
            for i in left..=right {
                res.push(matrix[top][i]);
            }
            for i in top+1..=bottom {
                res.push(matrix[i][right]);
            }
            if top < bottom && left < right {
                for i in (left+1..=(right-1)).rev() {
                    res.push(matrix[bottom][i]);
                }
                for i in (top+1..=bottom).rev() {
                    res.push(matrix[i][left]);
                }
            }
    
            if top == bottom || left == right {
                break;
            }
            top += 1;
            bottom -= 1;
            left += 1;
            right -= 1;
        }
    
        res
    }
}
// @lc code=end

