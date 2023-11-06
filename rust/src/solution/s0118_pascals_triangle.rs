use std::collections::HashMap;

/**
 * [118] Pascal's Triangle
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" />
 *  
 * <strong class="example">Example 1:
 * Input: numRows = 5
 * Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 * <strong class="example">Example 2:
 * Input: numRows = 1
 * Output: [[1]]
 *  
 * Constraints:
 *
 * 	1 <= numRows <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle/
// discuss: https://leetcode.com/problems/pascals-triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate(num_rows: usize) -> Vec<Vec<usize>> {
        let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
        let mut result = vec![];
        for i in 0..num_rows {
            let mut row = vec![];
            for j in 0..=i {
                row.push(Solution::helper(num_rows, i, j, &mut memo));
            }
            result.push(row);
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
    fn test_118() {
        let start = std::time::Instant::now();
        let result = Solution::generate(50);
        println!("{:?}", start.elapsed());
    }
}
