// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
struct Solution;
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut count = 0;
        let mut num = num;
        while num != 0 {
            count += 1;
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
        }
        count
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::number_of_steps(14), 6);
    assert_eq!(Solution::number_of_steps(8), 4);
    assert_eq!(Solution::number_of_steps(123), 12);
}
