/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

use std::{cmp::max, collections::{HashMap, HashSet}};

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut windows = std::collections::HashMap::new();
        let s = s.chars().collect::<Vec<char>>();

        let mut left = 0;
        let mut right = 0;
        let mut length = 0;
        while right < s.len() {
            let c = s[right];
            right += 1;
            windows.entry(c).and_modify(|e|*e += 1).or_insert(1);
            while right > left && windows.get(&c).unwrap().to_owned() > 1 {
                let d = s[left];
                windows.entry(d).and_modify(|e|*e -= 1);
                left += 1;
            }
            length = core::cmp::max(length, right - left);

        }

        return length as i32;
    }
}
// @lc code=end

