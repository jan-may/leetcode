/**
 * [1646] Get Maximum in Generated Array
 *
 * You are given an integer n. A 0-indexed integer array nums of length n + 1 is generated in the following way:
 *
 * 	nums[0] = 0
 * 	nums[1] = 1
 * 	nums[2 * i] = nums[i] when 2 <= 2 * i <= n
 * 	nums[2 * i + 1] = nums[i] + nums[i + 1] when 2 <= 2 * i + 1 <= n
 *
 * Return the maximum integer in the array nums​​​.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 7
 * Output: 3
 * Explanation: According to the given rules:
 *   nums[0] = 0
 *   nums[1] = 1
 *   nums[(1 * 2) = 2] = nums[1] = 1
 *   nums[(1 * 2) + 1 = 3] = nums[1] + nums[2] = 1 + 1 = 2
 *   nums[(2 * 2) = 4] = nums[2] = 1
 *   nums[(2 * 2) + 1 = 5] = nums[2] + nums[3] = 1 + 2 = 3
 *   nums[(3 * 2) = 6] = nums[3] = 2
 *   nums[(3 * 2) + 1 = 7] = nums[3] + nums[4] = 2 + 1 = 3
 * Hence, nums = [0,1,1,2,1,3,2,3], and the maximum is max(0,1,1,2,1,3,2,3) = 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 2
 * Output: 1
 * Explanation: According to the given rules, nums = [0,1,1]. The maximum is max(0,1,1) = 1.
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 3
 * Output: 2
 * Explanation: According to the given rules, nums = [0,1,1,2]. The maximum is max(0,1,1,2) = 2.
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/get-maximum-in-generated-array/
// discuss: https://leetcode.com/problems/get-maximum-in-generated-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let n = n as usize;
        let mut nums = vec![0; (n + 1) as usize];
        if n == 0 {
            return 0;
        }
        nums[1] = 1;
        let mut max = 1;
        for i in 2..=n {
            nums[i] = nums[i / 2];
            if i % 2 == 1 {
                nums[i] += nums[i / 2 + 1];
            }
            max = max.max(nums[i]);
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1646() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
        assert_eq!(Solution::get_maximum_generated(2), 1);
        assert_eq!(Solution::get_maximum_generated(3), 2);
    }
}
