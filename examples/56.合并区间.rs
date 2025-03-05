/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|x| x[0]);
        let mut last = intervals.remove(0);
        let mut res = vec![];
        
        for arr in  intervals {
            if arr[0] <= last[1]  {
                if last[1] >= arr[1] {
                    continue
                }else {
                    last[1] = arr[1];
                }
            } else {
                res.push(last);
                last = arr;
            }
            
        }
        res.push(last);
        res
    }
}
// @lc code=end

