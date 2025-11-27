/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */


// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        use std::collections::HashMap;
        let map = HashMap::from([
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]);
        let mut res = vec![];
        let mut path = vec![];
        let len = digits.len();
        let digits = digits.chars().collect::<Vec<char>>();



        fn backtrace(map: &HashMap<char, &str>, digits: &Vec<char>, index: usize, len: usize, res: &mut Vec<String>, path:  &mut Vec<char>) {
            if path.len() == len {
                res.push(path.iter().collect());
                return;
            }
            if index >= digits.len() {
                return;
            }
            let ch = digits[index];
            for c in map.get(&ch).unwrap().chars() {
                path.push(c);
                backtrace(map, digits, index + 1, len, res, path);
                path.pop();
            }
        }

        backtrace(&map, &digits, 0, len, &mut res, &mut path);
        res
    }
}
// @lc code=end

