/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 */

// @lc code=start
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        let mut used = vec![false; candidates.len()];
        fn backtrace(candidates: &Vec<i32>, target: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start_index: usize, used: &mut Vec<bool>) {
            if path.iter().sum::<i32>() == target{
                res.push(path.clone());
                return;
            }
            
            // let val = candidates[start_index];
            for (i, val) in candidates.iter().skip(start_index).enumerate(){
                let index = start_index + i;
                if path.iter().sum::<i32>() + val > target  || (i > 0 && candidates[index] == candidates[index-1] && !used[index -1]) {
                    continue;
                }
                path.push(*val);
                used[i] = true;
                backtrace(candidates, target, path, res, start_index + i + 1, used);
                used[i] = false;
                path.pop();
            }
        }

        let mut candidates = candidates;
        candidates.sort();
        backtrace(&candidates, target, &mut path, &mut res, 0, &mut used);
        res

    }
}
// @lc code=end

