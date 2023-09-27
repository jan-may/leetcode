/**
 * [35] Search Insert Position
 *
 * Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
 * You must write an algorithm with O(log n) runtime complexity.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums contains distinct values sorted in ascending order.
 * 	-10^4 <= target <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-insert-position/
// discuss: https://leetcode.com/problems/search-insert-position/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // binary search
        let mut left = 0;
        let mut right = nums.len() as i64 - 1;
        let target_i64 = target as i64;

        while left <= right {
            let middle = left + (right - left) / 2;
            let middle_value = nums[middle as usize] as i64;

            if middle_value == target_i64 {
                return middle as i32;
            }

            if middle_value > target_i64 {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }

        left as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_35() {}
}
