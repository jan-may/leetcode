/**
 * [383] Ransom Note
 *
 * Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
 * Each letter in magazine can only be used once in ransomNote.
 *  
 * <strong class="example">Example 1:
 * Input: ransomNote = "a", magazine = "b"
 * Output: false
 * <strong class="example">Example 2:
 * Input: ransomNote = "aa", magazine = "ab"
 * Output: false
 * <strong class="example">Example 3:
 * Input: ransomNote = "aa", magazine = "aab"
 * Output: true
 *  
 * Constraints:
 *
 * 	1 <= ransomNote.length, magazine.length <= 10^5
 * 	ransomNote and magazine consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ransom-note/
// discuss: https://leetcode.com/problems/ransom-note/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut magazine_chars = vec![0; 26];
        let mut ransom_note_chars = vec![0; 26];

        for c in magazine.chars() {
            magazine_chars[(c as u8 - 'a' as u8) as usize] += 1;
        }

        for c in ransom_note.chars() {
            ransom_note_chars[(c as u8 - 'a' as u8) as usize] += 1;
        }

        for i in 0..26 {
            if ransom_note_chars[i] > magazine_chars[i] {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383() {
        assert_eq!(
            Solution::can_construct("a".to_string(), "b".to_string()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_string(), "ab".to_string()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_string(), "aab".to_string()),
            true
        );
    }
}
