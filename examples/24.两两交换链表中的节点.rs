/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_mut().unwrap().next.is_none() {
            return head;
        }
        let mut cur = &mut head;
        while cur.is_some() && cur.as_mut().unwrap().next.is_some() {
            let mut cur_node = cur.take().unwrap();
            let mut next_node = cur_node.as_mut().next.take().unwrap();
            cur_node.next = next_node.next.take();
            cur.insert(next_node).next = Some(cur_node);
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        };
        head
    }
}
// @lc code=end

