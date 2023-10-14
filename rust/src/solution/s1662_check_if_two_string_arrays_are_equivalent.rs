/**
 * [1662] Check If Two String Arrays are Equivalent
 *
 * Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.
 * A string is represented by an array if the array elements concatenated in order forms the string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: word1 = ["ab", "c"], word2 = ["a", "bc"]
 * Output: true
 * Explanation:
 * word1 represents string "ab" + "c" -> "abc"
 * word2 represents string "a" + "bc" -> "abc"
 * The strings are the same, so return true.
 * <strong class="example">Example 2:
 *
 * Input: word1 = ["a", "cb"], word2 = ["ab", "c"]
 * Output: false
 *
 * <strong class="example">Example 3:
 *
 * Input: word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= word1.length, word2.length <= 10^3
 * 	1 <= word1[i].length, word2[i].length <= 10^3
 * 	1 <= sum(word1[i].length), sum(word2[i].length) <= 10^3
 * 	word1[i] and word2[i] consist of lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
// discuss: https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1662() {
        assert_eq!(
            Solution::array_strings_are_equal(vec_string!["ab", "c"], vec_string!["a", "bc"]),
            true
        );
        assert_eq!(
            Solution::array_strings_are_equal(vec_string!["a", "cb"], vec_string!["ab", "c"]),
            false
        );
        assert_eq!(
            Solution::array_strings_are_equal(
                vec_string!["abc", "d", "defg"],
                vec_string!["abcddefg"]
            ),
            true
        );
    }
}
