/**
 * [75] Sort Colors
 *
 * Given an array nums with n objects colored red, white, or blue, sort them <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
 * We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
 * You must solve this problem without using the library's sort function.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,0,1]
 * Output: [0,1,2]
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 300
 * 	nums[i] is either 0, 1, or 2.
 *
 *  
 * Follow up: Could you come up with a one-pass algorithm using only constant extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-colors/
// discuss: https://leetcode.com/problems/sort-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        Solution::quicksort(nums, 0, nums.len() - 1)
    }
    fn quicksort(nums: &mut Vec<i32>, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let pivot = nums[start];
        let mut i = start;
        let mut j = end;
        while i < j {
            while i < j && nums[j] >= pivot {
                j -= 1;
            }
            nums[i] = nums[j];
            while i < j && nums[i] <= pivot {
                i += 1;
            }
            nums[j] = nums[i];
        }
        nums[i] = pivot;
        Solution::quicksort(nums, start, i);
        Solution::quicksort(nums, i + 1, end);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_75() {
        let mut vec = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0, 0, 1, 1, 2, 2]);
        let mut vec = vec![2, 0, 1];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0, 1, 2]);
    }
}
