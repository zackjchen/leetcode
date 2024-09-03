use lib::ListNode;

struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        loop {
            if fast.is_none() {
                break;
            }
            if fast.as_ref().unwrap().next.is_none() {
                slow = &slow.as_ref().unwrap().next;
                break;
            }
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }
        slow.clone()
    }
}
// 1 2 3 4 5 6
//             p
//       q

fn main() {
    let mut head = None;
    let mut p = &mut head;
    for i in 1..7 {
        let _node = Box::new(ListNode::new(i));
        p = &mut p.insert(Box::new(ListNode::new(i))).next;
    }
    println!("{:?}", head);
    let res = Solution::middle_node(head);
    println!("{:?}", res)
}
