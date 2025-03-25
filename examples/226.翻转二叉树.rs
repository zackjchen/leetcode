/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
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
use std::collections::VecDeque;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = VecDeque::new();
    
        if let Some(node) = root.clone() {
            stack.push_back(node);
        }else {
            return None;
        }
        while !stack.is_empty() {
            if let Some(node) = stack.pop_front(){
                let mut node_borrow = node.borrow_mut();
                let t = node_borrow.left.clone();
                node_borrow.left = node_borrow.right.clone();
                node_borrow.right = t;
    
                if let Some(left) = node_borrow.left.clone(){
                    stack.push_back(left);
                }
                if let Some(right) = node_borrow.right.clone(){
                    stack.push_back(right);
                }
            }
        }
        root
    }
}
// @lc code=end

