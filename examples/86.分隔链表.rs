/*
 * @lc app=leetcode.cn id=86 lang=rust
 *
 * [86] 分隔链表
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut head1 = None;
        let mut head2 = None;
        let mut cur1 = &mut head1;
        let mut cur2 = &mut head2;
        loop {
            match head {
                Some(mut node) => {
                    if node.val < x {
                        head = node.next.take();
                        cur1 = &mut cur1.insert(node).next;
                    } else {
                        head = node.next.take();
                        cur2 = &mut cur2.insert(node).next;       
                    }
                },
                None => break,
            }
        }
        *cur1 = head2;
        head1
    }
}
// @lc code=end

