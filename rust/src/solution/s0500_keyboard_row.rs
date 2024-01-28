/**
 * [500] Keyboard Row
 *
 * Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.
 * In the American keyboard:
 * 
 * 	the first row consists of the characters "qwertyuiop",
 * 	the second row consists of the characters "asdfghjkl", and
 * 	the third row consists of the characters "zxcvbnm".
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/10/12/keyboard.png" style="width: 800px; max-width: 600px; height: 267px;" />
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words = ["Hello","Alaska","Dad","Peace"]
 * Output: ["Alaska","Dad"]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: words = ["omk"]
 * Output: []
 * 
 * <strong class="example">Example 3:
 * 
 * Input: words = ["adsdf","sfd"]
 * Output: ["adsdf","sfd"]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 20
 * 	1 <= words[i].length <= 100
 * 	words[i] consists of English letters (both lowercase and uppercase). 
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/keyboard-row/
// discuss: https://leetcode.com/problems/keyboard-row/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const FIRST_ROW: &str = "qwertyuiop";
const SECOND_ROW: &str = "asdfghjkl";
const THIRD_ROW: &str = "zxcvbnm";

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut valid_words = vec![];
        for word in words{
            let check_word = word.to_lowercase();
            if check_word.chars().all(|c| FIRST_ROW.contains(c)) ||
                check_word.chars().all(|c| SECOND_ROW.contains(c)) ||
                check_word.chars().all(|c| THIRD_ROW.contains(c)){
                valid_words.push(word);
            }
        }
        valid_words
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_500() {
        assert_eq!(Solution::find_words(vec!["Hello".to_string(),"Alaska".to_string(),"Dad".to_string(),"Peace".to_string()]), vec!["Alaska".to_string(),"Dad".to_string()]
        );
    }
}
