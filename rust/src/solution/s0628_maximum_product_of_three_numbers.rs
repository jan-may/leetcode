/**
 * [628] Maximum Product of Three Numbers
 *
 * Given an integer array nums, find three numbers whose product is maximum and return the maximum product.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,2,3]
 * Output: 6
 * <strong class="example">Example 2:
 * Input: nums = [1,2,3,4]
 * Output: 24
 * <strong class="example">Example 3:
 * Input: nums = [-1,-2,-3]
 * Output: -6
 *  
 * Constraints:
 * 
 * 	3 <= nums.length <= 10^4
 * 	-1000 <= nums[i] <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-of-three-numbers/
// discuss: https://leetcode.com/problems/maximum-product-of-three-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let len = nums.len();
        let mut max = nums[len - 1] * nums[len - 2] * nums[len - 3];
        if nums[0] < 0 && nums[1] < 0 {
            max = std::cmp::max(max, nums[0] * nums[1] * nums[len - 1]);
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_628() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
        assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6);
    }
}
