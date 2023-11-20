/**
 * [54] Spiral Matrix
 *
 * Given an m x n matrix, return all elements of the matrix in spiral order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral1.jpg" style="width: 242px; height: 242px;" />
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,2,3,6,9,8,7,4,5]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 10
 * 	-100 <= matrix[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix/
// discuss: https://leetcode.com/problems/spiral-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = vec![];
        let mut i = 0;
        let mut val = 1;
        while { i < m - 1 && i < n - 1 } {
            for x in i..n - i {
                result.push(matrix[i][x]);
                val += 1;
            }
            for y in i + 1..m - i {
                result.push(matrix[y][n - i - 1]);
                val += 1;
            }
            for x in (i..n - i - 1).rev() {
                result.push(matrix[m - i - 1][x]);
                val += 1;
            }
            for y in (i + 1..m - i - 1).rev() {
                result.push(matrix[y][i]);
                val += 1;
            }
            i += 1;
        }
        if m == n && m % 2 == 1 {
            result.push(matrix[m / 2][n / 2]);
        }
        if m < n && m % 2 == 1 {
            for x in i..n - i {
                result.push(matrix[i][x]);
                val += 1;
            }
        }
        if m > n && n % 2 == 1 {
            for y in i..m - i {
                result.push(matrix[y][i]);
                val += 1;
            }
        }
        // trim the result
        result.truncate(m * n);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_54() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
