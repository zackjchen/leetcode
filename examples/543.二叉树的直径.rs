/*
 * @lc app=leetcode.cn id=543 lang=rust
 *
 * [543] 二叉树的直径
 */


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut h = 1;
        fn deep(root: Option<Rc<RefCell<TreeNode>>>, h: &mut i32) -> i32 {
            if let Some(node) = root {
                let left = deep(node.borrow().left.clone(), h);
                let right = deep(node.borrow().right.clone(), h);
                *h  = (*h).max(left + right + 1);
                left.max(right) + 1
            }else{
                0
            }
        }
        deep(root, &mut h);
        h - 1
    }
}