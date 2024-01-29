/**
 * [704] Binary Search
 *
 * Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
 * You must write an algorithm with O(log n) runtime complexity.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [-1,0,3,5,9,12], target = 2
 * Output: -1
 * Explanation: 2 does not exist in nums so return -1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^4
 * 	-10^4 < nums[i], target < 10^4
 * 	All the integers in nums are unique.
 * 	nums is sorted in ascending order.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-search/
// discuss: https://leetcode.com/problems/binary-search/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() -1) as i32;
        while left <= right {
            let mid = ((left + right) / 2) as usize;
            if mid >= nums.len() {
                break;
            }
            if nums[mid ] < target{
                left = (mid + 1) as i32;
            }
            else if nums[mid] > target{
                right = (mid as i32 - 1);
            }
            else {
                return mid as i32;
            }
        }
        -1
    }
}



// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_704() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![5], -5), -1);
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 13), -1);
    }
}
