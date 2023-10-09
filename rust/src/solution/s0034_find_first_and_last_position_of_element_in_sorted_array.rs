/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
 * If target is not found in the array, return [-1, -1].
 * You must write an algorithm with O(log n) runtime complexity.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * <strong class="example">Example 2:
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * <strong class="example">Example 3:
 * Input: nums = [], target = 0
 * Output: [-1,-1]
 *  
 * Constraints:
 * 
 * 	0 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	nums is a non-decreasing array.
 * 	-10^9 <= target <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// discuss: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let middle = left + (right - left) / 2;
            let middle_value = nums[middle as usize];
            if middle_value == target {
                let mut start = middle;
                let mut end = middle;

                while start > 0 && nums[start as usize - 1] == target {
                    start -= 1;
                }

                while end < nums.len() as i32 - 1 && nums[end as usize + 1] == target {
                    end += 1;
                }

                return vec![start as i32, end as i32];
            }

            if middle_value > target {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }
        vec![-1, -1]
    }
}



// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    }
}
