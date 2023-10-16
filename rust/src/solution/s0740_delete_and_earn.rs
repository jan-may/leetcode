use std::cmp::max;
use std::collections::HashMap;

/**
 * [740] Delete and Earn
 *
 * You are given an integer array nums. You want to maximize the number of points you get by performing the following operation any number of times:
 *
 * 	Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
 *
 * Return the maximum number of points you can earn by applying the above operation some number of times.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,4,2]
 * Output: 6
 * Explanation: You can perform the following operations:
 * - Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
 * - Delete 2 to earn 2 points. nums = [].
 * You earn a total of 6 points.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,2,3,3,3,4]
 * Output: 9
 * Explanation: You can perform the following operations:
 * - Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
 * - Delete a 3 again to earn 3 points. nums = [3].
 * - Delete a 3 once more to earn 3 points. nums = [].
 * You earn a total of 9 points.
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	1 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-and-earn/
// discuss: https://leetcode.com/problems/delete-and-earn/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut tab: HashMap<i32, i32> = HashMap::new();
        let mut memo: HashMap<i32, i32> = HashMap::new();
        let mut max_num = 0;
        for &num in nums.iter() {
            let count = tab.entry(num).or_insert(0);
            *count += 1;
            max_num = max(max_num, num);
        }
        Self::dp(max_num, &tab, &mut memo)
    }
    fn dp(num: i32, tab: &HashMap<i32, i32>, memo: &mut HashMap<i32, i32>) -> i32 {
        match num {
            0 => 0,
            1 => *tab.get(&1).unwrap_or(&0),
            _ => {
                if let Some(&v) = memo.get(&num) {
                    return v;
                }
                let gain = *tab.get(&num).unwrap_or(&0) * num;
                let result = max(
                    Self::dp(num - 1, tab, memo),
                    Self::dp(num - 2, tab, memo) + gain,
                );
                memo.insert(num, result);
                result
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_740() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
