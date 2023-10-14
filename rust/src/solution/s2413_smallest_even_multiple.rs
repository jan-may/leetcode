/**
 * [2413] Smallest Even Multiple
 *
 * Given a positive integer n, return the smallest positive integer that is a multiple of both 2 and n.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 5
 * Output: 10
 * Explanation: The smallest multiple of both 5 and 2 is 10.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 6
 * Output: 6
 * Explanation: The smallest multiple of both 6 and 2 is 6. Note that a number is a multiple of itself.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 150
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-even-multiple/
// discuss: https://leetcode.com/problems/smallest-even-multiple/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        return match n % 2 {
            0 => n,
            _ => n * 2,
        };
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2413() {
        assert_eq!(Solution::smallest_even_multiple(0), 0);
        assert_eq!(Solution::smallest_even_multiple(5), 10);
        assert_eq!(Solution::smallest_even_multiple(6), 6);
        assert_eq!(Solution::smallest_even_multiple(7), 14);
        assert_eq!(Solution::smallest_even_multiple(8), 8);
        assert_eq!(Solution::smallest_even_multiple(9), 18);
        assert_eq!(Solution::smallest_even_multiple(116273), 232546);
    }
}
