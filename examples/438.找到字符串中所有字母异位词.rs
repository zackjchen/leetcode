/*
 * @lc app=leetcode.cn id=438 lang=rust
 *
 * [438] 找到字符串中所有字母异位词
 */

// @lc code=start
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.chars().collect::<Vec<_>>();
        let mut target: HashMap<char, i32> = HashMap::new();
        let mut window = HashMap::new();
        let mut res = vec![];
        let mut left: usize = 0;
        p.chars().for_each(|x| {
            *target.entry(x).or_insert(0) += 1;
        });
        let tar_sum = target.values().fold(0, |acc,x|acc+x);
    
    
        for (i,x) in s.iter().enumerate() {
    
            if (i - left) < tar_sum as usize {
                *window.entry(*x).or_insert(0) += 1;
            }
    
            if equal(&target,&window ) {
                let s1: String = s[left..=i].iter().collect::<String>();
                // res.push(s1);
                res.push(left as i32);
            }
    
            while (i-left) >= (tar_sum-1) as usize  && left <= i {
                let c = s[left];
                left += 1;
                *window.entry(c).or_insert(0) -= 1;
    
            }
        }
        res
    }

}
// @lc code=end

fn equal(a: &HashMap<char, i32>, b: &HashMap<char, i32>) -> bool {
    for (k,v) in a.iter() {
        if b.get(k) != Some(v) {
            return false;
        }
    }
    true
}