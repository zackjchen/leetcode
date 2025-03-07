/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head = None;
        let mut cur = &mut head;
        loop {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    let sum = n1.val + n2.val + carry;
                    carry = sum / 10;
                    cur = &mut cur.insert(Box::new(ListNode::new(sum % 10))).next;
                    l1 = n1.next;
                    l2 = n2.next;
                    println!("sum: {}", sum);
                },
                (x,y) => {
                    let mut node = x.or(y);
                    while let Some(n) = node {
                        let sum = n.val + carry;
                        carry = sum / 10;
                        cur = &mut cur.insert(Box::new(ListNode::new(sum % 10))).next;
                        node = n.next;
                    }
                    if carry > 0 {
                        let _ = cur.insert(Box::new(ListNode::new(carry)));
                    }
                    break;
                }
            }
        }
        head
        // 更好的写法，是一个个的累加
        // let mut dummy = ListNode::new(0); // 哨兵节点
        // let mut cur = &mut dummy;
        // let mut carry = 0; // 进位
        // while l1.is_some() || l2.is_some() || carry != 0 {
        //     if let Some(node) = l1 {
        //         carry += node.val; // 节点值和进位加在一起
        //         l1 = node.next; // 下一个节点
        //     }
        //     if let Some(node) = l2 {
        //         carry += node.val; // 节点值和进位加在一起
        //         l2 = node.next; // 下一个节点
        //     }
        //     cur.next = Some(Box::new(ListNode::new(carry % 10))); // 每个节点保存一个数位
        //     carry /= 10; // 新的进位
        //     cur = cur.next.as_mut()?; // 下一个节点
        // }
        // dummy.next // 哨兵节点的下一个节点就是头节点
    }
}
// @lc code=end

