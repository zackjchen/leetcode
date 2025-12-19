/*
 * @lc app=leetcode.cn id=93 lang=rust
 *
 * [93] 复原 IP 地址
 */

// @lc code=start
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = vec![];
        let mut path: Vec<String> = vec![];

        fn backtrack(s: &str, res: &mut Vec<String>, path: &mut Vec<String>, start: usize) {
            if start == s.len() {
                res.push(path.join("."));
            } 

            for i in start..s.len(){
                let cur = &s[start..=i];
                if cur.len() <= 3 && path.len() < 4 {
                    match cur.parse::<i64>() {
                        Ok(v) => if v <= 255 {path.push(v.to_string());} else {continue},
                        Err(_) => continue, // 3个字符，但是02这种0开头的数字不合法
                    }  

                } else {
                    continue;
                }
                backtrack(s, res, path, i + 1);
                path.pop();
            }


        }
        backtrack(&s, &mut res, &mut path, 0);
        res
    }
}
// @lc code=end

