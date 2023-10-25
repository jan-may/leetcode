/**
 * [448] Find All Numbers Disappeared in an Array
 *
 * Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [4,3,2,7,8,2,3,1]
 * Output: [5,6]
 * <strong class="example">Example 2:
 * Input: nums = [1,1]
 * Output: [2]
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	1 <= nums[i] <= n
 *
 *  
 * Follow up: Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
// discuss: https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut nums = nums;
        for i in 0..nums.len() {
            let index = (nums[i].abs() - 1) as usize;
            nums[index] = -nums[index].abs();
        }
        for i in 0..nums.len() {
            if nums[i] > 0 {
                result.push((i + 1) as i32);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_448() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }
}
