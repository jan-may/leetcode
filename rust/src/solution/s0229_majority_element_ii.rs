/**
 * [229] Majority Element II
 *
 * Given an integer array of size n, find all elements that appear more than &lfloor; n/3 &rfloor; times.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,2,3]
 * Output: [3]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1]
 * Output: [1]
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,2]
 * Output: [1,2]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^4
 * 	-10^9 <= nums[i] <= 10^9
 *
 *  
 * Follow up: Could you solve the problem in linear time and in O(1) space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element-ii/
// discuss: https://leetcode.com/problems/majority-element-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let size = n / 3;
        let map = nums
            .iter()
            .fold(std::collections::HashMap::new(), |mut map, &num| {
                *map.entry(num).or_insert(0) += 1;
                map
            });
        map.into_iter()
            .filter(|(_, v)| *v > size)
            .map(|(k, _)| k)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_229() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq!(Solution::majority_element(vec![1]), vec![1]);
        assert_eq!(Solution::majority_element(vec![1, 2]), vec![1, 2]);
    }
}
