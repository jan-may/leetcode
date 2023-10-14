/**
 * [2235] Add Two Integers
 *
 * Given two integers num1 and num2, return the sum of the two integers.
 *  
 * <strong class="example">Example 1:
 *
 * Input: num1 = 12, num2 = 5
 * Output: 17
 * Explanation: num1 is 12, num2 is 5, and their sum is 12 + 5 = 17, so 17 is returned.
 *
 * <strong class="example">Example 2:
 *
 * Input: num1 = -10, num2 = 4
 * Output: -6
 * Explanation: num1 + num2 = -6, so -6 is returned.
 *
 *  
 * Constraints:
 *
 * 	-100 <= num1, num2 <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-two-integers/
// discuss: https://leetcode.com/problems/add-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2235() {}
}
