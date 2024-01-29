/**
 * [821] Shortest Distance to a Character
 *
 * Given a string s and a character c that occurs in s, return an array of integers answer where answer.length == s.length and answer[i] is the distance from index i to the closest occurrence of character c in s.
 * The distance between two indices i and j is abs(i - j), where abs is the absolute value function.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "loveleetcode", c = "e"
 * Output: [3,2,1,0,1,0,0,1,2,2,1,0]
 * Explanation: The character 'e' appears at indices 3, 5, 6, and 11 (0-indexed).
 * The closest occurrence of 'e' for index 0 is at index 3, so the distance is abs(0 - 3) = 3.
 * The closest occurrence of 'e' for index 1 is at index 3, so the distance is abs(1 - 3) = 2.
 * For index 4, there is a tie between the 'e' at index 3 and the 'e' at index 5, but the distance is still the same: abs(4 - 3) == abs(4 - 5) = 1.
 * The closest occurrence of 'e' for index 8 is at index 6, so the distance is abs(8 - 6) = 2.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "aaab", c = "b"
 * Output: [3,2,1,0]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^4
 * 	s[i] and c are lowercase English letters.
 * 	It is guaranteed that c occurs at least once in s.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-distance-to-a-character/
// discuss: https://leetcode.com/problems/shortest-distance-to-a-character/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut result = vec![-1; s.len()];

        // Set distances to 0 where the character is found
        for (i, char) in s.char_indices() {
            if char == c {
                result[i] = 0;
            }
        }

        let mut prev = 0;
        let mut current = 0;

        // Find the first occurrence of c
        while current < s.len() && result[current] != 0 {
            current += 1;
        }

        // Update the distances before the first occurrence of c
        for i in 0..current {
            result[i] = (current - i) as i32;
        }
        prev = current;
        current += 1;

        // Update the distances between occurrences of c
        while current < s.len() {
            if result[current] == 0 {
                for i in prev + 1..current {
                    result[i] = std::cmp::min(i - prev, current - i) as i32;
                }
                prev = current;
            }
            current += 1;
        }

        // Update the distances after the last occurrence of c
        for i in prev + 1..s.len() {
            result[i] = (i - prev) as i32;
        }

        result
    }
}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_821() {
        assert_eq!(
            Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        assert_eq!(
            Solution::shortest_to_char("aaab".to_string(), 'b'),
            vec![3, 2, 1, 0]
        );
           assert_eq!(
            Solution::shortest_to_char("abaa".to_string(), 'b'),
            vec![1,0,1,2]
        );
    }
}
