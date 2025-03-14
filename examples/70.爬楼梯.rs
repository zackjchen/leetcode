/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut res = 1; // n == 0 时的情况, 1种方法
        let mut first = 0;
        let mut second = 0;
        for _ in 0..n {
            first = second;
            second = res;
            res = first + second;
        }
    }
}
// @lc code=end

