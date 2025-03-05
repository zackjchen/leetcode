/*
 * @lc app=leetcode.cn id=189 lang=rust
 *
 * [189] 轮转数组
 * 1 2 3 4 5 6 7
 * 5 6 7 1 2 3 4
 * 先把后k个取出来临时存储，然后把后面的元素往后移动k个位置，最后把临时存储的元素放到前面
 */

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        // 如果k大于数组长度，取余数
        let k = (k % l as i32) as usize;
        if k == 0 {
            return;
        }
        let arr: Vec<i32> = Vec::from(&nums[(l-k)%l..]);
        for i in (k..l).rev() {
            nums[i] = nums[(i-k)%l];
        }
        nums[..k].copy_from_slice(arr.as_slice());
    }
}
// @lc code=end

