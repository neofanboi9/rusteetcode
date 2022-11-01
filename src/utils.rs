// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(items: Vec<i32>) -> Self {
        let mut current = ListNode { val: 0, next: None };
        for (ix, item) in items.iter().rev().enumerate() {
            if ix == 0 {
                current = ListNode::new(*item);
                continue;
            }
            current = ListNode {
                val: *item,
                next: Some(Box::new(current)),
            }
        }
        current
    }
}

pub fn vec_to_list_node(v: Vec<i32>) -> Option<Box<ListNode>> {
    Some(Box::new(v.into()))
}
