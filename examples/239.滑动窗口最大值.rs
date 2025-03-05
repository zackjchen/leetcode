/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut windows = BinaryHeap::new();
        let mut left = 0usize;
        let mut right = k as usize;
        let mut res = vec![];
        nums[left..right].iter().enumerate().for_each(|(i,x)| windows.push((*x,i)));
        res.push(windows.peek().unwrap().0);
        while right < nums.len()  {
            windows.push((nums[right],right));
            println!("{:?}", windows.peek().unwrap().1);
            println!("windows:{:?}", windows);
            while !windows.is_empty() && windows.peek().unwrap().1 <= right-k as usize {
                windows.pop();
            }
            let (cur_max,_) = *windows.peek().unwrap();
            res.push(cur_max);
            left += 1;
            right += 1;
        }
        res
    }
}
// @lc code=end

