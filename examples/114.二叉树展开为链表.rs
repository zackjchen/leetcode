/*
 * @lc app=leetcode.cn id=114 lang=rust
 *
 * [114] 二叉树展开为链表
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = std::collections::VecDeque::new();
        match root.clone() {
            Some(node) => stack.push_back(node),
            None => return,
        }
        while let Some(node) = stack.pop_back() {
            if let Some(right) = node.borrow_mut().right.take() {
                stack.push_back(right.clone());
            }
            if let Some(left) = node.borrow_mut().left.take() {
                stack.push_back(left.clone());
            }
    
            if !stack.is_empty() {
                // cur_node.right = next_node
                node.borrow_mut().right = stack.back().cloned();
            }
        }
    }
}
// @lc code=end

