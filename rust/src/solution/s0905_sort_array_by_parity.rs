/**
 * [905] Sort Array By Parity
 *
 * Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.
 * Return any array that satisfies this condition.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,1,2,4]
 * Output: [2,4,3,1]
 * Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5000
 * 	0 <= nums[i] <= 5000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-array-by-parity/
// discuss: https://leetcode.com/problems/sort-array-by-parity/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut results = nums;
        let mut i = 0;
        let mut j = results.len() - 1;
        while i < j {
            if results[i] % 2 == 0 {
                i += 1;
            } else {
                results.swap(i, j);
                j -= 1;
            }
        }
        results
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_905() {
        assert_eq!(
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
            vec![4, 2, 1, 3]
        );
        assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
    }
}
