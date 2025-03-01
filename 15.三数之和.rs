/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        println!("{:?}", nums);
        let mut res = vec![];
        
        for first in 0..nums.len() - 1 {
            let mut third = nums.len() - 1;
            // 第二次从头遍历的时候和上一次遍历过的数据要不一样
            // 例如 [-1,-1, 0, 1]
            // 第一个first = -1【index=0】, 遍历后second，third没有满足条件的话
            // 第二次first 的index=1的时候仍然是-1，再去找也没有满足条件的
            // 所以要跳过,
            if first != 0 && nums[first] == nums[first - 1] {
                continue;
            }
            for second in first + 1..nums.len() { 
                println!("=>{} {} {}", first, second, third);
    
                if second != first+1 && nums[second] == nums[second-1] {
                    continue;
                }
                while nums[first] + nums[second] + nums[third] > 0 && third > second {
                    third -= 1;
                }
                if nums[first] + nums[second] + nums[third] == 0 && third > second{
                    println!("{} {} {}", first, second, third);
                     res.push(vec![nums[first], nums[second], nums[third]]);
                }
            }
            
        }
        res
    }
}
// @lc code=end

