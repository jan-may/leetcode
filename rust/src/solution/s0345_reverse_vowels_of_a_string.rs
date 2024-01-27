/**
 * [345] Reverse Vowels of a String
 *
 * Given a string s, reverse only all the vowels in the string and return it.
 * The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
 *  
 * <strong class="example">Example 1:
 * Input: s = "hello"
 * Output: "holle"
 * <strong class="example">Example 2:
 * Input: s = "leetcode"
 * Output: "leotcede"
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 3 * 10^5
 * 	s consist of printable ASCII characters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-vowels-of-a-string/
// discuss: https://leetcode.com/problems/reverse-vowels-of-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < j {
            if !VOWELS.contains(&chars[i].to_ascii_lowercase()) {
                i += 1;
            } else if
                !VOWELS.contains(&chars[j].to_ascii_lowercase()){
                j -= 1;
            } else {
                chars.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        chars.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_345() {
        assert_eq!(
            Solution::reverse_vowels("hello".to_string()),
            "holle".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }
}
