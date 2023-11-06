/**
 * [392] Is Subsequence
 *
 * Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
 * A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "<u>a</u>b<u>c</u>d<u>e</u>" while "aec" is not).
 *  
 * <strong class="example">Example 1:
 * Input: s = "abc", t = "ahbgdc"
 * Output: true
 * <strong class="example">Example 2:
 * Input: s = "axc", t = "ahbgdc"
 * Output: false
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 100
 * 	0 <= t.length <= 10^4
 * 	s and t consist only of lowercase English letters.
 *
 *  
 * Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 10^9, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/is-subsequence/
// discuss: https://leetcode.com/problems/is-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        if t.is_empty() {
            return false;
        }
        let mut s_iter = s.chars();
        let mut current_char = s_iter.next();
        for char in t.chars() {
            if Some(char) == current_char {
                current_char = s_iter.next();
                if current_char.is_none() {
                    return true;
                }
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_392() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }
}
