/**
 * [507] Perfect Number
 *
 * A <a href="https://en.wikipedia.org/wiki/Perfect_number" target="_blank">perfect number</a> is a positive integer that is equal to the sum of its positive divisors, excluding the number itself. A divisor of an integer x is an integer that can divide x evenly.
 * Given an integer n, return true if n is a perfect number, otherwise return false.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: num = 28
 * Output: true
 * Explanation: 28 = 1 + 2 + 4 + 7 + 14
 * 1, 2, 4, 7, and 14 are all divisors of 28.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: num = 7
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= num <= 10^8
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-number/
// discuss: https://leetcode.com/problems/perfect-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num <= 1 {
            return false;
        }
        let mut sum = 1;
        let mut i = 2;
        while i * i <= num {
            if num % i == 0 {
                sum += i;
                if i * i != num {
                    sum += num / i;
                }
            }
            i += 1;
        }
        sum == num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_507() {
        assert_eq!(Solution::check_perfect_number(28), true);
        assert_eq!(Solution::check_perfect_number(7), false);


    }
}
