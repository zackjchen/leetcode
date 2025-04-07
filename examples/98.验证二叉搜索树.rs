/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
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
    /// 中序遍历， 判断当前节点值是否大于前一个节点值
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![];
        let mut last_val = i64::MIN;
        match root {
            Some(v) => stack.push(Some(v)),
            None => return true,
        }
        while let Some(node) = stack.pop() {
            match node {
                Some(v) => {
                    if let Some(right) = v.borrow().right.clone(){
                        stack.push(Some(right));
                    }
                    stack.push(Some(v.clone()));
                    stack.push(None);
                    if let Some(left) = v.borrow().left.clone(){
                        stack.push(Some(left));
                    }            
                },
                None => {
                    let v = stack.pop().unwrap().unwrap();
                    if v.borrow().val as i64 > last_val {
                        last_val = v.borrow().val as i64;
                    }else{
                        return false
                    }
                },
            }
        }
        true
    }
}
// @lc code=end

