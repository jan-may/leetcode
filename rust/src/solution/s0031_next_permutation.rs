/**
 * [31] Next Permutation
 *
 * A permutation of an array of integers is an arrangement of its members into a sequence or linear order.
 *
 * 	For example, for arr = [1,2,3], the following are all the permutations of arr: [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
 *
 * The next permutation of an array of integers is the next lexicographically greater permutation of its integer. More formally, if all the permutations of the array are sorted in one container according to their lexicographical order, then the next permutation of that array is the permutation that follows it in the sorted container. If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).
 *
 * 	For example, the next permutation of arr = [1,2,3] is [1,3,2].
 * 	Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
 * 	While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical larger rearrangement.
 *
 * Given an array of integers nums, find the next permutation of nums.
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in place</a> and use only constant extra memory.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: [1,3,2]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,2,1]
 * Output: [1,2,3]
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,1,5]
 * Output: [1,5,1]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-permutation/
// discuss: https://leetcode.com/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i == 0 {
            nums.reverse();
            return;
        }
        let mut j = nums.len() - 1;
        while j >= i && nums[j] <= nums[i - 1] {
            j -= 1;
        }
        nums.swap(i - 1, j);
        nums[i..].reverse();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        assert_eq!(
            Solution::next_permutation(&mut vec![1, 2, 3]),
            vec![1, 3, 2]
        );
        assert_eq!(
            Solution::next_permutation(&mut vec![3, 2, 1]),
            vec![1, 2, 3]
        );
        assert_eq!(
            Solution::next_permutation(&mut vec![1, 1, 5]),
            vec![1, 5, 1]
        );
    }
}
