/**
 * [645] Set Mismatch
 *
 * You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.
 * You are given an integer array nums representing the data status of this set after the error.
 * Find the number that occurs twice and the number that is missing and return them in the form of an array.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,2,2,4]
 * Output: [2,3]
 * <strong class="example">Example 2:
 * Input: nums = [1,1]
 * Output: [1,2]
 *  
 * Constraints:
 * 
 * 	2 <= nums.length <= 10^4
 * 	1 <= nums[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/set-mismatch/
// discuss: https://leetcode.com/problems/set-mismatch/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut dup = 0;
        let mut miss = 0;
        for i in 0..nums.len() {
            let index = nums[i].abs() as usize - 1;
            if nums[index] < 0 {
                dup = index as i32 + 1;
            } else {
                nums[index] = -nums[index];
            }
        }
        for i in 0..nums.len() {
            if nums[i] > 0 {
                miss = i as i32 + 1;
            }
        }
        vec![dup, miss]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_645() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
