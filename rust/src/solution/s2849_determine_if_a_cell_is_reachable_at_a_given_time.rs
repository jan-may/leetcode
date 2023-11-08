use std::cmp::min;

/**
 * [2849] Determine if a Cell Is Reachable at a Given Time
 *
 * You are given four integers sx, sy, fx, fy, and a non-negative integer t.
 * In an infinite 2D grid, you start at the cell (sx, sy). Each second, you must move to any of its adjacent cells.
 * Return true if you can reach cell (fx, fy) after exactly t seconds, or false otherwise.
 * A cell's adjacent cells are the 8 cells around it that share at least one corner with it. You can visit the same cell several times.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/08/05/example2.svg" style="width: 443px; height: 243px;" />
 * Input: sx = 2, sy = 4, fx = 7, fy = 7, t = 6
 * Output: true
 * Explanation: Starting at cell (2, 4), we can reach cell (7, 7) in exactly 6 seconds by going through the cells depicted in the picture above.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/08/05/example1.svg" style="width: 383px; height: 202px;" />
 * Input: sx = 3, sy = 1, fx = 7, fy = 3, t = 3
 * Output: false
 * Explanation: Starting at cell (3, 1), it takes at least 4 seconds to reach cell (7, 3) by going through the cells depicted in the picture above. Hence, we cannot reach cell (7, 3) at the third second.
 *
 *  
 * Constraints:
 *
 * 	1 <= sx, sy, fx, fy <= 10^9
 * 	0 <= t <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/
// discuss: https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        if sx == fx && sy == fy && t == 1 {
            return false;
        }
        let mut t = t;
        let mut steps_x = (sx - fx).abs();
        let mut steps_y = (sy - fy).abs();
        let steps_diagonal = std::cmp::min(steps_x, steps_y);
        t -= steps_diagonal;
        let steps_left = steps_x - steps_diagonal + steps_y - steps_diagonal;
        return match t < 0 {
            true => false,
            false => steps_left <= t,
        };
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2849() {
        assert_eq!(Solution::is_reachable_at_time(2, 4, 7, 7, 6), true);
        assert_eq!(Solution::is_reachable_at_time(3, 1, 7, 3, 3), false);
        assert_eq!(Solution::is_reachable_at_time(1, 1, 1, 2, 0), false);
        assert_eq!(Solution::is_reachable_at_time(1, 2, 1, 2, 1), false);
        assert_eq!(Solution::is_reachable_at_time(1, 1, 2, 3, 2), true);
        assert_eq!(Solution::is_reachable_at_time(1, 3, 2, 3, 0), true);
    }
}
