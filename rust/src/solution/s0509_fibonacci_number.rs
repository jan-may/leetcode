use std::collections::HashMap;

/**
 * [509] Fibonacci Number
 *
 * The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,
 *
 * F(0) = 0, F(1) = 1
 * F(n) = F(n - 1) + F(n - 2), for n > 1.
 *
 * Given n, calculate F(n).
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 2
 * Output: 1
 * Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 3
 * Output: 2
 * Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 4
 * Output: 3
 * Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fibonacci-number/
// discuss: https://leetcode.com/problems/fibonacci-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Self::dp(n, &mut memo)
    }

    fn dp(i: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        match i {
            0 | 1 => i,
            _ => {
                if let Some(&v) = memo.get(&i) {
                    return v;
                }
                let result = Self::dp(i - 1, memo) + Self::dp(i - 2, memo);
                memo.insert(i, result);
                result
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_509() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
        assert_eq!(Solution::fib(30), 832040);
    }
}
