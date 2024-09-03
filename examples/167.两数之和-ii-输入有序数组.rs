/*
 * @lc app=leetcode.cn id=167 lang=rust
 *
 * [167] 两数之和 II - 输入有序数组
 */

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        loop {
            if numbers[i] + numbers[j] > target {
                j = j - 1;
            } else if numbers[i] + numbers[j] < target {
                i = i + 1;
            } else {
                return vec![i as i32 + 1, j as i32 + 1];
            }
        }
    }
}
// @lc code=end
