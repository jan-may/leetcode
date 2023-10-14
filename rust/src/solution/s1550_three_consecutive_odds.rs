/**
 * [1550] Three Consecutive Odds
 *
 * Given an integer array arr, return true if there are three consecutive odd numbers in the array. Otherwise, return false.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [2,6,4,1]
 * Output: false
 * Explanation: There are no three consecutive odds.
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [1,2,34,3,4,5,7,23,12]
 * Output: true
 * Explanation: [5,7,23] are three consecutive odds.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 1000
 * 	1 <= arr[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/three-consecutive-odds/
// discuss: https://leetcode.com/problems/three-consecutive-odds/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut flag = 0;
        for i in 0..arr.len() {
            if arr[i] % 2 != 0 {
                flag += 1;
            } else {
                flag = 0;
            }
            if flag == 3 {
                return true;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1550() {
        assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false);
        assert_eq!(
            Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
            true
        );
    }
}
