/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        unsafe {
            let mut fast = &mut dummy as *mut Box<ListNode>;
            let mut slow = &mut dummy as *mut Box<ListNode>;
            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap();
            }
            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            println!("{:?}", slow);
            (*slow).next = (*slow).next.take().unwrap().next;
        }
        dummy.next

        // rust的独特写法，可以不用找前一个节点，直接删除
        // unsafe {
        //     let mut fast = &mut head as *mut Option<Box<ListNode>>;
        //     let mut slow = &mut head as *mut Option<Box<ListNode>>;
        //     for _ in 0..n {
        //         fast = &mut (*fast).as_mut().unwrap().next ;
        //     }
        //     while (*fast).is_some() {
        //         fast = &mut (*fast).as_mut().unwrap().next ;
        //         slow = &mut (*slow).as_mut().unwrap().next ;
        //     }
        //     *slow = (*slow).take().unwrap().next
        // }
        // head
    }
}
// @lc code=end
