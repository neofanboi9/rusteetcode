// https://leetcode.com/problems/middle-of-the-linked-list/
use super::utils::ListNode;

fn find_link_list_size(head: Option<&Box<ListNode>>) -> i32 {
    let mut count = 0;
    let mut node = head;
    while node.is_some() {
        count += 1;
        node = node.as_ref().unwrap().next.as_ref();
    }
    count
}
struct Solution;
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let size = find_link_list_size(head.as_ref());
        let mut m = 0;
        let mut node = head;
        while m < size / 2 {
            m += 1;
            node = node.unwrap().next;
        }
        node
    }
}

#[test]
fn tests() {
    use super::utils::vec_to_list_node;
    assert_eq!(
        Solution::middle_node(vec_to_list_node(vec![1, 2, 3, 4, 5])),
        vec_to_list_node(vec![3, 4, 5])
    );
    assert_eq!(
        Solution::middle_node(vec_to_list_node(vec![1, 2, 3, 4, 5, 6])),
        vec_to_list_node(vec![4, 5, 6])
    );
}
