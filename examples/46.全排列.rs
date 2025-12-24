/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut temp = Vec::new();
        let mut used = vec![false; nums.len()];

        fn backtrace(result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>, used: &mut Vec<bool>) {
            if temp.len() == nums.len() {
                result.push(temp.clone());
            }

            for i  in 0..nums.len() {
                if !used[i] {
                    temp.push(nums[i]);
                    used[i] = true;
                }else {
                    // 使用过的就跳过
                    continue;
                }

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

