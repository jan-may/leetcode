/**
 * [73] Set Matrix Zeroes
 *
 * Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
 * You must do it <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in place</a>.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/mat1.jpg" style="width: 450px; height: 169px;" />
 * Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
 * Output: [[1,0,1],[0,0,0],[1,0,1]]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/mat2.jpg" style="width: 450px; height: 137px;" />
 * Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
 * Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[0].length
 * 	1 <= m, n <= 200
 * 	-2^31 <= matrix[i][j] <= 2^31 - 1
 *
 *  
 * Follow up:
 *
 * 	A straightforward solution using O(mn) space is probably a bad idea.
 * 	A simple improvement uses O(m + n) space, but still not the best solution.
 * 	Could you devise a constant space solution?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/set-matrix-zeroes/
// discuss: https://leetcode.com/problems/set-matrix-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut first_row_zero = false;
        let mut first_col_zero = false;
        let row_len = matrix.len();
        let col_len = matrix[0].len();

        for i in 0..row_len {
            if matrix[i][0] == 0 {
                first_col_zero = true;
                break;
            }
        }

        for j in 0..col_len {
            if matrix[0][j] == 0 {
                first_row_zero = true;
                break;
            }
        }

        for i in 1..row_len {
            for j in 1..col_len {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0
                }
            }
        }

        for i in 1..row_len {
            for j in 1..col_len {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0
                }
            }
        }

        if first_row_zero {
            for j in 0..col_len {
                matrix[0][j] = 0
            }
        }

        if first_col_zero {
            for i in 0..row_len {
                matrix[i][0] = 0
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_73() {
        Solution::set_zeroes(&mut vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
    }
}
