/**
 * [1351] Count Negative Numbers in a Sorted Matrix
 *
 * Given a m x n matrix grid which is sorted in non-increasing order both row-wise and column-wise, return the number of negative numbers in grid.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
 * Output: 8
 * Explanation: There are 8 negatives number in the matrix.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: grid = [[3,2],[1,0]]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 100
 * 	-100 <= grid[i][j] <= 100
 * 
 *  
 * Follow up: Could you find an O(n + m) solution?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
// discuss: https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        for row in grid.iter() {
            count += cols - Self::binary_search(row);
        }

        count as i32
    }

    fn binary_search(row: &Vec<i32>) -> usize {
        let mut low = 0;
        let mut high = row.len();

        while low < high {
            let mid = low + (high - low) / 2;
            if row[mid] < 0 {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1351() {
    }
}
