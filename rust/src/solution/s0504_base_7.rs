/**
 * [504] Base 7
 *
 * Given an integer num, return a string of its base 7 representation.
 *  
 * <strong class="example">Example 1:
 * Input: num = 100
 * Output: "202"
 * <strong class="example">Example 2:
 * Input: num = -7
 * Output: "-10"
 *  
 * Constraints:
 * 
 * 	-10^7 <= num <= 10^7
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/base-7/
// discuss: https://leetcode.com/problems/base-7/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut num = num;
        if num == 0 {
            return "0".to_string();
        }
        let mut result = String::new();
        let mut is_negative = false;
        if num < 0 {
            is_negative = true;
            num = -num;
        }
        while num > 0 {
            result.push_str(&(num % 7).to_string());
            num /= 7;
        }
        if is_negative {
            result.push('-');
        }
        result.chars().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_504() {

    }
}
