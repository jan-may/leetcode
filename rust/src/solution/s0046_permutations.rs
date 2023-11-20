/**
 * [46] Permutations
 *
 * Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * <strong class="example">Example 2:
 * Input: nums = [0,1]
 * Output: [[0,1],[1,0]]
 * <strong class="example">Example 3:
 * Input: nums = [1]
 * Output: [[1]]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 6
 * 	-10 <= nums[i] <= 10
 * 	All the integers of nums are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutations/
// discuss: https://leetcode.com/problems/permutations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut nums = nums;
        Solution::backtrack(&mut nums, 0, &mut result);
        result
    }

    fn backtrack(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
        } else {
            for i in start..nums.len() {
                nums.swap(start, i);
                Solution::backtrack(nums, start + 1, result);
                nums.swap(start, i);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46() {}
}
