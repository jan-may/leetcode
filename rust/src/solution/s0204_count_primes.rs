/**
 * [204] Count Primes
 *
 * Given an integer n, return the number of prime numbers that are strictly less than n.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 10
 * Output: 4
 * Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 0
 * Output: 0
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 1
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 5 * 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-primes/
// discuss: https://leetcode.com/problems/count-primes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut primes = vec![true; n as usize];
        for i in 2..(n as f64).sqrt() as i32 + 1 {
            if primes[i as usize] {
                let mut j = i * i;
                while j < n {
                    primes[j as usize] = false;
                    j += i;
                }
            }
        }
        let sum = primes.iter().filter(|&&x| x).count() as i32;
        match n {
            0 | 1 => 0,
            _ => sum - 2,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_204() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(1), 0);
    }
}
