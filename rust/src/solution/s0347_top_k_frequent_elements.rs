use std::collections::HashMap;

/**
 * [347] Top K Frequent Elements
 *
 * Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,1,1,2,2,3], k = 2
 * Output: [1,2]
 * <strong class="example">Example 2:
 * Input: nums = [1], k = 1
 * Output: [1]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 * 	k is in the range [1, the number of unique elements in the array].
 * 	It is guaranteed that the answer is unique.
 *
 *  
 * Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/top-k-frequent-elements/
// discuss: https://leetcode.com/problems/top-k-frequent-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut hashmap: HashMap<i32, i32> = std::collections::HashMap::new();
        for num in nums {
            let count = hashmap.entry(num).or_insert(0);
            *count += 1;
        }
        let mut v: Vec<(&i32, &i32)> = hashmap.iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1));
        for i in 0..k as usize {
            result.push(*v[i].0);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_347() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
