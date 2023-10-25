/**
 * [1089] Duplicate Zeros
 *
 * Given a fixed-length integer array arr, duplicate each occurrence of zero, shifting the remaining elements to the right.
 * Note that elements beyond the length of the original array are not written. Do the above modifications to the input array in place and do not return anything.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [1,0,2,3,0,4,5,0]
 * Output: [1,0,0,2,3,0,0,4]
 * Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [1,2,3]
 * Output: [1,2,3]
 * Explanation: After calling your function, the input array is modified to: [1,2,3]
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^4
 * 	0 <= arr[i] <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/duplicate-zeros/
// discuss: https://leetcode.com/problems/duplicate-zeros/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                arr.insert(i, 0);
                arr.pop();
                i += 1;
            }
            i += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1089() {}
}
