use std::collections::HashMap;

/**
 * [1814] Count Nice Pairs in an Array
 *
 * You are given an array nums that consists of non-negative integers. Let us define rev(x) as the reverse of the non-negative integer x. For example, rev(123) = 321, and rev(120) = 21. A pair of indices (i, j) is nice if it satisfies all of the following conditions:
 *
 * 	0 <= i < j < nums.length
 * 	nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])
 *
 * Return the number of nice pairs of indices. Since that number can be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [42,11,1,97]
 * Output: 2
 * Explanation: The two pairs are:
 *  - (0,3) : 42 + rev(97) = 42 + 79 = 121, 97 + rev(42) = 97 + 24 = 121.
 *  - (1,2) : 11 + rev(1) = 11 + 1 = 12, 1 + rev(11) = 1 + 11 = 12.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [13,10,35,24,76]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-nice-pairs-in-an-array/
// discuss: https://leetcode.com/problems/count-nice-pairs-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut result: usize = 0;
        let mod_num: usize = 1000000007;
        for num in nums {
            let rev = Solution::reverse(num);
            let diff = num - rev;
            let count = map.entry(diff).or_insert(0);
            *count += 1;
        }
        for (_, count) in map {
            result += (count * (count - 1) / 2) % mod_num;
        }
        (result % mod_num) as i32
    }

    fn reverse(num: i32) -> i32 {
        let mut result = 0;
        let mut num = num;
        while num > 0 {
            result = result * 10 + num % 10;
            num /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1814() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
        assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
    }
}
