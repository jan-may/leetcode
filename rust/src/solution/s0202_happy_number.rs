/**
 * [202] Happy Number
 *
 * Write an algorithm to determine if a number n is happy.
 * A happy number is a number defined by the following process:
 *
 * 	Starting with any positive integer, replace the number by the sum of the squares of its digits.
 * 	Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
 * 	Those numbers for which this process ends in 1 are happy.
 *
 * Return true if n is a happy number, and false if not.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 19
 * Output: true
 * Explanation:
 * 1^2 + 9^2 = 82
 * 8^2 + 2^2 = 68
 * 6^2 + 8^2 = 100
 * 1^2 + 0^2 + 0^2 = 1
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 2
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/happy-number/
// discuss: https://leetcode.com/problems/happy-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        // floyd cycle detection -> fast and slow pointer
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Solution::square_sum(slow);
            fast = Solution::square_sum(Solution::square_sum(fast));
            if slow == fast {
                break;
            }
        }
        slow == 1
    }

    fn square_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {}
}
