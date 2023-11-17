/**
 * [1812] Determine Color of a Chessboard Square
 *
 * You are given coordinates, a string that represents the coordinates of a square of the chessboard. Below is a chessboard for your reference.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/screenshot-2021-02-20-at-22159-pm.png" style="width: 400px; height: 396px;" />
 * Return true if the square is white, and false if the square is black.
 * The coordinate will always represent a valid chessboard square. The coordinate will always have the letter first, and the number second.
 *  
 * <strong class="example">Example 1:
 *
 * Input: coordinates = "a1"
 * Output: false
 * Explanation: From the chessboard above, the square with coordinates "a1" is black, so return false.
 *
 * <strong class="example">Example 2:
 *
 * Input: coordinates = "h3"
 * Output: true
 * Explanation: From the chessboard above, the square with coordinates "h3" is white, so return true.
 *
 * <strong class="example">Example 3:
 *
 * Input: coordinates = "c7"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	coordinates.length == 2
 * 	'a' <= coordinates[0] <= 'h'
 * 	'1' <= coordinates[1] <= '8'
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/determine-color-of-a-chessboard-square/
// discuss: https://leetcode.com/problems/determine-color-of-a-chessboard-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let mut chars = coordinates.chars();
        let c = chars.next().unwrap();
        let r = chars.next().unwrap();
        (c as u8 - 'a' as u8 + r as u8 - '1' as u8) % 2 == 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1812() {
        assert_eq!(Solution::square_is_white("a1".to_string()), false);
        assert_eq!(Solution::square_is_white("h3".to_string()), true);
        assert_eq!(Solution::square_is_white("c7".to_string()), false);
    }
}
