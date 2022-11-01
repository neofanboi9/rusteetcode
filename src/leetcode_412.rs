// https://leetcode.com/problems/fizz-buzz/
struct Solution;
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n + 1)
            .into_iter()
            .map(|t| match (t % 3, t % 5) {
                (0, 0) => "FizzBuzz".to_owned(),
                (0, _) => "Fizz".to_owned(),
                (_, 0) => "Buzz".to_owned(),
                _ => format!("{}", t).to_owned(),
            })
            .collect()
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    assert_eq!(
        Solution::fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
}
