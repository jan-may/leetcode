/**
 * [36] Valid Sudoku
 *
 * Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
 * <ol>
 * 	Each row must contain the digits 1-9 without repetition.
 * 	Each column must contain the digits 1-9 without repetition.
 * 	Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
 * </ol>
 * Note:
 *
 * 	A Sudoku board (partially filled) could be valid but is not necessarily solvable.
 * 	Only the filled cells need to be validated according to the mentioned rules.
 *
 *  
 * <strong class="example">Example 1:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px" />
 * Input: board =
 * [["5","3",".",".","7",".",".",".","."]
 * ,["6",".",".","1","9","5",".",".","."]
 * ,[".","9","8",".",".",".",".","6","."]
 * ,["8",".",".",".","6",".",".",".","3"]
 * ,["4",".",".","8",".","3",".",".","1"]
 * ,["7",".",".",".","2",".",".",".","6"]
 * ,[".","6",".",".",".",".","2","8","."]
 * ,[".",".",".","4","1","9",".",".","5"]
 * ,[".",".",".",".","8",".",".","7","9"]]
 * Output: true
 *
 * <strong class="example">Example 2:
 *
 * Input: board =
 * [["8","3",".",".","7",".",".",".","."]
 * ,["6",".",".","1","9","5",".",".","."]
 * ,[".","9","8",".",".",".",".","6","."]
 * ,["8",".",".",".","6",".",".",".","3"]
 * ,["4",".",".","8",".","3",".",".","1"]
 * ,["7",".",".",".","2",".",".",".","6"]
 * ,[".","6",".",".",".",".","2","8","."]
 * ,[".",".",".","4","1","9",".",".","5"]
 * ,[".",".",".",".","8",".",".","7","9"]]
 * Output: false
 * Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
 *
 *  
 * Constraints:
 *
 * 	board.length == 9
 * 	board[i].length == 9
 * 	board[i][j] is a digit 1-9 or '.'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-sudoku/
// discuss: https://leetcode.com/problems/valid-sudoku/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            if !Solution::is_row_valid(board.clone(), i) {
                return false;
            }
            if !Solution::is_col_valid(board.clone(), i) {
                return false;
            }
            if !Solution::is_box_valid(board.clone(), i / 3, i % 3) {
                return false;
            }
        }
        true
    }

    pub fn is_row_valid(board: Vec<Vec<char>>, row_index: i32) -> bool {
        let mut row = board[row_index as usize].clone();
        row.sort();
        for i in 0..8 {
            if row[i] != '.' && row[i] == row[i + 1] {
                return false;
            }
        }
        true
    }

    pub fn is_col_valid(board: Vec<Vec<char>>, col_index: i32) -> bool {
        let mut col = vec![];
        for i in 0..9 {
            col.push(board[i as usize][col_index as usize]);
        }
        col.sort();
        for i in 0..8 {
            if col[i] != '.' && col[i] == col[i + 1] {
                return false;
            }
        }
        true
    }

    pub fn is_box_valid(board: Vec<Vec<char>>, row_index: i32, col_index: i32) -> bool {
        let mut boxs = vec![];
        for i in 0..3 {
            for j in 0..3 {
                boxs.push(board[(row_index * 3 + i) as usize][(col_index * 3 + j) as usize]);
            }
        }
        boxs.sort();
        for i in 0..8 {
            if boxs[i] != '.' && boxs[i] == boxs[i + 1] {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_36() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }
}
