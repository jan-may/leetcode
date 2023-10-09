/**
 * [2535] Difference Between Element Sum and Digit Sum of an Array
 *
 * You are given a positive integer array nums.
 * 
 * 	The element sum is the sum of all the elements in nums.
 * 	The digit sum is the sum of all the digits (not necessarily distinct) that appear in nums.
 * 
 * Return the absolute difference between the element sum and digit sum of nums.
 * Note that the absolute difference between two integers x and y is defined as |x - y|.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,15,6,3]
 * Output: 9
 * Explanation: 
 * The element sum of nums is 1 + 15 + 6 + 3 = 25.
 * The digit sum of nums is 1 + 1 + 5 + 6 + 3 = 16.
 * The absolute difference between the element sum and digit sum is |25 - 16| = 9.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,2,3,4]
 * Output: 0
 * Explanation:
 * The element sum of nums is 1 + 2 + 3 + 4 = 10.
 * The digit sum of nums is 1 + 2 + 3 + 4 = 10.
 * The absolute difference between the element sum and digit sum is |10 - 10| = 0.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 2000
 * 	1 <= nums[i] <= 2000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/difference-between-element-sum-and-digit-sum-of-an-array/
// discuss: https://leetcode.com/problems/difference-between-element-sum-and-digit-sum-of-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut element_sum = 0;
        let mut digit_sum = 0;
        for num in nums {
            element_sum += num;
            let mut num = num;
            while num > 0 {
                digit_sum += num % 10;
                num /= 10;
            }
        }
        (element_sum - digit_sum).abs()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2535() {
        assert_eq!(Solution::difference_of_sum(vec![1, 15, 6, 3]), 9);
        assert_eq!(Solution::difference_of_sum(vec![1, 2, 3, 4]), 0);
    }
}
