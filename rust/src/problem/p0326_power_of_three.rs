/**
 * [326] Power of Three
 *
 * Given an integer n, return true if it is a power of three. Otherwise, return false.
 * An integer n is a power of three, if there exists an integer x such that n == 3^x.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 27
 * Output: true
 * Explanation: 27 = 3^3
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 0
 * Output: false
 * Explanation: There is no x where 3^x = 0.
 *
 * <strong class="example">Example 3:
 *
 * Input: n = -1
 * Output: false
 * Explanation: There is no x where 3^x = (-1).
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= n <= 2^31 - 1
 *
 *  
 * Follow up: Could you solve it without loops/recursion?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/power-of-three/
// discuss: https://leetcode.com/problems/power-of-three/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n== 1 || n > 0  && n % 3 == 0 && n == 3_i32.pow((n as f64).log(3.0).round() as u32)
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_326() {
        assert_eq!(Solution::is_power_of_three(27), true);
        assert_eq!(Solution::is_power_of_three(0), false);
        assert_eq!(Solution::is_power_of_three(-1), false);
        assert_eq!(Solution::is_power_of_three(9), true);
        assert_eq!(Solution::is_power_of_three(1), true);
    }
}
