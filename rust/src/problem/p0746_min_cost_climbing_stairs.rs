use std::cmp::min;
use std::collections::HashMap;

/**
 * [746] Min Cost Climbing Stairs
 *
 * You are given an integer array cost where cost[i] is the cost of i^th step on a staircase. Once you pay the cost, you can either climb one or two steps.
 * You can either start from the step with index 0, or the step with index 1.
 * Return the minimum cost to reach the top of the floor.
 *  
 * <strong class="example">Example 1:
 *
 * Input: cost = [10,<u>15</u>,20]
 * Output: 15
 * Explanation: You will start at index 1.
 * - Pay 15 and climb two steps to reach the top.
 * The total cost is 15.
 *
 * <strong class="example">Example 2:
 *
 * Input: cost = [<u>1</u>,100,<u>1</u>,1,<u>1</u>,100,<u>1</u>,<u>1</u>,100,<u>1</u>]
 * Output: 6
 * Explanation: You will start at index 0.
 * - Pay 1 and climb two steps to reach index 2.
 * - Pay 1 and climb two steps to reach index 4.
 * - Pay 1 and climb two steps to reach index 6.
 * - Pay 1 and climb one step to reach index 7.
 * - Pay 1 and climb two steps to reach index 9.
 * - Pay 1 and climb one step to reach the top.
 * The total cost is 6.
 *
 *  
 * Constraints:
 *
 * 	2 <= cost.length <= 1000
 * 	0 <= cost[i] <= 999
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/min-cost-climbing-stairs/
// discuss: https://leetcode.com/problems/min-cost-climbing-stairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        let len = (cost.len()) as i32;
        Self::dp(len, &cost, &mut memo)
    }

    fn dp(i: i32, cost: &Vec<i32>, memo: &mut HashMap<i32, i32>) -> i32 {
        if i <= 1 {
            return 0;
        }
        if let Some(&v) = memo.get(&i) {
            return v;
        }
        let result = min(
            Self::dp(i - 1, cost, memo) + cost[(i - 1) as usize],
            Self::dp(i - 2, cost, memo) + cost[(i - 2) as usize],
        );
        memo.insert(i, result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_746() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
