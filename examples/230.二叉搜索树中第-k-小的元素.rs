/*
 * @lc app=leetcode.cn id=230 lang=rust
 *
 * [230] 二叉搜索树中第 K 小的元素
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = std::collections::VecDeque::new();
        stack.push_back(root);
        while let Some(node) = stack.pop_back() {
            match node {
                Some(node) => {

                    if let Some(right) = node.borrow_mut().right.clone() {
                        stack.push_back(Some(right));
                    }
                    stack.push_back(Some(node.clone()));
                    stack.push_back(None);
                    if let Some(left) = node.borrow_mut().left.clone() {
                        stack.push_back(Some(left));
                    }

                },
                None => {
                    let node = stack.pop_back().unwrap().unwrap();
                    k -= 1;
                    if k == 0 {
                        return node.borrow().val;
                    }
                },
            }
        }
        -1 // 这里应该不会到达
    }
}
// @lc code=end

