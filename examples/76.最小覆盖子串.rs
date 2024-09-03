/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 */

// @lc code=start
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut valid = 0;
        let mut len = std::usize::MAX;
        let mut start = 0;
        let (mut i, mut j) = (0, 0);
        let mut windows = std::collections::HashMap::new();
        let mut need = std::collections::HashMap::new();
        for c in t.chars() {
            *need.entry(c).or_insert(0) += 1;
        }

        while j < s.len() {
            let c = s[j];
            j += 1;
            if need.contains_key(&c) {
                *windows.entry(c).or_insert(0) += 1;
                if windows.get(&c) == need.get(&c) {
                    // TODO
                    valid += 1;
                }
            }
            while valid == need.len() {
                // TODO
                // 更新最小覆盖子串
                // d 是将移出窗口的字符
                let d = s[i];
                if len > j - i {
                    len = j - i;
                    start = i;
                }
                i += 1;
                if need.contains_key(&d) {
                    if windows.get(&d) == need.get(&d) {
                        valid -= 1;
                    }
                    windows.entry(d).and_modify(|x| *x -= 1);
                }
            }
        }
        return if len == std::usize::MAX {
            "".to_string()
        } else {
            s[start..start + len].iter().collect::<String>()
        };
    }
}
// @lc code=end
