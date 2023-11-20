/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 * Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
 *  
 * <strong class="example">Example 1:
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 * <strong class="example">Example 2:
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *  
 * Constraints:
 *
 * 	1 <= num1.length, num2.length <= 200
 * 	num1 and num2 consist of digits only.
 * 	Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/multiply-strings/
// discuss: https://leetcode.com/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let mut result = String::new();

        let mut num1: Vec<u8> = num1
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let mut num2: Vec<u8> = num2
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let mut temp = vec![0; num1.len() + num2.len()];

        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                let mul = num1[i] * num2[j];
                let p1 = i + j;
                let p2 = i + j + 1;
                let sum = mul + temp[p2];

                temp[p1] += sum / 10;
                temp[p2] = sum % 10;
            }
        }

        for i in temp {
            if !(result.len() == 0 && i == 0) {
                result.push_str(&i.to_string());
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
        assert_eq!(
            Solution::multiply("789654".to_string(), "123".to_string()),
            "97127442".to_string()
        );
    }
}
