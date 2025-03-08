/*
 * @lc app=leetcode.cn id=148 lang=rust
 *
 * [148] 排序链表
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() || head.as_ref().unwrap().next.is_none() {
                return head;
            }
            let mut f = &head;
            let mut t = &head;
            while f.is_some() && f.as_ref().unwrap().next.is_some() {
                f = &f.as_ref().unwrap().next.as_ref().unwrap().next;
                t = &t.as_ref().unwrap().next;
            }
            #[allow(mutable_transmutes)]
            let t: &mut Option<Box<ListNode>> = unsafe {
                std::mem::transmute(t)
            };
            let tail = Self::sort_list(t.take());
            let front = Self::sort_list(head.take());
        
            head = Self::merge(front, tail);
        
            head
        }
        
        fn merge(list1:Option<Box<ListNode>>,list2:Option<Box<ListNode>>) ->Option<Box<ListNode>> {
            let mut head = None;
            let mut cur = &mut head;
            let mut queue = BinaryHeap::new();
            if list1.is_some() {
                queue.push(Reverse(list1.unwrap()));
            }
            if list2.is_some() {
                queue.push(Reverse(list2.unwrap()));
            }
        
            while !queue.is_empty() {
                if let Some(mut node) = queue.pop(){
                    let next = node.0.next.take();
                    if next.is_some() {
                        queue.push(Reverse(next.unwrap()));
                    }
                    cur = &mut cur.insert(node.0).next;
                }
            }
            head
        }
    }
}
// @lc code=end

