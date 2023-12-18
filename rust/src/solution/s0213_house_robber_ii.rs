/**
 * [213] House Robber II
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.
 * Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,3,2]
 * Output: 3
 * Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,2,3]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/house-robber-ii/
// discuss: https://leetcode.com/problems/house-robber-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()-1];
        let mut dp2 = vec![0; nums.len()-1];

        if nums.len() == 1 {
            return nums[0];
        }

        if nums.len() == 2{
            return std::cmp::max(nums[0], nums[1]);
        }

        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);
        for i in 2..nums.len() - 1 {
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
        }

        dp2[0] = nums[1];
        dp2[1] = std::cmp::max(nums[1], nums[2]);
        for i in 2..nums.len() - 1 {
            dp2[i] = std::cmp::max(dp2[i - 1], dp2[i - 2] + nums[i + 1]);
        }
        std::cmp::max(dp[nums.len() - 2], dp2[nums.len() - 2])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_213() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }
}
