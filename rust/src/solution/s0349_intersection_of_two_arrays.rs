use std::collections::HashSet;

/**
 * [349] Intersection of Two Arrays
 *
 * Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [9,4]
 * Explanation: [4,9] is also accepted.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 1000
 * 	0 <= nums1[i], nums2[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/intersection-of-two-arrays/
// discuss: https://leetcode.com/problems/intersection-of-two-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set: HashSet<i32> = std::collections::HashSet::new();
        let mut result: HashSet<i32> = std::collections::HashSet::new();
        for num in nums1 {
            set.insert(num);
        }
        for num in nums2 {
            if set.contains(&num) {
                result.insert(num);
            }
        }
        result.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_349() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![9, 4]
        );
    }
}
