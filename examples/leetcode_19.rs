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
}
struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 在头节点上新建一个dummy节点
        // 考虑测试用例
        // [1] 0
        // [1] 1
        // [ ] 0 下面的没有考虑到
        if head.is_none() || n == 0 {
            return None;
        }
        let mut h2 = Box::new(ListNode {
            val: -1,
            next: head,
        });
        unsafe {
            let mut p = &mut h2 as *mut Box<ListNode>;
            let mut q = &mut h2 as *mut Box<ListNode>;
            for _ in 0..n {
                // *的优先级较小，需要括号
                // 这里的*是由于 p为*mut Box<ListNode>，要获取内容，或者修改内容的值时解引用
                // 签名这个p是一个原始指针，把它指向另一个位置时不用解引用，直接赋值
                // .next指向是一个Option<Box<ListNode>>类型，p是可变引用，需要unwrap
                // 但是直接unwrap返回的是有所有权的，不是引用
                p = (*p).next.as_mut().unwrap();
            }
            while (*p).next.is_some() {
                p = (*p).next.as_mut().unwrap();
                q = (*q).next.as_mut().unwrap();
            }
            (*q).next = (*q).next.take().unwrap().next;
        }
        h2.next
    }
}

fn main() {
    let mut head = None;
    let mut p = &mut head;
    for i in 0..0 {
        let node = ListNode::new(i);
        p = &mut p.insert(Box::new(node)).next;
    }

    let res = Solution::remove_nth_from_end(head, 0);

    println!("{:?}", res);
}
