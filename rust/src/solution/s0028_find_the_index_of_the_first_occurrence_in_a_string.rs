/**
 * [28] Find the Index of the First Occurrence in a String
 *
 * Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
 *  
 * <strong class="example">Example 1:
 *
 * Input: haystack = "sadbutsad", needle = "sad"
 * Output: 0
 * Explanation: "sad" occurs at index 0 and 6.
 * The first occurrence is at index 0, so we return 0.
 *
 * <strong class="example">Example 2:
 *
 * Input: haystack = "leetcode", needle = "leeto"
 * Output: -1
 * Explanation: "leeto" did not occur in "leetcode", so we return -1.
 *
 *  
 * Constraints:
 *
 * 	1 <= haystack.length, needle.length <= 10^4
 * 	haystack and needle consist of only lowercase English characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
// discuss: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        for i in 0..haystack.len() {
            if haystack[i..].starts_with(&needle) {
                return i as i32;
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }
}
