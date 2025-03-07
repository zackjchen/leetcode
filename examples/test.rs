
use lib::ListNode;

fn main() {

    let list1 = vec![1,4,7];
    let list2 = vec![2,5,8,10,12];
    let list3 = vec![3,7,9];
    let head1 = ListNode::from_vec(list1);
    let head2 = ListNode::from_vec(list2);
    let head3 = ListNode::from_vec(list3);
    // let heads = vec![head1, head2, head3];
    let res = swap_pairs(head2);
    res.as_ref().unwrap().print_list();
}

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