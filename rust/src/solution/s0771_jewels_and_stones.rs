use std::collections::HashMap;

/**
 * [771] Jewels and Stones
 *
 * You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.
 * Letters are case sensitive, so "a" is considered a different type of stone from "A".
 *  
 * <strong class="example">Example 1:
 * Input: jewels = "aA", stones = "aAAbbbb"
 * Output: 3
 * <strong class="example">Example 2:
 * Input: jewels = "z", stones = "ZZ"
 * Output: 0
 *  
 * Constraints:
 *
 * 	1 <= jewels.length, stones.length <= 50
 * 	jewels and stones consist of only English letters.
 * 	All the characters of jewels are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jewels-and-stones/
// discuss: https://leetcode.com/problems/jewels-and-stones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut counter = 0;

        for c in stones.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in jewels.chars() {
            if map.contains_key(&c) {
                counter += map.get(&c).unwrap();
            }
        }
        counter
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_771() {}
}
