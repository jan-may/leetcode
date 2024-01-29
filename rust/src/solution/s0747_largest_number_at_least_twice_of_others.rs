/**
 * [747] Largest Number At Least Twice of Others
 *
 * You are given an integer array nums where the largest integer is unique.
 * Determine whether the largest element in the array is at least twice as much as every other number in the array. If it is, return the index of the largest element, or return -1 otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [3,6,1,0]
 * Output: 1
 * Explanation: 6 is the largest integer.
 * For every other number in the array x, 6 is at least twice as big as x.
 * The index of value 6 is 1, so we return 1.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,2,3,4]
 * Output: -1
 * Explanation: 4 is less than twice the value of 3, so we return -1.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= nums.length <= 50
 * 	0 <= nums[i] <= 100
 * 	The largest element in nums is unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-number-at-least-twice-of-others/
// discuss: https://leetcode.com/problems/largest-number-at-least-twice-of-others/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut second_max = 0;
        let mut max_index = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num > max {
                second_max = max;
                max = num;
                max_index = i;
            } else if num > second_max {
                second_max = num;
            }
        }
        if max >= second_max * 2 {
            max_index as i32
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_747() {
        assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
        assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
    }
}
