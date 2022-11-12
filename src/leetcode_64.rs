struct Solution;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
        dp[m - 1][n - 1] = grid[m - 1][n - 1];
        for ix in 0..n - 1 {
            dp[m - 1][n - ix - 2] = grid[m - 1][n - ix - 2] + dp[m - 1][n - ix - 1];
        }
        for ix in 0..m - 1 {
            dp[m - ix - 2][n - 1] = grid[m - ix - 2][n - 1] + dp[m - ix - 1][n - 1];
        }
        for ix in 0..m - 1 {
            for jx in 0..n - 1 {
                let mut min = dp[m - 2 - ix + 1][n - 2 - jx];
                if min > dp[m - 2 - ix][n - 2 - jx + 1] {
                    min = dp[m - 2 - ix][n - 2 - jx + 1];
                }
                dp[m - 2 - ix][n - 2 - jx] = grid[m - 2 - ix][n - 2 - jx] + min;
            }
        }
        dp[0][0]
    }
}

#[test]
fn tests() {
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        12
    )
}
