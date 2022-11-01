// https://leetcode.com/problems/richest-customer-wealth/
struct Solution;
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().fold(0, |accum, account| {
            let wealth: i32 = account.iter().sum();
            (wealth > accum)
                .then(|| wealth)
                .or_else(|| Some(accum))
                .unwrap()
        })
    }
}

#[test]
fn tests() {
    assert_eq!(
        Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
        6
    );
    assert_eq!(
        Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
        10
    );
    assert_eq!(
        Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
        17
    );
}
