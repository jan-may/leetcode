/**
 * [1881] Maximum Value after Insertion
 *
 * You are given a very large integer n, represented as a string,​​​​​​ and an integer digit x. The digits in n and the digit x are in the inclusive range [1, 9], and n may represent a negative number.
 * You want to maximize n's numerical value by inserting x anywhere in the decimal representation of n​​​​​​. You cannot insert x to the left of the negative sign.
 *
 * 	For example, if n = 73 and x = 6, it would be best to insert it between 7 and 3, making n = 763.
 * 	If n = -55 and x = 2, it would be best to insert it before the first 5, making n = -255.
 *
 * Return a string representing the maximum value of n​​​​​​ after the insertion.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = "99", x = 9
 * Output: "999"
 * Explanation: The result is the same regardless of where you insert 9.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = "-13", x = 2
 * Output: "-123"
 * Explanation: You can make n one of {-213, -123, -132}, and the largest of those three is -123.
 *
 *  
 * Constraints:
 *
 * 	1 <= n.length <= 10^5
 * 	1 <= x <= 9
 * 	The digits in n​​​ are in the range [1, 9].
 * 	n is a valid representation of an integer.
 * 	In the case of a negative n,​​​​​​ it will begin with '-'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-value-after-insertion/
// discuss: https://leetcode.com/problems/maximum-value-after-insertion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut n = n;
        let mut is_negative = if n.starts_with('-') { true } else { false };
        let mut result = match is_negative {
            true => {
                n.remove(0);
                "-".to_string()
            }
            false => "".to_string(),
        };
        let mut inserted = false;
        for c in n.chars() {
            if !inserted && is_negative && c.to_digit(10).unwrap() > x as u32 {
                result.push_str(&x.to_string());
                inserted = true;
            }
            if !inserted && !is_negative && c.to_digit(10).unwrap() < x as u32 {
                result.push_str(&x.to_string());
                inserted = true;
            }
            result.push(c);
        }
        if !inserted {
            result.push_str(&x.to_string());
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1881() {
        assert_eq!(Solution::max_value("99".to_string(), 9), "999".to_string());
        assert_eq!(
            Solution::max_value("-13".to_string(), 2),
            "-123".to_string()
        );
        assert_eq!(
            Solution::max_value("-132".to_string(), 3),
            "-1323".to_string()
        );
        assert_eq!(
            Solution::max_value("28824579515".to_string(), 8),
            "828824579515".to_string()
        );
    }
}
