/**
 * [387] First Unique Character in a String
 *
 * Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.
 *  
 * <strong class="example">Example 1:
 * Input: s = "leetcode"
 * Output: 0
 * <strong class="example">Example 2:
 * Input: s = "loveleetcode"
 * Output: 2
 * <strong class="example">Example 3:
 * Input: s = "aabb"
 * Output: -1
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/first-unique-character-in-a-string/
// discuss: https://leetcode.com/problems/first-unique-character-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = [0; 26];
        for c in s.chars() {
            map[c as usize - 'a' as usize] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if map[c as usize - 'a' as usize] == 1 {
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
    fn test_387() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }
}
