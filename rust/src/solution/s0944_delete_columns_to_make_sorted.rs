/**
 * [944] Delete Columns to Make Sorted
 *
 * You are given an array of n strings strs, all of the same length.
 * The strings can be arranged such that there is one on each line, making a grid.
 *
 * 	For example, strs = ["abc", "bce", "cae"] can be arranged as follows:
 *
 * abc
 * bce
 * cae
 *
 * You want to delete the columns that are not sorted lexicographically. In the above example (0-indexed), columns 0 ('a', 'b', 'c') and 2 ('c', 'e', 'e') are sorted, while column 1 ('b', 'c', 'a') is not, so you would delete column 1.
 * Return the number of columns that you will delete.
 *  
 * <strong class="example">Example 1:
 *
 * Input: strs = ["cba","daf","ghi"]
 * Output: 1
 * Explanation: The grid looks as follows:
 *   cba
 *   daf
 *   ghi
 * Columns 0 and 2 are sorted, but column 1 is not, so you only need to delete 1 column.
 *
 * <strong class="example">Example 2:
 *
 * Input: strs = ["a","b"]
 * Output: 0
 * Explanation: The grid looks as follows:
 *   a
 *   b
 * Column 0 is the only column and is sorted, so you will not delete any columns.
 *
 * <strong class="example">Example 3:
 *
 * Input: strs = ["zyx","wvu","tsr"]
 * Output: 3
 * Explanation: The grid looks as follows:
 *   zyx
 *   wvu
 *   tsr
 * All 3 columns are not sorted, so you will delete all 3.
 *
 *  
 * Constraints:
 *
 * 	n == strs.length
 * 	1 <= n <= 100
 * 	1 <= strs[i].length <= 1000
 * 	strs[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-columns-to-make-sorted/
// discuss: https://leetcode.com/problems/delete-columns-to-make-sorted/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut result = 0;
        let len = strs[0].len();

        for i in 0..len {
            let mut my_strs = vec![];
            for j in 0..strs.len() {
                my_strs.push(strs[j].chars().nth(i).unwrap());
            }
            // check if the vector is sorted
            for word in my_strs.windows(2) {
                if word[0] > word[1] {
                    result += 1;
                    break;
                }
            }
        }
        result
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_944() {}
}
