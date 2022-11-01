use std::cmp::Ordering;

// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
struct Solution;
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut v = mat
            .iter()
            .map(|row| row.iter().sum::<i32>())
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        v.sort_by(|a, b| {
            let ord = a.1.cmp(&b.1);
            (ord == Ordering::Equal)
                .then(|| a.0.cmp(&b.0))
                .or_else(|| Some(ord))
                .unwrap()
        });
        let (left, _) = v.split_at(k as usize);
        left.iter().map(|(ix, _)| *ix as i32).collect()
    }
}

#[test]
fn tests() {
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            3,
        ),
        vec![2, 0, 3]
    );
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
            ],
            2,
        ),
        vec![0, 2]
    );
}
