
use lib::ListNode;

fn main() {

    let list1 = vec![1,4,7];
    let list2 = vec![2,5,8,10,12];
    let list3 = vec![3,7,9];
    let head1 = ListNode::from_vec(list1);
    let head2 = ListNode::from_vec(list2);
    let head3 = ListNode::from_vec(list3);
    // let heads = vec![head1, head2, head3];
    let res = reverse_k_group(head2,2);
    res.as_ref().unwrap().print_list();
}

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut next_head = &mut head;
    for _ in 0..k {
        if next_head.is_some(){
            next_head = &mut next_head.as_mut().unwrap().next;
        }else {
            return head;
        }
    }

    let mut new_head = reverse_k_group(next_head.take(), k);
    // 这里要反转链表
    for _ in 0..k {
        if let Some(mut node) = head {
            head = node.next.take();
            node.next = new_head.take();
            new_head = Some(node);
        }

    }
    new_head
}