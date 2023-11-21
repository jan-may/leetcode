/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
 * A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png" style="width: 300px; height: 243px;" />
 *  
 * <strong class="example">Example 1:
 *
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 * <strong class="example">Example 2:
 *
 * Input: digits = ""
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: digits = "2"
 * Output: ["a","b","c"]
 *
 *  
 * Constraints:
 *
 * 	0 <= digits.length <= 4
 * 	digits[i] is a digit in the range ['2', '9'].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let values = vec![
            ('2', vec!["a", "b", "c"]),
            ('3', vec!["d", "e", "f"]),
            ('4', vec!["g", "h", "i"]),
            ('5', vec!["j", "k", "l"]),
            ('6', vec!["m", "n", "o"]),
            ('7', vec!["p", "q", "r", "s"]),
            ('8', vec!["t", "u", "v"]),
            ('9', vec!["w", "x", "y", "z"]),
        ];
        let mut result = vec![];
        for digit in digits.chars() {
            let mut tmp = vec![];
            for (key, value) in values.iter() {
                if key == &digit {
                    if result.is_empty() {
                        tmp = value.iter().map(|s| s.to_string()).collect();
                    } else {
                        for s in result.iter() {
                            for v in value.iter() {
                                tmp.push(format!("{}{}", s, v));
                            }
                        }
                    }
                    break;
                }
            }
            result = tmp;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            vec![] as Vec<String>
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }
}
