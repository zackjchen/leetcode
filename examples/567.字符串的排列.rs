/*
 * @lc app=leetcode.cn id=567 lang=rust
 *
 * [567] 字符串的排列
 */

use std::collections::HashMap;

// @lc code=start
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut window = std::collections::HashMap::new();
        let mut need = std::collections::HashMap::new();
        for c in s1.chars() {
            need.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        let (mut left, mut right) = (0, 0);
        let s2 = s2.chars().collect::<Vec<char>>();
        let mut valid = 0;

        while right < s2.len() {
            let c = s2[right];
            right += 1;

            // 更新窗口数据
            if need.contains_key(&c) {
                window.entry(c).and_modify(|e| *e += 1).or_insert(1);
                if window.get(&c) == need.get(&c) {
                    valid += 1;
                }
            }

            // println!("left: {}, right: {}, valid: {}, window: {:?}, need: {:?}", left, right, valid, window, need);
            // 判断左侧窗口是否需要收缩
            while right - left >= s1.len() {
                let d = s2[left];
                left += 1;

                // 判断是否找到了合法的子串
                if valid == need.len() {
                    return true;
                }
                // 更新窗口数据
                if need.contains_key(&d) {
                    if window.get(&d) == need.get(&d) {
                        valid -= 1;
                    }
                    window.entry(d).and_modify(|e| *e -= 1);
                }
            }
        }
        return false;
    }
}
// @lc code=end
