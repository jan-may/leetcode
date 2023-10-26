use std::collections::HashMap;

/**
 * [205] Isomorphic Strings
 *
 * Given two strings s and t, determine if they are isomorphic.
 * Two strings s and t are isomorphic if the characters in s can be replaced to get t.
 * All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
 *  
 * <strong class="example">Example 1:
 * Input: s = "egg", t = "add"
 * Output: true
 * <strong class="example">Example 2:
 * Input: s = "foo", t = "bar"
 * Output: false
 * <strong class="example">Example 3:
 * Input: s = "paper", t = "title"
 * Output: true
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 5 * 10^4
 * 	t.length == s.length
 * 	s and t consist of any valid ascii character.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/isomorphic-strings/
// discuss: https://leetcode.com/problems/isomorphic-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map: HashMap<char, char> = std::collections::HashMap::new();
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();
        for _ in 0..s.len() {
            let s_char = s_chars.next().unwrap();
            let t_char = t_chars.next().unwrap();
            if map.contains_key(&s_char) {
                if map[&s_char] != t_char {
                    return false;
                }
            } else {
                if map.values().any(|&v| v == t_char) {
                    return false;
                }
                map.insert(s_char, t_char);
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_205() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_string(), "add".to_string()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
    }
}
