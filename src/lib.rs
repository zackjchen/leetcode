use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(list: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut cur = &mut head;
        for v in list{
            let node = ListNode::new(v);
            *cur = Some(Box::new(node));
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }

    pub fn print_list(&self) {
        println!("List: {} ->", self.val);
        let mut cur = &self.next;
        while let Some(node) = cur {
            print!("{} -> ", node.val);
            cur = &node.next;
        }
        println!("null");
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
    
}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T=i32> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl <T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

