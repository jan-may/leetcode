/**
 * [18] 4Sum
 *
 * Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
 *
 * 	0 <= a, b, c, d < n
 * 	a, b, c, and d are distinct.
 * 	nums[a] + nums[b] + nums[c] + nums[d] == target
 *
 * You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,0,-1,0,-2,2], target = 0
 * Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,2,2,2,2], target = 8
 * Output: [[2,2,2,2]]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 200
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/4sum/
// discuss: https://leetcode.com/problems/4sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = Vec::new();
        let target = target as i64; // Convert target to i64

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut left = j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    // Convert each number to i64 before summing to avoid overflow
                    let sum =
                        nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;
                    if sum == target {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        right -= 1;
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    } else if sum < target {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18() {}
}
