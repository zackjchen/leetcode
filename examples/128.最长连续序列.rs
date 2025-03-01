/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut max_len = 0;
        for n in set.iter(){
            let curr = *n;
            let mut current_len = 1;
            if set.contains(&(curr-1)) {
                continue;
            }
            while set.contains(&(curr + current_len)){
                current_len += 1;
            }
            if current_len > max_len {
                max_len = current_len;
            }
        }
        max_len
    }
}
// @lc code=end

