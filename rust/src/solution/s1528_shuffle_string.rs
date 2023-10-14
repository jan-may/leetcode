/**
 * [1528] Shuffle String
 *
 * You are given a string s and an integer array indices of the same length. The string s will be shuffled such that the character at the i^th position moves to indices[i] in the shuffled string.
 * Return the shuffled string.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/09/q1.jpg" style="width: 321px; height: 243px;" />
 * Input: s = "codeleet", indices = [4,5,6,7,0,2,1,3]
 * Output: "leetcode"
 * Explanation: As shown, "codeleet" becomes "leetcode" after shuffling.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abc", indices = [0,1,2]
 * Output: "abc"
 * Explanation: After shuffling, each character remains in its position.
 *
 *  
 * Constraints:
 *
 * 	s.length == indices.length == n
 * 	1 <= n <= 100
 * 	s consists of only lowercase English letters.
 * 	0 <= indices[i] < n
 * 	All values of indices are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shuffle-string/
// discuss: https://leetcode.com/problems/shuffle-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![' '; s.len()];
        for (i, c) in s.chars().enumerate() {
            result[indices[i] as usize] = c;
        }
        result.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1528() {
        assert_eq!(
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(
            Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string()
        );
    }
}
