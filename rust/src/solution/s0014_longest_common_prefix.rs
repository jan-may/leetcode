/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 * If there is no common prefix, return an empty string "".
 *  
 * <strong class="example">Example 1:
 *
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 * <strong class="example">Example 2:
 *
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *  
 * Constraints:
 *
 * 	1 <= strs.length <= 200
 * 	0 <= strs[i].length <= 200
 * 	strs[i] consists of only lowercase English letters.
 *
 */
use std::ops::Add;

pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-prefix/
// discuss: https://leetcode.com/problems/longest-common-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::from("");

        if strs.len() == 0 {
            return result;
        }

        let mut min_len = strs[0].len();
        for i in 1..strs.len() {
            if strs[i].len() < min_len {
                min_len = strs[i].len();
            }
        }

        for i in 0..min_len {
            let mut c = strs[0].chars().nth(i).unwrap();
            for j in 1..strs.len() {
                if strs[j].chars().nth(i).unwrap() != c {
                    return result;
                }
            }
            result = result.add(c.to_string().as_str());
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "dog".to_string(),
                "dog".to_string()
            ]),
            "dog".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "dog".to_string(),
                "dog".to_string(),
                "dog".to_string()
            ]),
            "dog".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "dog".to_string(),
                "dog".to_string(),
                "dog".to_string(),
                "dog".to_string()
            ]),
            "dog".to_string()
        );
    }
}
