/**
 * [58] Length of Last Word
 *
 * Given a string s consisting of words and spaces, return the length of the last word in the string.
 * A word is a maximal <span data-keyword="substring-nonempty">substring</span> consisting of non-space characters only.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "Hello World"
 * Output: 5
 * Explanation: The last word is "World" with length 5.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "   fly me   to   the moon  "
 * Output: 4
 * Explanation: The last word is "moon" with length 4.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "luffy is still joyboy"
 * Output: 6
 * Explanation: The last word is "joyboy" with length 6.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of only English letters and spaces ' '.
 * 	There will be at least one word in s.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/length-of-last-word/
// discuss: https://leetcode.com/problems/length-of-last-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut result = 0;
        let iter = s.as_str().split_whitespace();
        while let Some(word) = iter.last() {
            result = word.len() as i32;
            break;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_58() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
