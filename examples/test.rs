
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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
    // 这里用stack和队列是一模一样的
    let mut queue = std::collections::VecDeque::new();
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        queue.push_back(left);
        queue.push_back(right);
    }else {
        return true;
    }
    while !queue.is_empty() {
        // 保证每次都能取出两个数据，不要这一层Option
        let left = queue.pop_back().unwrap();
        let right = queue.pop_back().unwrap();
        match (left, right) {
            (None, None) => continue,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(left), Some(right)) => {
                if left.borrow().val != right.borrow().val {
                    return false;
                }
                queue.push_back(left.borrow().left.clone());
                queue.push_back(right.borrow().right.clone());
                queue.push_back(left.borrow().right.clone());
                queue.push_back(right.borrow().left.clone());
            }   
        }
    }
    true
}