/**
 * [45] Jump Game II
 *
 * You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].
 * Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:
 *
 * 	0 <= j <= nums[i] and
 * 	i + j < n
 *
 * Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,3,0,1,4]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	0 <= nums[i] <= 1000
 * 	It's guaranteed that you can reach nums[n - 1].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-ii/
// discuss: https://leetcode.com/problems/jump-game-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut cur_end = 0;
        let mut cur_farthest = 0;
        for i in 0..nums.len() - 1 {
            cur_farthest = cur_farthest.max(i + nums[i] as usize);
            if i == cur_end {
                jumps += 1;
                cur_end = cur_farthest;
            }
        }
        jumps
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45() {}
}
