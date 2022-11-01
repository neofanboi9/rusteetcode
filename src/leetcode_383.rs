use std::collections::HashMap;

// https://leetcode.com/problems/ransom-note/
struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters: HashMap<char, i32> = HashMap::with_capacity(magazine.len());
        for ch in magazine.chars() {
            letters.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }
        for ch in ransom_note.chars() {
            letters.entry(ch).and_modify(|e| *e -= 1).or_insert(-1);
        }
        letters.values().all(|e| *e > -1)
    }
}

#[test]
fn tests() {
    assert!(!Solution::can_construct("a".to_owned(), "b".to_owned()));
    assert!(!Solution::can_construct("aa".to_owned(), "ab".to_owned()));
    assert!(Solution::can_construct("aa".to_owned(), "aab".to_owned()));
}
