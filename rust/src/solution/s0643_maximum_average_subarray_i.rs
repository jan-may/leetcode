/**
 * [643] Maximum Average Subarray I
 *
 * You are given an integer array nums consisting of n elements, and an integer k.
 * Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10^-5 will be accepted.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,12,-5,-6,50,3], k = 4
 * Output: 12.75000
 * Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [5], k = 1
 * Output: 5.00000
 * 
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= k <= n <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-average-subarray-i/
// discuss: https://leetcode.com/problems/maximum-average-subarray-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max = std::i32::MIN;
        let mut sum = 0;
        let k = k as usize;
        for i in 0..nums.len() {
            sum += nums[i];
            if i >= k {
                sum -= nums[i - k];
            }
            if i >= k - 1 {
                max = std::cmp::max(max, sum);
            }
        }
        max as f64 / k as f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_643() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75
        );
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.0);
    }
}
