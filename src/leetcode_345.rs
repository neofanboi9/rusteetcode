use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn reverse_vowles(s: String) -> String {
        let vowels: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u']
            .into_iter()
            .collect::<HashSet<_>>();
        let mut ss = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = s.len().saturating_sub(1);
        while i < j {
            let left = ss[i].to_ascii_lowercase();
            if !vowels.contains(&left) {
                i = i.saturating_add(1);
                continue;
            }
            let right = ss[j].to_ascii_lowercase();
            if !vowels.contains(&right) {
                j = j.saturating_sub(1);
                continue;
            }
            ss.swap(i, j);
            i = i.saturating_add(1);
            j = j.saturating_sub(1);
        }
        ss.into_iter().collect::<String>()
    }
}

#[test]
fn tests() {
    assert_eq!(
        Solution::reverse_vowles("hello".to_owned()),
        "holle".to_owned()
    );
    assert_eq!(
        Solution::reverse_vowles("leetcode".to_owned()),
        "leotcede".to_owned()
    );
    assert_eq!(Solution::reverse_vowles(" ".to_owned()), " ".to_owned());
    assert_eq!(Solution::reverse_vowles("a.".to_owned()), "a.".to_owned());
}