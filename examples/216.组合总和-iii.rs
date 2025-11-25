/*
 * @lc app=leetcode.cn id=216 lang=rust
 *
 * [216] 组合总和 III
 */

// @lc code=start
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
            let mut res = vec![];
        let mut curr = vec![];

        fn backtrace(k: i32, n: i32, start: i32, curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if curr.len() as i32 == k {
                if curr.iter().sum::<i32>() == n{
                    res.push(curr.clone());
                }
                return;
            }

            for i in start..=9 {
                curr.push(i);
                backtrace(k, n, i+1, curr, res);
                curr.pop();
            }
        }

        backtrace(k, n, 1, &mut curr, &mut res);

        return res;
    }
}
// @lc code=end

