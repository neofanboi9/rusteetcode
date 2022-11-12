pub struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let i = nums.len()-1;
        if i == 0 {
            return true
        }
        for j in 0..i {
            let ix = i-j-1;
            if ix + nums[ix] as usize >= i {
                return Solution::can_jump(nums.drain(0..ix+1).collect())
            }
        }
        false
    }
}

#[test]
fn tests() {
    assert!(Solution::can_jump(vec![2,3,1,1,4]));
    assert!(!Solution::can_jump(vec![3,2,1,0,4]));
}
