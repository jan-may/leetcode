/**
 * [1685] Sum of Absolute Differences in a Sorted Array
 *
 * You are given an integer array nums sorted in non-decreasing order.
 * Build and return an integer array result with the same length as nums such that result[i] is equal to the summation of absolute differences between nums[i] and all the other elements in the array.
 * In other words, result[i] is equal to sum(|nums[i]-nums[j]|) where 0 <= j < nums.length and j != i (0-indexed).
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,3,5]
 * Output: [4,3,5]
 * Explanation: Assuming the arrays are 0-indexed, then
 * result[0] = |2-2| + |2-3| + |2-5| = 0 + 1 + 3 = 4,
 * result[1] = |3-2| + |3-3| + |3-5| = 1 + 0 + 2 = 3,
 * result[2] = |5-2| + |5-3| + |5-5| = 3 + 2 + 0 = 5.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,4,6,8,10]
 * Output: [24,15,13,15,21]
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^5
 * 	1 <= nums[i] <= nums[i + 1] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/
// discuss: https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        for i in 0..nums.len() {
            result[i] = (nums[i] * (i as i32) - nums[0..i].iter().sum::<i32>())
                + (nums[i..].iter().sum::<i32>() - nums[i] * ((nums.len() - i) as i32));
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1685() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 3, 5]),
            vec![4, 3, 5]
        );
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
            vec![24, 15, 13, 15, 21]
        );
    }
}
