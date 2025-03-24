
use std::{cell::RefCell, rc::Rc};

use lib::TreeNode;


fn main() {
    
    // let list1 = vec![1,4,7];
    // let list2 = vec![2,5,8,10,12];
    // let list3 = vec![4,2,1,3,7,9];
    // let head1 = ListNode::from_vec(list1);
    // let head2 = ListNode::from_vec(list2);
    // let head3 = ListNode::from_vec(list3);
    // // let heads = vec![head1, head2, head3];
    // let res = sort_list(head3);
    // // let res = merge(head1,head2);
    // res.as_ref().unwrap().print_list();


}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> i32 {
    let mut queue = std::collections::VecDeque::new();

    if root.is_none() {
        return 0;
    }
    let mut res = 0;
    queue.push_back(root.unwrap());
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size{
            if let Some(node) = queue.pop_front(){
                if let Some(left) = node.borrow().left.clone(){
                    queue.push_back(left);
                }
                if let Some(right) = node.borrow().right.clone(){
                    queue.push_back(right);
                }
            }
        }
        res += 1;
    }

    res
}