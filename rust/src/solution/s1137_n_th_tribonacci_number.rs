use std::collections::HashMap;

/**
 * [1137] N-th Tribonacci Number
 *
 * The Tribonacci sequence Tn is defined as follows:
 *
 * T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
 *
 * Given n, return the value of Tn.
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: n = 4
 * Output: 4
 * Explanation:
 * T_3 = 0 + 1 + 1 = 2
 * T_4 = 1 + 1 + 2 = 4
 *
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: n = 25
 * Output: 1389537
 *
 *
 *  
 * Constraints:
 *
 *
 * 	0 <= n <= 37
 * 	The answer is guaranteed to fit within a 32-bit integer, ie. answer <= 2^31 - 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-th-tribonacci-number/
// discuss: https://leetcode.com/problems/n-th-tribonacci-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Self::dp(n, &mut memo)
    }

    fn dp(i: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        match i {
            0 => 0,
            1 | 2 => 1,
            _ => {
                if let Some(&v) = memo.get(&i) {
                    return v;
                }
                let result = Self::dp(i - 1, memo) + Self::dp(i - 2, memo) + Self::dp(i - 3, memo);
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
    fn test_1137() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
