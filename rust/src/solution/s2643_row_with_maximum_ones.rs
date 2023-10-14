/**
 * [2643] Row With Maximum Ones
 *
 * Given a m x n binary matrix mat, find the 0-indexed position of the row that contains the maximum count of ones, and the number of ones in that row.
 * In case there are multiple rows that have the maximum count of ones, the row with the smallest row number should be selected.
 * Return an array containing the index of the row, and the number of ones in it.
 *  
 * <strong class="example">Example 1:
 *
 * Input: mat = [[0,1],[1,0]]
 * Output: [0,1]
 * Explanation: Both rows have the same number of 1's. So we return the index of the smaller row, 0, and the maximum count of ones (1). So, the answer is [0,1].
 *
 * <strong class="example">Example 2:
 *
 * Input: mat = [[0,0,0],[0,1,1]]
 * Output: [1,2]
 * Explanation: The row indexed 1 has the maximum count of ones (2). So we return its index, 1, and the count. So, the answer is [1,2].
 *
 * <strong class="example">Example 3:
 *
 * Input: mat = [[0,0],[1,1],[0,0]]
 * Output: [1,2]
 * Explanation: The row indexed 1 has the maximum count of ones (2). So the answer is [1,2].
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 100
 * 	mat[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/row-with-maximum-ones/
// discuss: https://leetcode.com/problems/row-with-maximum-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0, 0];
        let mut max = 0;
        for i in 0..mat.len() {
            let mut count = 0;
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 {
                    count += 1;
                }
            }
            if count > max {
                max = count;
                result[0] = i as i32;
                result[1] = count;
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
    fn test_2643() {
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]),
            vec![0, 1]
        );
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            vec![1, 2]
        );
    }
}
