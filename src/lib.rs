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
        let cur = &mut head;
        for v in list{
            let node = ListNode::new(v);
            cur.replace(Box::new(node));
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
