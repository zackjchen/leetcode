/*
 * @lc app=leetcode.cn id=238 lang=rust
 *
 * [238] 除自身以外数组的乘积
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());
        let mut prefix = 1;
        for n in nums.iter() {
            res.push(prefix);
            prefix = prefix * n;
        }
        prefix = 1;
        for i in (0..nums.iter().len()).rev() {
            res[i] = res[i] * prefix;
            prefix = prefix * nums[i];
    
        }
        res
    }
}
// @lc code=end

