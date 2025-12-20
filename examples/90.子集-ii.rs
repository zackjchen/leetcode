/*
 * @lc app=leetcode.cn id=90 lang=rust
 *
 * [90] 子集 II
 */

// @lc code=start
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // 先排序，方便跳过重复元素

        let mut result = Vec::new();
        let mut temp = Vec::new();
        fn backtrace(result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {

            result.push(temp.clone());
        
            for i in start..nums.len(){
                // i > start 保证了同一层级不会重复使用相同元素
                if i > start && nums[i] == nums[i - 1] {
                    continue; // 跳过重复元素
                }
                temp.push(nums[i]);
                backtrace(result, temp, nums, i+1);
                temp.pop();
            }
        }

        backtrace(&mut result, &mut temp, &nums, 0);
        result

    }
}
// @lc code=end

