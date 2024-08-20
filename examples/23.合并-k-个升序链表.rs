/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并 K 个升序链表
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

use std::{cmp::Reverse, collections::BinaryHeap};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut curr = &mut head;
        let mut heap =  BinaryHeap::new();
        for mut node in lists.into_iter() {
            if let Some(node) = node {
                heap.push(Reverse(node));                
            }
        }

        while let Some(mut node) = heap.pop() {
            let y = node.0.next.take();
            curr = &mut curr.insert(node.0).next;
            if y.is_some() {
                heap.push(Reverse(y.unwrap()));
            }
        }
        head
    }
}
// @lc code=end

