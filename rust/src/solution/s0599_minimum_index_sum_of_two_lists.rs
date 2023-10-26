use std::collections::HashMap;

/**
 * [599] Minimum Index Sum of Two Lists
 *
 * Given two arrays of strings list1 and list2, find the common strings with the least index sum.
 * A common string is a string that appeared in both list1 and list2.
 * A common string with the least index sum is a common string such that if it appeared at list1[i] and list2[j] then i + j should be the minimum value among all the other common strings.
 * Return all the common strings with the least index sum. Return the answer in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"]
 * Output: ["Shogun"]
 * Explanation: The only common string is "Shogun".
 *
 * <strong class="example">Example 2:
 *
 * Input: list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["KFC","Shogun","Burger King"]
 * Output: ["Shogun"]
 * Explanation: The common string with the least index sum is "Shogun" with index sum = (0 + 1) = 1.
 *
 * <strong class="example">Example 3:
 *
 * Input: list1 = ["happy","sad","good"], list2 = ["sad","happy","good"]
 * Output: ["sad","happy"]
 * Explanation: There are three common strings:
 * "happy" with index sum = (0 + 1) = 1.
 * "sad" with index sum = (1 + 0) = 1.
 * "good" with index sum = (2 + 2) = 4.
 * The strings with the least index sum are "sad" and "happy".
 *
 *  
 * Constraints:
 *
 * 	1 <= list1.length, list2.length <= 1000
 * 	1 <= list1[i].length, list2[i].length <= 30
 * 	list1[i] and list2[i] consist of spaces ' ' and English letters.
 * 	All the strings of list1 are unique.
 * 	All the strings of list2 are unique.
 * 	There is at least a common string between list1 and list2.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-index-sum-of-two-lists/
// discuss: https://leetcode.com/problems/minimum-index-sum-of-two-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut hashmap = HashMap::new();
        for (i, item) in list1.iter().enumerate() {
            hashmap.insert(item, i);
        }
        let mut result = vec![];
        let mut min_sum = std::usize::MAX;
        for (i, item) in list2.iter().enumerate() {
            if let Some(j) = hashmap.get(item) {
                if i + j < min_sum {
                    min_sum = i + j;
                    result.clear();
                    result.push(item.clone());
                } else if i + j == min_sum {
                    result.push(item.clone());
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_599() {
        assert_eq!(
            Solution::find_restaurant(
                vec_string!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                vec_string![
                    "Piatti",
                    "The Grill at Torrey Pines",
                    "Hungry Hunter Steakhouse",
                    "Shogun"
                ]
            ),
            vec_string!["Shogun"]
        );
        assert_eq!(
            Solution::find_restaurant(
                vec_string!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                vec_string!["KFC", "Shogun", "Burger King"]
            ),
            vec_string!["Shogun"]
        );
        assert_eq!(
            Solution::find_restaurant(
                vec_string!["happy", "sad", "good"],
                vec_string!["sad", "happy", "good"]
            ),
            vec_string!["sad", "happy"]
        );
    }
}
