struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut offset = 1;
        let mut ix = 1;
        while ix < nums.len() {
            if nums[ix - 1] == nums[ix] {
                ix += 1;
            } else if nums[ix - 1] != nums[ix] {
                nums[offset] = nums[ix];
                ix += 1;
                offset += 1;
            }
        }
        offset as i32
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::remove_duplicates(& mut vec![1,1,2]), 2);
    assert_eq!(Solution::remove_duplicates(& mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
}