/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
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
    // 前序遍历的第一个元素是根节点
    // 在中序遍历中找到根节点的位置
    // 中序遍历中根节点左边的元素是左子树，右边的元素是右子树
    // 获取根节点在中序遍历的下标，下标为右结点个数
    // 根据右结点个数获取前序遍历的左子树和右子树， 这里需要排除根节点
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        if preorder.len() == 1{
            return Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        }
        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let left_size = inorder.iter().position(|&x| x == preorder[0]).unwrap();     
        let inorder_left = inorder[0..left_size].to_vec();
        let inorder_right = inorder[left_size+1..].to_vec();
        let preorder_left = preorder[1..1+left_size].to_vec();
        let preorder_right = preorder[left_size+1..].to_vec();
        root.borrow_mut().left = Self::build_tree(preorder_left, inorder_left);
        root.borrow_mut().right = Self::build_tree(preorder_right, inorder_right);
        Some(root)
    }
}
// @lc code=end

