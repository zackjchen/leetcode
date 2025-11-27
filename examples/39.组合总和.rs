/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 */

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        fn backtrace(path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, target: i32, candidates: &Vec<i32>, start: usize) {
            if  path.iter().sum::<i32>() == target {
                res.push(path.clone());
                return;
            }
            // 当第多少个数的时候跳过前面的，但是不跳过自己，因为可以重复使用
            for (i, val) in candidates.iter().skip(start).enumerate() {
                if path.iter().sum::<i32>() + val > target {
                    continue;
                }
                path.push(*val);
                backtrace(path, res, target, candidates, i);
                path.pop();
            }
            
        }
        backtrace(&mut path, &mut res, target, &candidates, 0);
        res
    }
}
// @lc code=end

