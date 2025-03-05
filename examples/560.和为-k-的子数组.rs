/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为 K 的子数组
 */

// @lc code=start
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut pre_sum = 0;
        let mut count = 0;
        map.insert(0, 1);
        for e in nums.iter() {
            pre_sum += e;
            if map.get(&(pre_sum - k)).is_some() {
                count += map.get(&(pre_sum - k)).unwrap();
            }
            map.entry(pre_sum).and_modify(|x| *x += 1).or_insert(1);
        }
        count
    }
}
// @lc code=end

