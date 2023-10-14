/**
 * [2108] Find First Palindromic String in the Array
 *
 * Given an array of strings words, return the first palindromic string in the array. If there is no such string, return an empty string "".
 * A string is palindromic if it reads the same forward and backward.
 *  
 * <strong class="example">Example 1:
 *
 * Input: words = ["abc","car","ada","racecar","cool"]
 * Output: "ada"
 * Explanation: The first string that is palindromic is "ada".
 * Note that "racecar" is also palindromic, but it is not the first.
 *
 * <strong class="example">Example 2:
 *
 * Input: words = ["notapalindrome","racecar"]
 * Output: "racecar"
 * Explanation: The first and only string that is palindromic is "racecar".
 *
 * <strong class="example">Example 3:
 *
 * Input: words = ["def","ghi"]
 * Output: ""
 * Explanation: There are no palindromic strings, so the empty string is returned.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length <= 100
 * 	words[i] consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
// discuss: https://leetcode.com/problems/find-first-palindromic-string-in-the-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        let mut result = String::new();
        for word in words {
            let chars = word.chars().collect::<Vec<char>>();
            let mut is_palindrome = true;
            for i in 0..chars.len() / 2 {
                if chars[i] != chars[chars.len() - 1 - i] {
                    is_palindrome = false;
                    break;
                }
            }
            if is_palindrome {
                result = word;
                break;
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
    fn test_2108() {
        assert_eq!(
            Solution::first_palindrome(vec![
                "abc".to_string(),
                "car".to_string(),
                "ada".to_string(),
                "racecar".to_string(),
                "cool".to_string()
            ]),
            "ada".to_string()
        );
        assert_eq!(
            Solution::first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]),
            "racecar".to_string()
        );
        assert_eq!(
            Solution::first_palindrome(vec!["def".to_string(), "ghi".to_string()]),
            "".to_string()
        );
    }
}
