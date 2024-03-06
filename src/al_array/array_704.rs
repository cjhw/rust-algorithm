use std::cmp::Ordering;

// 二分查找
pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (right + left) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
                Ordering::Equal => return mid as i32,
            }
        }
        -1
    }
}
