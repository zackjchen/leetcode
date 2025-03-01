use std::{collections::HashSet, mem::swap};

fn main() {
    
    let nums = vec![-4, -1, -1, 0, 1, 2];
    let nums = vec![-1,0,1,2,-1,-4,-2,-3,3,0,4]

    ;
    // let nums = vec![-2,-1,1,2]    ;
    let res = three_sum( nums);
    println!("{:?}", res);
}
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    println!("{:?}", nums);
    let mut res = vec![];

    for first in 0..nums.len() - 1 {
        let mut third = nums.len() - 1;

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