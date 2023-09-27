/**
 * [69] Sqrt(x)
 *
 * Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
 * You must not use any built-in exponent function or operator.
 *
 * 	For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: x = 4
 * Output: 2
 * Explanation: The square root of 4 is 2, so we return 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: x = 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
 *
 *  
 * Constraints:
 *
 * 	0 <= x <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sqrtx/
// discuss: https://leetcode.com/problems/sqrtx/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left: u64 = 0;
        let mut mid: u64;
        let x = x as u64;
        let mut right: u64 = x + 1;

        while left != right - 1 {
            mid = (left + right) / 2;
            if mid * mid <= x {
                left = mid;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
        assert_eq!(Solution::my_sqrt(10), 3);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
