/**
 * [2418] Sort the People
 *
 * You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.
 * For each index i, names[i] and heights[i] denote the name and height of the i^th person.
 * Return names sorted in descending order by the people's heights.
 *  
 * <strong class="example">Example 1:
 *
 * Input: names = ["Mary","John","Emma"], heights = [180,165,170]
 * Output: ["Mary","Emma","John"]
 * Explanation: Mary is the tallest, followed by Emma and John.
 *
 * <strong class="example">Example 2:
 *
 * Input: names = ["Alice","Bob","Bob"], heights = [155,185,150]
 * Output: ["Bob","Alice","Bob"]
 * Explanation: The first Bob is the tallest, followed by Alice and the second Bob.
 *
 *  
 * Constraints:
 *
 * 	n == names.length == heights.length
 * 	1 <= n <= 10^3
 * 	1 <= names[i].length <= 20
 * 	1 <= heights[i] <= 10^5
 * 	names[i] consists of lower and upper case English letters.
 * 	All the values of heights are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-the-people/
// discuss: https://leetcode.com/problems/sort-the-people/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut result = names.clone();
        let mut height_map = std::collections::HashMap::new();
        for i in 0..heights.len() {
            height_map.insert(heights[i], &names[i]);
        }
        let mut heights = heights.clone();
        heights.sort();
        heights.reverse();
        for i in 0..heights.len() {
            result[i] = height_map.get(&heights[i]).unwrap().to_string();
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2418() {
        assert_eq!(
            Solution::sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            ),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
    }
}
