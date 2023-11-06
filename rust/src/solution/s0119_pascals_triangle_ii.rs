use std::collections::HashMap;

/**
 * [119] Pascal's Triangle II
 *
 * Given an integer rowIndex, return the rowIndex^th (0-indexed) row of the Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" />
 *  
 * <strong class="example">Example 1:
 * Input: rowIndex = 3
 * Output: [1,3,3,1]
 * <strong class="example">Example 2:
 * Input: rowIndex = 0
 * Output: [1]
 * <strong class="example">Example 3:
 * Input: rowIndex = 1
 * Output: [1,1]
 *  
 * Constraints:
 *
 * 	0 <= rowIndex <= 33
 *
 *  
 * Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle-ii/
// discuss: https://leetcode.com/problems/pascals-triangle-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
        let mut result = vec![];
        for j in 0..=row_index as usize {
            result.push(
                Solution::helper(row_index as usize, row_index as usize, j, &mut memo) as i32,
            );
        }
        result
    }

    fn helper(
        num_rows: usize,
        row: usize,
        col: usize,
        mut memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if row == 0 || col == 0 || row == col {
            return 1;
        }
        if let Some(&v) = memo.get(&(row, col)) {
            return v;
        }
        let result = Solution::helper(num_rows, row - 1, col - 1, memo)
            + Solution::helper(num_rows, row - 1, col, memo);
        memo.insert((row, col), result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_119() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
