// https://leetcode.com/problems/roman-to-integer/

use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let hash: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let ss = s + "I";
        ss.chars()
            .zip(ss.chars().skip(1))
            .fold(0, |accum, (curr, next)| {
                let a = hash.get(&curr).unwrap_or(&0);
                let b = hash.get(&next).unwrap_or(&0);
                accum + (a < b).then(|| -a).or(Some(*a)).unwrap()
            })
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
