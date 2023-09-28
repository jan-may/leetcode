/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest <span data-keyword="palindromic-string">palindromic</span> <span data-keyword="substring-nonempty">substring</span> in s.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut result = String::new();
        let mut max_len = 0;
        let mut start = 0;
        let mut end = 0;
        let mut i = 0;
        let mut j = 0;
        let mut len = 0;
        let mut is_palindrome = false;
        let mut chars: Vec<char> = s.chars().collect();
        while i < chars.len() {
            j = i + 1;
            while j < chars.len() {
                is_palindrome = true;
                len = j - i + 1;
                for k in 0..len / 2 {
                    if chars[i + k] != chars[j - k] {
                        is_palindrome = false;
                        break;
                    }
                }
                if is_palindrome && len > max_len {
                    max_len = len;
                    start = i;
                    end = j;
                }
                j += 1;
            }
            i += 1;
        }
        for k in start..=end {
            result.push(chars[k]);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert!(Solution::longest_palindrome("babad".to_string()) == "bab".to_string());
        assert!(Solution::longest_palindrome("cbbd".to_string()) == "bb".to_string());
        assert!(Solution::longest_palindrome("a".to_string()) == "a".to_string());
        assert!(Solution::longest_palindrome("ac".to_string()) == "a".to_string());
    }
}
