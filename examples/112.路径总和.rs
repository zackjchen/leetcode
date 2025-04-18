/*
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * [112] 路径总和
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
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            let target_sum = target_sum - node.val;
            if node.left.is_none() && node.right.is_none() && target_sum == 0  {
                return  true;   
            }

            return Self::has_path_sum(node.left.take() , target_sum) ||
                   Self::has_path_sum(node.right.take(), target_sum);
        }
        false
    }
}
// @lc code=end

