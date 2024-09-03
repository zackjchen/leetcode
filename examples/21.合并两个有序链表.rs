/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */

use lib::ListNode;

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // let (mut l1, mut l2) = (list1, list2);
        // let mut dummy = None;
        // let mut cur = &mut dummy;
        // *cur = loop {
        //     // 这个match消耗了l1和l2的所有权，得到了n1和n2的可变引用
        //     match (l1, l2){
        //         (Some(mut n1), Some(mut n2)) => {
        //             // 找到较小的值，将其插入到cur中
        //             // 之后将l1或l2指向下一个节点，因为拿走了所有权，所以这里是take
        //             // cur指向下一个节点，每次都是None，没有操作的链表再恢复Option的所有权
        //             if n1.val < n2.val {
        //                 l1 = n1.next.take();
        //                 l2 = Some(n2);
        //                 cur = &mut cur.insert(n1).next;
        //             }else{
        //                 l2 = n2.next.take();
        //                 l1 = Some(n1);
        //                 cur = &mut cur.insert(n2).next;
        //             }
        //         },
        //         (x, y) => break x.or(y),
        //     }
        // };
        // dummy

        let (l1, l2) = (list1, list2);
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}
// @lc code=end
