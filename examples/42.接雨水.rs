impl Solution {
    /// 双指针法接雨水
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0i32;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;


        while left < right{
            left_max = left_max.max(height[left]);
            right_max = right_max.max(height[right]);

            if height[left] < height[right]{
                res += left_max - height[left];
                left += 1;
            }else {
                res += right_max - height[right];
                right -= 1;
            }
        }

        res
    }
}