use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * [1329] Sort the Matrix Diagonally
 *
 * A matrix diagonal is a diagonal line of cells starting from some cell in either the topmost row or leftmost column and going in the bottom-right direction until reaching the matrix's end. For example, the matrix diagonal starting from mat[2][0], where mat is a 6 x 3 matrix, includes cells mat[2][0], mat[3][1], and mat[4][2].
 * Given an m x n matrix mat of integers, sort each matrix diagonal in ascending order and return the resulting matrix.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/1482_example_1_2.png" style="width: 500px; height: 198px;" />
 * Input: mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]]
 * Output: [[1,1,1,1],[1,2,2,2],[1,2,3,3]]
 *
 * <strong class="example">Example 2:
 *
 * Input: mat = [[11,25,66,1,69,7],[23,55,17,45,15,52],[75,31,36,44,58,8],[22,27,33,25,68,4],[84,28,14,11,5,50]]
 * Output: [[5,17,4,1,52,7],[11,11,25,45,8,69],[14,23,25,44,58,15],[22,27,31,36,50,66],[84,28,75,33,55,68]]
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 100
 * 	1 <= mat[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-the-matrix-diagonally/
// discuss: https://leetcode.com/problems/sort-the-matrix-diagonally/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // for every outer zell -> sort its diagonal line
        let width: usize = mat[0].len();
        let height: usize = mat.len();
        let mut result = vec![vec![0; width]; height];
        let mut heap = BinaryHeap::new();

        for i in 0..width {
            for j in 0..height {
                let x = i + j;
                if x < width {
                    heap.push(Reverse(mat[j][x]));
                }
            }
            for j in 0..height {
                let x = i + j;
                if x < width {
                    result[j][x] = heap.pop().unwrap().0;
                }
            }
            heap.clear();
        }

        for i in 1..height {
            for j in 0..width {
                let x = i + j;
                if x < height {
                    heap.push(Reverse(mat[x][j]));
                }
            }
            for j in 0..width {
                let x = i + j;
                if x < height {
                    result[x][j] = heap.pop().unwrap().0;
                }
            }
            heap.clear();
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1329() {
        assert_eq!(
            Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]),
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
        );
        assert_eq!(
            Solution::diagonal_sort(vec![
                vec![11, 25, 66, 1, 69, 7],
                vec![23, 55, 17, 45, 15, 52],
                vec![75, 31, 36, 44, 58, 8],
                vec![22, 27, 33, 25, 68, 4],
                vec![84, 28, 14, 11, 5, 50]
            ]),
            vec![
                vec![5, 17, 4, 1, 52, 7],
                vec![11, 11, 25, 45, 8, 69],
                vec![14, 23, 25, 44, 58, 15],
                vec![22, 27, 31, 36, 50, 66],
                vec![84, 28, 75, 33, 55, 68]
            ]
        );
    }
}
