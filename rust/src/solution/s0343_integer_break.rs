/**
 * [343] Integer Break
 *
 * Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers.
 * Return the maximum product you can get.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 2
 * Output: 1
 * Explanation: 2 = 1 + 1, 1 &times; 1 = 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 10
 * Output: 36
 * Explanation: 10 = 3 + 3 + 4, 3 &times; 3 &times; 4 = 36.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 58
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-break/
// discuss: https://leetcode.com/problems/integer-break/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }

        let mut result = 1;
        let mut n = n;
        while n > 4 {
            result *= 3;
            n -= 3;
        }
        result * n
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_343() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
