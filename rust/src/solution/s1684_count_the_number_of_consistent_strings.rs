/**
 * [1684] Count the Number of Consistent Strings
 *
 * You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.
 * Return the number of consistent strings in the array words.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
 * Output: 2
 * Explanation: Strings "aaab" and "baa" are consistent since they only contain characters 'a' and 'b'.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"]
 * Output: 7
 * Explanation: All strings are consistent.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: allowed = "cad", words = ["cc","acd","b","ba","bac","bad","ac","d"]
 * Output: 4
 * Explanation: Strings "cc", "acd", "ac", and "d" are consistent.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 10^4
 * 	1 <= allowed.length <=^ 26
 * 	1 <= words[i].length <= 10
 * 	The characters in allowed are distinct.
 * 	words[i] and allowed contain only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-consistent-strings/
// discuss: https://leetcode.com/problems/count-the-number-of-consistent-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        let map = allowed.chars().fold([false; 26], |mut acc, c| {
            acc[c as usize - 'a' as usize] = true;
            acc
        });
        for word in words {
            if word.chars().all(|c| map[c as usize - 'a' as usize]) {
                count += 1;
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1684() {
        assert_eq!(
            Solution::count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            Solution::count_consistent_strings(
                "abc".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ]
            ),
            7
        );
        assert_eq!(
            Solution::count_consistent_strings(
                "cad".to_string(),
                vec![
                    "cc".to_string(),
                    "acd".to_string(),
                    "b".to_string(),
                    "ba".to_string(),
                    "bac".to_string(),
                    "bad".to_string(),
                    "ac".to_string(),
                    "d".to_string()
                ]
            ),
            4
        );

    }
}
