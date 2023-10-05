/**
 * [2574] Left and Right Sum Differences
 *
 * Given a 0-indexed integer array nums, find a 0-indexed integer array answer where:
 *
 *     answer.length == nums.length.
 *     answer[i] = |leftSum[i] - rightSum[i]|.
 *
 * Where:
 *
 *     leftSum[i] is the sum of elements to the left of the index i in the array nums. If there is no such element, leftSum[i] = 0.
 *     rightSum[i] is the sum of elements to the right of the index i in the array nums. If there is no such element, rightSum[i] = 0.
 *
 * Return the array answer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [10,4,8,3]
 * Output: [15,1,11,22]
 * Explanation: The array leftSum is [0,10,14,22] and the array rightSum is [15,11,3,0].
 * The array answer is [|0 - 15|,|10 - 11|,|14 - 3|,|22 - 0|] = [15,1,11,22].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1]
 * Output: [0]
 * Explanation: The array leftSum is [0] and the array rightSum is [0].
 * The array answer is [|0 - 0|] = [0].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/left-and-right-sum-differences/
// discuss: https://leetcode.com/problems/left-and-right-sum-differences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left_sum = vec![0; nums.len()];
        let mut right_sum = vec![0; nums.len()];
        let mut result = vec![0; nums.len()];
        let mut left = 0;
        let mut right = 0;
        for i in 0..nums.len() {
            left_sum[i] = left;
            left += nums[i];
            right_sum[nums.len() - 1 - i] = right;
            right += nums[nums.len() - 1 - i];
        }
        for i in 0..nums.len() {
            result[i] = (left_sum[i] - right_sum[i]).abs();
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2574() {
        assert_eq!(
            Solution::left_right_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
        assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
    }
}
