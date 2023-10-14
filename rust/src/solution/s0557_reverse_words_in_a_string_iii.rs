/**
 * [557] Reverse Words in a String III
 *
 * Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
 *  
 * <strong class="example">Example 1:
 * Input: s = "Let's take LeetCode contest"
 * Output: "s'teL ekat edoCteeL tsetnoc"
 * <strong class="example">Example 2:
 * Input: s = "God Ding"
 * Output: "doG gniD"
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 5 * 10^4
 * 	s contains printable ASCII characters.
 * 	s does not contain any leading or trailing spaces.
 * 	There is at least one word in s.
 * 	All the words in s are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-words-in-a-string-iii/
// discuss: https://leetcode.com/problems/reverse-words-in-a-string-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::new();
        for mut word in s.split_whitespace() {
            for c in word.chars().rev() {
                result.push(c);
            }
            result.push(' ');
        }
        result.remove(result.len() - 1);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_557() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
        assert_eq!(
            Solution::reverse_words("God Ding".to_string()),
            "doG gniD".to_string()
        );
    }
}
