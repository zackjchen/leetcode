/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] 将有序数组转换为二叉搜索树
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
    /// 将有序数组转换为二叉搜索树
    /// 1. 取中间值作为根节点
    /// 2. 左边的值作为左子树，右边的值作为右子树
    /// 3. 递归处理左子树和右子树
    /// 4. 返回根节点
    /// 5. 递归结束条件是数组为空
    /// 6. 时间复杂度O(n)，空间复杂度O(logn)
    /// 7. 递归深度为O(logn)，每次递归都要创建一个新的节点
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let m = nums.len() / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(nums[m])));
        let left = nums[0..m].to_vec();
        let right = nums[m + 1..].to_vec();
        node.borrow_mut().left = Self::sorted_array_to_bst(left);
        node.borrow_mut().right = Self::sorted_array_to_bst(right);
        Some(node)
    }
}
// @lc code=end

