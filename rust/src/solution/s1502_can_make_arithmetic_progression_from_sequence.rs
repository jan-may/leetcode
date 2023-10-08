/**
 * [1502] Can Make Arithmetic Progression From Sequence
 *
 * A sequence of numbers is called an arithmetic progression if the difference between any two consecutive elements is the same.
 * Given an array of numbers arr, return true if the array can be rearranged to form an arithmetic progression. Otherwise, return false.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [3,5,1]
 * Output: true
 * Explanation: We can reorder the elements as [1,3,5] or [5,3,1] with differences 2 and -2 respectively, between each consecutive elements.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: arr = [1,2,4]
 * Output: false
 * Explanation: There is no way to reorder the elements to obtain an arithmetic progression.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= arr.length <= 1000
 * 	-10^6 <= arr[i] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
// discuss: https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr.clone();
        arr.sort();
        let offset = arr[1] - arr[0];
        for i in 1..arr.len() -1{
            if arr[i+1] - arr[i] != offset {
                return false
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1502() {
        assert_eq!(Solution::can_make_arithmetic_progression(vec![3,5,1]), true);
        assert_eq!(Solution::can_make_arithmetic_progression(vec![1,2,4]), false);
    }
}
