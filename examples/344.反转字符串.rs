/*
 * @lc app=leetcode.cn id=344 lang=rust
 *
 * [344] 反转字符串
 */

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut i,mut j) = (0, s.len()-1);
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}
// @lc code=end

