/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = *nums.iter().max().unwrap();
        let mut last = 0;
        println!("{:?}, {:?}", max, last);
        for e in nums {
            last = e.max(last + e);
            max = max.max(last);
        }
        max
    }
}
// @lc code=end

