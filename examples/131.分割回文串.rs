/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 */

// @lc code=start
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut temp = Vec::new();
        backtrace(&mut result, &mut temp, &s, 0);
        result
    }

}
// @lc code=end


fn backtrace(result: &mut Vec<Vec<String>>, temp: &mut Vec<String>, s: &str, start: usize) {
    if start >= s.len() {
        result.push(temp.clone());
        return;
    }

    for i in start..s.len(){
        if is_palindrome(s[start..=i].as_ref()) {
            temp.push(s[start..=i].to_string());
        } else {
            continue;
        }
        backtrace(result, temp, s, i+1);
        temp.pop();
    }
}


fn is_palindrome(s: &str) -> bool {
    let mut l = 0;
    let mut r = s.len() - 1;
    let s = s.chars().collect::<Vec<_>>();
    while l < r{
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}