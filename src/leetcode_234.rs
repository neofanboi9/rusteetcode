use crate::utils::{vec_to_list_node, ListNode};

// https://leetcode.com/problems/palindrome-linked-list/

fn find_link_list_size(head: Option<&Box<ListNode>>) -> i32 {
    let mut count = 0;
    let mut node = head;
    while node.is_some() {
        count += 1;
        node = node.as_ref().unwrap().next.as_ref();
    }
    count
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

struct Solution;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let size = find_link_list_size(head.as_ref());
        let mut n = &head;
        let mut c = size / 2 - 1;
        while c > 0 {
            c -= 1;
            n = &n.as_ref().unwrap().next;
        }
        let mut tl = reverse_list(n.as_ref().unwrap().as_ref().next.clone());
        let mut hd = head.as_ref();
        while tl.is_some() {
            if tl.as_ref().unwrap().val != hd.unwrap().val {
                return false;
            }
            tl = tl.unwrap().next.take();
            hd = hd.unwrap().next.as_ref();
        }
        true
    }
}

#[test]
fn tests() {
    assert!(Solution::is_palindrome(vec_to_list_node(vec![1, 2, 2, 1])));
    assert!(!Solution::is_palindrome(vec_to_list_node(vec![1, 2, 1, 1])));
}
