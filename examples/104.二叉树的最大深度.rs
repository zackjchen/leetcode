/*
 * @lc app=leetcode.cn id=104 lang=rust
 *
 * [104] 二叉树的最大深度
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = std::collections::VecDeque::new();

        if root.is_none() {
            return 0;
        }
        let mut res = 0;
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size{
                if let Some(node) = queue.pop_front(){
                    if let Some(left) = node.borrow().left.clone(){
                        queue.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone(){
                        queue.push_back(right);
                    }
                }
            }
            res += 1;
        }
    
        res
    }
}
// @lc code=end

