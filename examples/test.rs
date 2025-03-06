
use lib::ListNode;

fn main() {

    let list = vec![1,2,3,2,1];
    let head = ListNode::from_vec(list);
    head.as_ref().unwrap().print_list();
    let res = is_palindrome(head);
    println!("res={:?}", res);
}
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut list = Vec::new();
    let mut p = &head;
    while let Some(node) = p {
        list.push(node.val);
        p = &node.next;
    }
    let mut left = 0;
    let mut right = list.len() - 1;
    while left < right {
        if list[left] != list[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}