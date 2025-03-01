/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;
        let mut max_h = 0;
        while left < right {
            while height[left] <= max_h && left < right {
                left += 1;
            }
            while height[right] <= max_h && left < right {
                right -= 1;
            }
            let min_h = height[left].min(height[right]);
            let curr_area = min_h * (right - left) as i32;
            max_area = max_area.max(curr_area);
            max_h = min_h;

        }
        max_area
    }
}
// @lc code=end

