/**
 * [342] Power of Four
 *
 * Given an integer n, return true if it is a power of four. Otherwise, return false.
 * An integer n is a power of four, if there exists an integer x such that n == 4^x.
 *  
 * <strong class="example">Example 1:
 * Input: n = 16
 * Output: true
 * <strong class="example">Example 2:
 * Input: n = 5
 * Output: false
 * <strong class="example">Example 3:
 * Input: n = 1
 * Output: true
 *  
 * Constraints:
 *
 * 	-2^31 <= n <= 2^31 - 1
 *
 *  
 * Follow up: Could you solve it without loops/recursion?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/power-of-four/
// discuss: https://leetcode.com/problems/power-of-four/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_342() {
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(5), false);
        assert_eq!(Solution::is_power_of_four(1), true);
    }
}
