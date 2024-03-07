pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut showIndex = 0;
        for pos in (0..nums.len()) {
            if nums[pos] != val {
                nums[showIndex] = nums[pos];
                showIndex += 1;
            }
        }
        showIndex as i32
    }
}
