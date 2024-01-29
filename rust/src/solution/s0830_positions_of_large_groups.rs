/**
 * [830] Positions of Large Groups
 *
 * In a string <font face="monospace">s</font> of lowercase letters, these letters form consecutive groups of the same character.
 * For example, a string like s = "abbxxxxzyy" has the groups "a", "bb", "xxxx", "z", and "yy".
 * A group is identified by an interval [start, end], where start and end denote the start and end indices (inclusive) of the group. In the above example, "xxxx" has the interval [3,6].
 * A group is considered large if it has 3 or more characters.
 * Return the intervals of every large group sorted in increasing order by start index.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "abbxxxxzzy"
 * Output: [[3,6]]
 * Explanation: "xxxx" is the only large group with start index 3 and end index 6.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "abc"
 * Output: []
 * Explanation: We have groups "a", "b", and "c", none of which are large groups.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "abcdddeeeeaabbbcd"
 * Output: [[3,5],[6,9],[12,14]]
 * Explanation: The large groups are "ddd", "eeee", and "bbb".
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s contains lowercase English letters only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/positions-of-large-groups/
// discuss: https://leetcode.com/problems/positions-of-large-groups/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {

        let mut result: Vec<Vec<i32>> = vec![];

        // first character
        let mut current = ' ';
        let mut last = ' ';
        let mut counter = 0;

        for (i, char) in s.char_indices() {
            if (i == 0) {
                current = char;
                last = char;
                counter = 1;
                continue;
            }

            if (char == current) {
                counter += 1;
            } else {
                if (counter >= 3) {
                    result.push(vec![i as i32 - counter as i32, i as i32 - 1]);
                }
                last = current;
                current = char;
                counter = 1;
            }
        }
        // last character
        if (counter >= 3) {
            result.push(vec![s.len() as i32 - counter as i32, s.len() as i32 - 1]);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_830() {
        // assert_eq!(
        //     Solution::large_group_positions("abbxxxxzzy".to_string()),
        //     vec![vec![3, 6]]
        // );
        // assert_eq!(
        //     Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
        //     vec![vec![3, 5], vec![6, 9], vec![12, 14]]
        // );
        assert_eq!(
            Solution::large_group_positions("aaa".to_string()),
            vec![vec![0,2]]
        );
    }
}
