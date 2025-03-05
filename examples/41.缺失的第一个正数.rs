/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 * 思路，将数组中的元素放到对应值-1的位置，然后遍历数组，找到第一个不符合的元素
 */

// @lc code=start
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut p = 0;
        while p < l {
            let t = nums[p];
            // 小于等于0，当前位置已经放对，超过数组长度，重复的值，不用处理
            if t <= 0 || t-1 == p as i32 || t >= l as i32 || nums[(t-1) as usize] == t {
                p += 1;
                continue;
            }
            nums.swap(t as usize-1, p);
    
        }
        for (i,n) in nums.iter().enumerate() {
            if *n - 1 != i as i32 {
                return (i+1) as i32;
            }        
        }
        (l + 1) as i32
    }
}
// @lc code=end

