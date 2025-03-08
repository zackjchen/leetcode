
use std::{cmp::Reverse, collections::BinaryHeap};

use lib::ListNode;

fn main() {

    let list1 = vec![1,4,7];
    let list2 = vec![2,5,8,10,12];
    let list3 = vec![4,2,1,3,7,9];
    let head1 = ListNode::from_vec(list1);
    let head2 = ListNode::from_vec(list2);
    let head3 = ListNode::from_vec(list3);
    // let heads = vec![head1, head2, head3];
    let res = sort_list(head3);
    // let res = merge(head1,head2);
    res.as_ref().unwrap().print_list();
}

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
    let tail = sort_list(t.take());
    let front = sort_list(head.take());

    head = merge(front, tail);

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