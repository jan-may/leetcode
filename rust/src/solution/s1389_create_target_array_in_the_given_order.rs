/**
 * [1389] Create Target Array in the Given Order
 *
 * Given two arrays of integers nums and index. Your task is to create target array under the following rules:
 *
 * 	Initially target array is empty.
 * 	From left to right read nums[i] and index[i], insert at index index[i] the value nums[i] in target array.
 * 	Repeat the previous step until there are no elements to read in nums and index.
 *
 * Return the target array.
 * It is guaranteed that the insertion operations will be valid.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [0,1,2,3,4], index = [0,1,2,2,1]
 * Output: [0,4,1,3,2]
 * Explanation:
 * nums       index     target
 * 0            0        [0]
 * 1            1        [0,1]
 * 2            2        [0,1,2]
 * 3            2        [0,1,3,2]
 * 4            1        [0,4,1,3,2]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3,4,0], index = [0,1,2,3,0]
 * Output: [0,1,2,3,4]
 * Explanation:
 * nums       index     target
 * 1            0        [1]
 * 2            1        [1,2]
 * 3            2        [1,2,3]
 * 4            3        [1,2,3,4]
 * 0            0        [0,1,2,3,4]
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1], index = [0]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length, index.length <= 100
 * 	nums.length == index.length
 * 	0 <= nums[i] <= 100
 * 	0 <= index[i] <= i
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/create-target-array-in-the-given-order/
// discuss: https://leetcode.com/problems/create-target-array-in-the-given-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..nums.len() {
            result.insert(index[i] as usize, nums[i]);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1389() {
        assert_eq!(
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
        assert_eq!(
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(Solution::create_target_array(vec![1], vec![0]), vec![1]);
    }
}
