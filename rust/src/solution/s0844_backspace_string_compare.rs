/**
 * [844] Backspace String Compare
 *
 * Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means a backspace character.
 * Note that after backspacing an empty text, the text will continue empty.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "ab#c", t = "ad#c"
 * Output: true
 * Explanation: Both s and t become "ac".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "ab##", t = "c#d#"
 * Output: true
 * Explanation: Both s and t become "".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "a#c", t = "b"
 * Output: false
 * Explanation: s becomes "c" while t becomes "b".
 *
 *  
 * Constraints:
 *
 * 	<span>1 <= s.length, t.length <= 200</span>
 * 	<span>s and t only contain lowercase letters and '#' characters.</span>
 *
 *  
 * Follow up: Can you solve it in O(n) time and O(1) space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/backspace-string-compare/
// discuss: https://leetcode.com/problems/backspace-string-compare/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_stack: Vec<char> = vec![];
        let mut t_stack: Vec<char> = vec![];
        for c in s.chars() {
            if c == '#' {
                s_stack.pop();
            } else {
                s_stack.push(c);
            }
        }
        for c in t.chars() {
            if c == '#' {
                t_stack.pop();
            } else {
                t_stack.push(c);
            }
        }
        s_stack == t_stack
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_844() {
        assert_eq!(
            Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a#c".to_string(), "b".to_string()),
            false
        );
    }
}
