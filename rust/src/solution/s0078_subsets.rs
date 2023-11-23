/**
 * [78] Subsets
 *
 * Given an integer array nums of unique elements, return all possible <span data-keyword="subset">subsets</span> (the power set).
 * The solution set must not contain duplicate subsets. Return the solution in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0]
 * Output: [[],[0]]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10
 * 	-10 <= nums[i] <= 10
 * 	All the numbers of nums are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subsets/
// discuss: https://leetcode.com/problems/subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..nums.len() {
            let mut tmp = vec![];
            for j in 0..result.len() {
                let mut t = result[j].clone();
                t.push(nums[i]);
                tmp.push(t);
            }
            result.append(&mut tmp);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_78() {}
}
