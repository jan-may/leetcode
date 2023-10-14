/**
 * [2586] Count the Number of Vowel Strings in Range
 *
 * You are given a 0-indexed array of string words and two integers left and right.
 * A string is called a vowel string if it starts with a vowel character and ends with a vowel character where vowel characters are 'a', 'e', 'i', 'o', and 'u'.
 * Return the number of vowel strings words[i] where i belongs to the inclusive range [left, right].
 *  
 * <strong class="example">Example 1:
 *
 * Input: words = ["are","amy","u"], left = 0, right = 2
 * Output: 2
 * Explanation:
 * - "are" is a vowel string because it starts with 'a' and ends with 'e'.
 * - "amy" is not a vowel string because it does not end with a vowel.
 * - "u" is a vowel string because it starts with 'u' and ends with 'u'.
 * The number of vowel strings in the mentioned range is 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: words = ["hey","aeo","mu","ooo","artro"], left = 1, right = 4
 * Output: 3
 * Explanation:
 * - "aeo" is a vowel string because it starts with 'a' and ends with 'o'.
 * - "mu" is not a vowel string because it does not start with a vowel.
 * - "ooo" is a vowel string because it starts with 'o' and ends with 'o'.
 * - "artro" is a vowel string because it starts with 'a' and ends with 'o'.
 * The number of vowel strings in the mentioned range is 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 1000
 * 	1 <= words[i].length <= 10
 * 	words[i] consists of only lowercase English letters.
 * 	0 <= left <= right < words.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-vowel-strings-in-range/
// discuss: https://leetcode.com/problems/count-the-number-of-vowel-strings-in-range/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut result = 0;
        for (i, word) in words.iter().enumerate() {
            // first and last char is vowel
            if word.starts_with(|c| "aeiou".contains(c))
                && word.ends_with(|c| "aeiou".contains(c) && i as i32 >= left && i as i32 <= right)
            {
                result += 1;
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
    fn test_2586() {
        assert_eq!(
            Solution::vowel_strings(
                vec!["are".to_string(), "amy".to_string(), "u".to_string()],
                0,
                2
            ),
            2
        );
        assert_eq!(
            Solution::vowel_strings(
                vec![
                    "hey".to_string(),
                    "aeo".to_string(),
                    "mu".to_string(),
                    "ooo".to_string(),
                    "artro".to_string()
                ],
                1,
                4
            ),
            3
        );
    }
}
