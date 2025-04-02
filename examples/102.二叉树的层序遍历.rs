/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层序遍历
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
    /// 思路1:
    /// 单个队列，每一行最后一个结点的时候插入none，表示该行结束
    /// 每次遍历到none，表示该行的下一行全部加入到队列了，再插入一个新的none
    /// 思路二：
    /// 两个队列，一个存储当前行，一个存储下一行，交替遍历
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut row = vec![];
        let mut queue = VecDeque::new();
        match root {
            Some(root) => {
                queue.push_back(Some(root));
                queue.push_back(None);
            },
            None => {
                return res;
            },
        }
    
        while let Some(node) = queue.pop_front() {
            match node {
                Some(node) => {
                    row.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(Some(left));
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(Some(right));
                    }
                },
                None => {
                    res.push(row);
                    row = vec![];
                    if !queue.is_empty() {
                        queue.push_back(None);
                    }
                },
            }
        }
        res
    
    }
}
// @lc code=end

