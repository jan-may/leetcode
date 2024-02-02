/**
 * [1619] Mean of Array After Removing Some Elements
 *
 * Given an integer array arr, return the mean of the remaining integers after removing the smallest 5% and the largest 5% of the elements.
 * Answers within 10^-5 of the actual answer will be considered accepted.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3]
 * Output: 2.00000
 * Explanation: After erasing the minimum and the maximum values of this array, all elements are equal to 2, so the mean is 2.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: arr = [6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]
 * Output: 4.00000
 * 
 * <strong class="example">Example 3:
 * 
 * Input: arr = [6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]
 * Output: 4.77778
 * 
 *  
 * Constraints:
 * 
 * 	20 <= arr.length <= 1000
 * 	arr.length is a multiple of 20.
 * 	<font face="monospace">0 <= arr[i] <= 10^5</font>
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/mean-of-array-after-removing-some-elements/
// discuss: https://leetcode.com/problems/mean-of-array-after-removing-some-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort_unstable();
        let n = arr.len();
        let start = n / 20;
        let end = n - start;
        let mut sum = 0;
        for i in start..end {
            sum += arr[i];
        }
        sum as f64 / (end - start) as f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1619() {
        assert_eq!(Solution::trim_mean(vec![1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3]), 2.0);
        assert_eq!(Solution::trim_mean(vec![6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]), 4.0);
    }
}
