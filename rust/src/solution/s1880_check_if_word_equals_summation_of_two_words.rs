/**
 * [1880] Check if Word Equals Summation of Two Words
 *
 * The letter value of a letter is its position in the alphabet starting from 0 (i.e. 'a' -> 0, 'b' -> 1, 'c' -> 2, etc.).
 * The numerical value of some string of lowercase English letters s is the concatenation of the letter values of each letter in s, which is then converted into an integer.
 *
 * 	For example, if s = "acb", we concatenate each letter's letter value, resulting in "021". After converting it, we get 21.
 *
 * You are given three strings firstWord, secondWord, and targetWord, each consisting of lowercase English letters 'a' through 'j' inclusive.
 * Return true if the summation of the numerical values of firstWord and secondWord equals the numerical value of targetWord, or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: firstWord = "acb", secondWord = "cba", targetWord = "cdb"
 * Output: true
 * Explanation:
 * The numerical value of firstWord is "acb" -> "021" -> 21.
 * The numerical value of secondWord is "cba" -> "210" -> 210.
 * The numerical value of targetWord is "cdb" -> "231" -> 231.
 * We return true because 21 + 210 == 231.
 *
 * <strong class="example">Example 2:
 *
 * Input: firstWord = "aaa", secondWord = "a", targetWord = "aab"
 * Output: false
 * Explanation:
 * The numerical value of firstWord is "aaa" -> "000" -> 0.
 * The numerical value of secondWord is "a" -> "0" -> 0.
 * The numerical value of targetWord is "aab" -> "001" -> 1.
 * We return false because 0 + 0 != 1.
 *
 * <strong class="example">Example 3:
 *
 * Input: firstWord = "aaa", secondWord = "a", targetWord = "aaaa"
 * Output: true
 * Explanation:
 * The numerical value of firstWord is "aaa" -> "000" -> 0.
 * The numerical value of secondWord is "a" -> "0" -> 0.
 * The numerical value of targetWord is "aaaa" -> "0000" -> 0.
 * We return true because 0 + 0 == 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= firstWord.length, secondWord.length, targetWord.length <= 8
 * 	firstWord, secondWord, and targetWord consist of lowercase English letters from 'a' to 'j' inclusive.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/
// discuss: https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        Self::get_num(&first_word) + Self::get_num(&second_word) == Self::get_num(&target_word)
    }

    fn get_num(word: &String) -> i32 {
        word.as_bytes()
            .iter()
            .fold(0, |acc, &x| acc * 10 + (x - b'a') as i32)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1880() {
        assert_eq!(
            Solution::is_sum_equal("acb".to_string(), "bca".to_string(), "cdb".to_string()),
            true
        );
        assert_eq!(
            Solution::is_sum_equal("aaa".to_string(), "a".to_string(), "aab".to_string()),
            false
        );
    }
}
