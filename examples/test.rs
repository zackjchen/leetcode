
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    let mut stack = VecDeque::new();
    
    if let Some(node) = root.clone() {
        stack.push_back(node);
    }else {
        return None;
    }
    while !stack.is_empty() {
        if let Some(node) = stack.pop_front(){
            let mut node_borrow = node.borrow_mut();
            let t = node_borrow.left.clone();
            node_borrow.left = node_borrow.right.clone();
            node_borrow.right = t;

            if let Some(left) = node_borrow.left.clone(){
                stack.push_back(left);
            }
            if let Some(right) = node_borrow.right.clone(){
                stack.push_back(right);
            }
        }
    }
    root
}