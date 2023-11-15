/**
 * [1743] Restore the Array From Adjacent Pairs
 *
 * There is an integer array nums that consists of n unique elements, but you have forgotten it. However, you do remember every pair of adjacent elements in nums.
 * You are given a 2D integer array adjacentPairs of size n - 1 where each adjacentPairs[i] = [ui, vi] indicates that the elements ui and vi are adjacent in nums.
 * It is guaranteed that every adjacent pair of elements nums[i] and nums[i+1] will exist in adjacentPairs, either as [nums[i], nums[i+1]] or [nums[i+1], nums[i]]. The pairs can appear in any order.
 * Return the original array nums. If there are multiple solutions, return any of them.
 *  
 * <strong class="example">Example 1:
 *
 * Input: adjacentPairs = [[2,1],[3,4],[3,2]]
 * Output: [1,2,3,4]
 * Explanation: This array has all its adjacent pairs in adjacentPairs.
 * Notice that adjacentPairs[i] may not be in left-to-right order.
 *
 * <strong class="example">Example 2:
 *
 * Input: adjacentPairs = [[4,-2],[1,4],[-3,1]]
 * Output: [-2,4,1,-3]
 * Explanation: There can be negative numbers.
 * Another solution is [-3,1,4,-2], which would also be accepted.
 *
 * <strong class="example">Example 3:
 *
 * Input: adjacentPairs = [[100000,-100000]]
 * Output: [100000,-100000]
 *
 *  
 * Constraints:
 *
 * 	nums.length == n
 * 	adjacentPairs.length == n - 1
 * 	adjacentPairs[i].length == 2
 * 	2 <= n <= 10^5
 * 	-10^5 <= nums[i], ui, vi <= 10^5
 * 	There exists some nums that has adjacentPairs as its pairs.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/
// discuss: https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
        for pair in adjacent_pairs.iter() {
            map.entry(pair[0]).or_insert(vec![]).push(pair[1]);
            map.entry(pair[1]).or_insert(vec![]).push(pair[0]);
        }
        let mut result = vec![];
        for (k, v) in map.iter() {
            if v.len() == 1 {
                result.push(*k);
                result.push(v[0]);
                break;
            }
        }
        while result.len() < adjacent_pairs.len() + 1 {
            let last = result[result.len() - 1];
            let next = map.get(&last).unwrap();
            if result[result.len() - 2] == next[0] {
                result.push(next[1]);
            } else {
                result.push(next[0]);
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
    fn test_1743() {
        assert_eq!(
            Solution::restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]),
            vec![1, 2, 3, 4]
        );
    }
}
