/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut temp = Vec::new();

        fn backtrace(result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
            result.push(temp.clone()); 

            for i in start..nums.len() {
                temp.push(nums[i]);
                backtrace(result, temp, nums, i + 1);
                temp.pop();
            }
        }

        backtrace(&mut result, &mut temp, &nums, 0);
        result

    }
}
// @lc code=end

