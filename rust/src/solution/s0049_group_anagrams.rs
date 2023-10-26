/**
 * [49] Group Anagrams
 *
 * Given an array of strings strs, group the anagrams together. You can return the answer in any order.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *  
 * <strong class="example">Example 1:
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * <strong class="example">Example 2:
 * Input: strs = [""]
 * Output: [[""]]
 * <strong class="example">Example 3:
 * Input: strs = ["a"]
 * Output: [["a"]]
 *  
 * Constraints:
 *
 * 	1 <= strs.length <= 10^4
 * 	0 <= strs[i].length <= 100
 * 	strs[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/group-anagrams/
// discuss: https://leetcode.com/problems/group-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        let mut res = vec![];
        for s in strs.into_iter() {
            let mut key = vec![0; 26];
            for c in s.chars() {
                key[(c as u8 - 'a' as u8) as usize] += 1;
            }
            map.entry(key).or_insert(vec![]).push(s);
        }
        for (_, v) in map.into_iter() {
            res.push(v);
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_49() {}
}
