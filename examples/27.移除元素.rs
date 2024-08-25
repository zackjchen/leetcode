/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut p = 0;
        for q in 0..nums.len() {
            if nums[q] != val{
                nums.swap(p, q);
                p += 1;
            }
        }
        p as i32
    }
}
// @lc code=end

