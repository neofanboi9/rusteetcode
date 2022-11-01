use crate::utils::{vec_to_list_node, ListNode};

struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h: Option<Box<ListNode>> = None;
        let mut current = head;
        while current.is_some() {
            if h.is_none() {
                h = current.take();
                current = h.as_mut().unwrap().next.take();
            } else {
                let nnext = current.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = h.take();
                h = current.take();
                current = nnext
            }
        }
        h
    }
}

#[test]
fn tests() {
    assert_eq!(
        Solution::reverse_list(vec_to_list_node(vec![1, 2, 3, 4, 5])),
        vec_to_list_node(vec![5, 4, 3, 2, 1])
    );
    assert_eq!(
        Solution::reverse_list(vec_to_list_node(vec![1, 2])),
        vec_to_list_node(vec![2, 1])
    );
    assert_eq!(
        Solution::reverse_list(vec_to_list_node(vec![])),
        vec_to_list_node(vec![])
    );
}
