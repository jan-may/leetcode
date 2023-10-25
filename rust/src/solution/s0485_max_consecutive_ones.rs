/**
 * [485] Max Consecutive Ones
 *
 * Given a binary array nums, return the maximum number of consecutive 1's in the array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,1,0,1,1,1]
 * Output: 3
 * Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,0,1,1,0,1]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	nums[i] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-consecutive-ones/
// discuss: https://leetcode.com/problems/max-consecutive-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut count = 0;
        for num in nums {
            if num == 1 {
                count += 1;
            } else {
                result = result.max(count);
                count = 0;
            }
        }
        result.max(count)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_485() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
