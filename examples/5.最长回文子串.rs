/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

use std::ops::Index;

// @lc code=start
impl Solution {
    fn palindrome(chars: &Vec<char>, mut i: isize, mut j: isize) -> String {
        let len = chars.len() as isize;
        while i >= 0 && j < len && chars[i as usize] == chars[j as usize] {
            i = i - 1;
            j = j + 1;
        }
        chars[(i+1) as usize  .. j as usize].iter().collect::<String>()
    }
    pub fn longest_palindrome(s: String) -> String {
        let mut res = String::new();
        let chars = s.chars().collect::<Vec<_>>();
        for i in 0..s.len() {
            let mut s1 = Solution::palindrome(&chars, i as isize, i as isize);
            let mut s2 = Solution::palindrome(&chars, i as isize, i as isize +1);
            let cur = if s1.len() < s2.len() {
                s2
            }else{
                s1
            };
            if res.len() < cur.len() {
                res = cur
            }
        }
        res
    }
}
// @lc code=end

