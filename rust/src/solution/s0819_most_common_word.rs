/**
 * [819] Most Common Word
 *
 * Given a string paragraph and a string array of the banned words banned, return the most frequent word that is not banned. It is guaranteed there is at least one word that is not banned, and that the answer is unique.
 * The words in paragraph are case-insensitive and the answer should be returned in lowercase.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.", banned = ["hit"]
 * Output: "ball"
 * Explanation: 
 * "hit" occurs 3 times, but it is a banned word.
 * "ball" occurs twice (and no other word does), so it is the most frequent non-banned word in the paragraph. 
 * Note that words in the paragraph are not case sensitive,
 * that punctuation is ignored (even if adjacent to words, such as "ball,"), 
 * and that "hit" isn't the answer even though it occurs more because it is banned.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: paragraph = "a.", banned = []
 * Output: "a"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= paragraph.length <= 1000
 * 	paragraph consists of English letters, space ' ', or one of the symbols: "!?',;.".
 * 	0 <= banned.length <= 100
 * 	1 <= banned[i].length <= 10
 * 	banned[i] consists of only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/most-common-word/
// discuss: https://leetcode.com/problems/most-common-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut map = std::collections::HashMap::new();
        let mut max = 0;
        let mut result = String::new();
        let mut banned_set = std::collections::HashSet::new();
        for word in banned {
            banned_set.insert(word);
        }
        let mut word = String::new();
        for c in paragraph.chars() {
            if c.is_alphabetic() {
                word.push(c.to_ascii_lowercase());
            } else {
                if !word.is_empty() && !banned_set.contains(&word) {
                    let count = map.entry(word.clone()).or_insert(0);
                    *count += 1;
                    if *count > max {
                        max = *count;
                        result = word.clone();
                    }
                }
                word.clear();
            }
        }
        if !word.is_empty() && !banned_set.contains(&word) {
            let count = map.entry(word.clone()).or_insert(0);
            *count += 1;
            if *count > max {
                max = *count;
                result = word.clone();
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
    fn test_819() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball".to_string()
        );
        assert_eq!(
            Solution::most_common_word("a.".to_string(), vec![]),
            "a".to_string()
        );
    }
}
