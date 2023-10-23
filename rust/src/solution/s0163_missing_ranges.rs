struct Solution;

impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut prev = lower - 1;
        for i in 0..=nums.len() {
            let curr = if i == nums.len() { upper + 1 } else { nums[i] };
            if curr - prev >= 2 {
                result.push(Self::get_range(prev + 1, curr - 1));
            }
            prev = curr;
        }
        result
    }
    pub fn get_range(lower: i32, upper: i32) -> Vec<i32> {
        if lower == upper {
            vec![lower, lower]
        } else {
            vec![lower, upper]
        }
    }
}
