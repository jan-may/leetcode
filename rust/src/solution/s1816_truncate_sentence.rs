/**
 * [1816] Truncate Sentence
 *
 * A sentence is a list of words that are separated by a single space with no leading or trailing spaces. Each of the words consists of only uppercase and lowercase English letters (no punctuation).
 *
 * 	For example, "Hello World", "HELLO", and "hello world hello world" are all sentences.
 *
 * You are given a sentence s​​​​​​ and an integer k​​​​​​. You want to truncate s​​​​​​ such that it contains only the first k​​​​​​ words. Return s​​​​​​ after truncating it.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "Hello how are you Contestant", k = 4
 * Output: "Hello how are you"
 * Explanation:
 * The words in s are ["Hello", "how" "are", "you", "Contestant"].
 * The first 4 words are ["Hello", "how", "are", "you"].
 * Hence, you should return "Hello how are you".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "What is the solution to this problem", k = 4
 * Output: "What is the solution"
 * Explanation:
 * The words in s are ["What", "is" "the", "solution", "to", "this", "problem"].
 * The first 4 words are ["What", "is", "the", "solution"].
 * Hence, you should return "What is the solution".
 * <strong class="example">Example 3:
 *
 * Input: s = "chopper is not a tanuki", k = 5
 * Output: "chopper is not a tanuki"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	k is in the range [1, the number of words in s].
 * 	s consist of only lowercase and uppercase English letters and spaces.
 * 	The words in s are separated by a single space.
 * 	There are no leading or trailing spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/truncate-sentence/
// discuss: https://leetcode.com/problems/truncate-sentence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut result = String::new();
        let mut count = 0;
        for c in s.chars() {
            if c == ' ' {
                count += 1;
            }
            if count == k {
                break;
            }
            result.push(c);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1816() {
        assert_eq!(
            Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4),
            "Hello how are you".to_string()
        );
        assert_eq!(
            Solution::truncate_sentence("What is the solution to this problem".to_string(), 4),
            "What is the solution".to_string()
        );
    }
}
