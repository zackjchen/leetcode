/*
 * @lc app=leetcode.cn id=47 lang=rust
 *
 * [47] 全排列 II
 */

// @lc code=start
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut result = Vec::new();
        let mut temp = Vec::new();
        let mut used = vec![false; nums.len()];

        fn backtrace(result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>, used: &mut Vec<bool>) {
            if temp.len() == nums.len() {
                result.push(temp.clone());
            }
            for i  in 0..nums.len() {
                if used[i] || (i > 0 && nums[i] == nums[i-1] && used[i-1]) {
                    continue;
                }
                temp.push(nums[i]);
                used[i] = true;
                backtrace(result, temp, &nums, used);
                temp.pop();
                used[i] = false;
            }
        }
        backtrace(&mut result, &mut temp, &nums, &mut used);
        result
    }
}
// @lc code=end

