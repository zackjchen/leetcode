/*
 * @lc app=leetcode.cn id=199 lang=rust
 *
 * [199] 二叉树的右视图
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
    /// 深度优先遍历
    /// 使用栈来模拟递归
    /// 用一个hashmap来存储每一层的节点，高度作为key，节点值作为value
    /// 如果当前层已经存在节点，则更新，否则插入，这样保证最后插入的节点是最右边的节点
    /// 先遍历右子树，再遍历左子树
    /// 最后需要按照深度排序
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        match root {
            Some(node) => {
                stack.push((node, 0));
            }
            None => return vec![],
        }
        while let Some((node, h)) = stack.pop() {
            map.entry(h).or_insert(node.borrow().val);
            if let Some(left) = node.borrow_mut().left.clone() {
                stack.push((left, h + 1));
            }
            if let Some(right) = node.borrow_mut().right.clone() {
                stack.push((right, h + 1));
            }
        }    
    
        let mut v = map.into_iter().collect::<Vec<_>>();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        let res = v.into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>();
        res
    }
}
// @lc code=end

