/*
 * @lc app=leetcode.cn id=437 lang=rust
 *
 * [437] 路径总和 III
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use lib::TreeNode;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        // 记录前缀和出现的次数
        let mut cnt = HashMap::new();
        cnt.insert(0, 1);
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i64, cnt: &mut HashMap<i64, i32>, mut curr_sum: i64) -> i32 {

            if let Some(node) = root {
                let node = node.borrow();
                curr_sum += node.val as i64;
                
                let mut result = 0;
                // 如果当前前缀和减去目标值在哈希表中存在，则说明找到了一个路径
                // 这里包含了相等的情况，所以默认插入了 (0, 1)
                if cnt.contains_key(&(curr_sum - target_sum )) {
                    result += cnt[&(curr_sum - target_sum)];
                }
                *cnt.entry(curr_sum).or_insert(0) += 1;
                result += dfs(&node.left, target_sum, cnt, curr_sum);
                result += dfs(&node.right, target_sum, cnt, curr_sum);
                *cnt.entry(curr_sum).or_insert(0) -= 1;

                result

            }else {
                0
            }
        }

        dfs(&root, target_sum, &mut cnt, 0)

    }
}
