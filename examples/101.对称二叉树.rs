/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
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
    // 递归
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
        fn compare(left: Option<Rc<RefCell<TreeNode<i32>>>>, right: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(_), None) | (None, Some(_))  => false,
                (Some(left), Some(right)) => {
                    return right.borrow().val == left.borrow().val 
                    && compare(left.borrow().left.clone(), right.borrow().right.clone()) 
                    && compare(left.borrow().right.clone(), right.borrow().left.clone())
                }
            }
        }
        
        if let Some(node) = root {
            return compare(node.borrow().left.clone(), node.borrow().right.clone())
        }else {
            return true
        }
    }
    // 迭代
    pub fn is_symmetric_2(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
        // 这里用stack和队列是一模一样的
        let mut queue = std::collections::VecDeque::new();
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            queue.push_back(left);
            queue.push_back(right);
        }else {
            return true;
        }
        while !queue.is_empty() {
            // 保证每次都能取出两个数据，不要这一层Option
            let left = queue.pop_back().unwrap();
            let right = queue.pop_back().unwrap();
            match (left, right) {
                (None, None) => continue,
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (Some(left), Some(right)) => {
                    if left.borrow().val != right.borrow().val {
                        return false;
                    }
                    queue.push_back(left.borrow().left.clone());
                    queue.push_back(right.borrow().right.clone());
                    queue.push_back(left.borrow().right.clone());
                    queue.push_back(right.borrow().left.clone());
                }   
            }
        }
        true
    }
}
// @lc code=end

