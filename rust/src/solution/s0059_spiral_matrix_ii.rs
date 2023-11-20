/**
 * [59] Spiral Matrix II
 *
 * Given a positive integer n, generate an n x n matrix filled with elements from 1 to n^2 in spiral order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiraln.jpg" style="width: 242px; height: 242px;" />
 * Input: n = 3
 * Output: [[1,2,3],[8,9,4],[7,6,5]]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1
 * Output: [[1]]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 20
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix-ii/
// discuss: https://leetcode.com/problems/spiral-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![vec![1]];
        }
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];
        let mut i = 0;
        let mut val = 1;
        while { i < n - 1 } {
            for x in i..n - i {
                matrix[i][x] = val;
                val += 1;
            }
            for y in i + 1..n - i {
                matrix[y][n - i - 1] = val;
                val += 1;
            }
            for x in (i..n - i - 1).rev() {
                matrix[n - i - 1][x] = val;
                val += 1;
            }
            for y in (i + 1..n - i - 1).rev() {
                matrix[y][i] = val;
                val += 1;
            }
            i += 1;
        }
        matrix
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_59() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
