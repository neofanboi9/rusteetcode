use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (ix, num) in nums.iter().enumerate() {
            match cache.get_key_value(&(target - *num)) {
                None => {
                    cache.entry(*num).or_insert(ix);
                },
                Some((_, jx)) => {
                    return vec![*jx as i32, ix as i32];
                }
            }
        }
        vec![-1, -1]
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 3], 4), vec![-1, -1]);
}