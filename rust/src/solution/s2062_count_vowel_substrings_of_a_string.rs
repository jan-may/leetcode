/**
 * [2062] Count Vowel Substrings of a String
 *
 * A substring is a contiguous (non-empty) sequence of characters within a string.
 * A vowel substring is a substring that only consists of vowels ('a', 'e', 'i', 'o', and 'u') and has all five vowels present in it.
 * Given a string word, return the number of vowel substrings in word.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: word = "aeiouu"
 * Output: 2
 * Explanation: The vowel substrings of word are as follows (underlined):
 * - "<u>aeiou</u>u"
 * - "<u>aeiouu</u>"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: word = "unicornarihan"
 * Output: 0
 * Explanation: Not all 5 vowels are present, so there are no vowel substrings.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: word = "cuaieuouac"
 * Output: 7
 * Explanation: The vowel substrings of word are as follows (underlined):
 * - "c<u>uaieuo</u>uac"
 * - "c<u>uaieuou</u>ac"
 * - "c<u>uaieuoua</u>c"
 * - "cu<u>aieuo</u>uac"
 * - "cu<u>aieuou</u>ac"
 * - "cu<u>aieuoua</u>c"
 * - "cua<u>ieuoua</u>c"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= word.length <= 100
 * 	word consists of lowercase English letters only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-vowel-substrings-of-a-string/
// discuss: https://leetcode.com/problems/count-vowel-substrings-of-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let mut result = 0;
        for x in 0..word.len() -1 {
            let mut count = 0;
            let (mut a , mut e, mut i, mut o, mut u) = (0,0,0,0,0);
            for j in x..word.len() {
                match word.chars().nth(j).unwrap() {
                    'a' => a = 1,
                    'e' => e = 1,
                    'i' => i = 1,
                    'o' => o = 1,
                    'u' => u = 1,
                    _ => {
                        a = 0;
                        e = 0;
                        i = 0;
                        o = 0;
                        u = 0;
                        break;
                    }
                }
                if a > 0  && e > 0 && i > 0  && o >0  && u > 0 {
                    count += 1;
                }
            }
            result += count;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2062() {
        assert_eq!(Solution::count_vowel_substrings("aeiouu".to_string()), 2);
      assert_eq!(Solution::count_vowel_substrings("unicornarihan".to_string()), 0);
       assert_eq!(Solution::count_vowel_substrings("cuaieuouac".to_string()), 7);
    }
}
