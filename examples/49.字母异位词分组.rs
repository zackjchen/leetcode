/*
 * @lc app=leetcode.cn id=49 lang=rust
 *
 * [49] 字母异位词分组
 */

use std::{collections::HashMap, hash::{Hash,DefaultHasher}};

// @lc code=start
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs{
            let mut a = s.clone().into_bytes();
            a.sort();
            let t_str = String::from_utf8(a).unwrap();
            if map.contains_key(&t_str) {
                map.get_mut(&t_str).unwrap().push(s);
            }else {
                map.insert(t_str, vec![s]);
            }
        }
        let res = map.into_values().collect::<Vec<Vec<String>>>();
        res
    }
}
// @lc code=end

