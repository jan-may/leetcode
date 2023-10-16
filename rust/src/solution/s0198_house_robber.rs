use std::cmp::max;
use std::collections::HashMap;

/**
 * [198] House Robber
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
 * Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,7,9,3,1]
 * Output: 12
 * Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
 * Total amount you can rob = 2 + 9 + 1 = 12.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 400
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/house-robber/
// discuss: https://leetcode.com/problems/house-robber/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = (nums.len() - 1) as i32;
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Self::dp(len, &nums, &mut memo)
    }

    fn dp(i: i32, nums: &Vec<i32>, memo: &mut HashMap<i32, i32>) -> i32 {
        let result = match i {
            0 => nums[0],
            1 => max(nums[0], nums[1]),
            _ => {
                if let Some(&v) = memo.get(&i) {
                    return v;
                }
                max(
                    Self::dp(i - 1, nums, memo),
                    Self::dp(i - 2, nums, memo) + nums[i as usize],
                )
            }
        };
        memo.insert(i, result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_198() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
