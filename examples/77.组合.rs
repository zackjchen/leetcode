/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] 组合
 */

// @lc code=start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut curr: Vec<i32> = vec![];
        pub fn backtrace(n: i32, k: i32, start: i32, curr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if curr.len() as i32 == k {
                result.push(curr.clone());
                return;
            }
            for i in start..=n-(k-curr.len() as i32)+1 {
                curr.push(i);
                backtrace(n, k, i + 1, curr, result);
                curr.pop();
            }
        }

        let start = 1;
        backtrace(n, k, start, &mut curr, &mut result);
        result

    }

}
    
// @lc code=end

