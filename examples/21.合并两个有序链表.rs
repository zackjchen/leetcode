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
        // let mut head = None;
        // let mut cur = &mut head;
        // let mut h1 = list1;
        // let mut h2 = list2;
        // *cur = loop {
        //     match (h1, h2) {
        //         (Some(mut n1), Some(mut n2)) => {
        //             if n1.val <= n2.val {
        //                 // 两个链表的所有权都被消耗了，所以这里都需要同时插回去
        //                 h1 = n1.next.take();
        //                 h2 = Some(n2);
        //                 cur = &mut cur.insert(n1).next;
        //             } else {
        //                 h2 = n2.next.take();
        //                 h1 = Some(n1);
        //                 cur = &mut cur.insert(n2).next;
        //             }
        //             // 这个match没有变量接受，所以不用返回
        //         },
        //         // 注意这个break也不是返回值，而是跳出循环，这个loop块只有这一个地方退出，所以这个break就是loop的返回值
        //         (x, y) => break x.or(y),
        //     }
        // };
        //// 常规写法
        // while h1.is_some() && h2.is_some() {
        //     if h1.as_mut().unwrap().val <= h2.as_mut().unwrap().val {
        //         *cur = h1.take();
        //         h1 = cur.as_mut().unwrap().next.take();
        //     }else {
        //         *cur = h2.take();
        //         h2 = cur.as_mut().unwrap().next.take();
        //     }
        //     cur = &mut cur.as_mut().unwrap().next;
        // }
        // if h1.is_some() {
        //     *cur = h1;
        // }
        // if h2.is_some() {
        //     *cur = h2;
        // }

        // 最优雅的写法，递归
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
