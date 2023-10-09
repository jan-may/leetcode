/**
 * [1773] Count Items Matching a Rule
 *
 * You are given an array items, where each items[i] = [typei, colori, namei] describes the type, color, and name of the i^th item. You are also given a rule represented by two strings, ruleKey and ruleValue.
 * The i^th item is said to match the rule if one of the following is true:
 * 
 * 	ruleKey == "type" and ruleValue == typei.
 * 	ruleKey == "color" and ruleValue == colori.
 * 	ruleKey == "name" and ruleValue == namei.
 * 
 * Return the number of items that match the given rule.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: items = [["phone","blue","pixel"],["computer","silver","lenovo"],["phone","gold","iphone"]], ruleKey = "color", ruleValue = "silver"
 * Output: 1
 * Explanation: There is only one item matching the given rule, which is ["computer","silver","lenovo"].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: items = [["phone","blue","pixel"],["computer","silver","phone"],["phone","gold","iphone"]], ruleKey = "type", ruleValue = "phone"
 * Output: 2
 * Explanation: There are only two items matching the given rule, which are ["phone","blue","pixel"] and ["phone","gold","iphone"]. Note that the item ["computer","silver","phone"] does not match.
 *  
 * Constraints:
 * 
 * 	1 <= items.length <= 10^4
 * 	1 <= typei.length, colori.length, namei.length, ruleValue.length <= 10
 * 	ruleKey is equal to either "type", "color", or "name".
 * 	All strings consist only of lowercase letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-items-matching-a-rule/
// discuss: https://leetcode.com/problems/count-items-matching-a-rule/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut result = 0;
        let rule_key = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => panic!("invalid rule key"),
        };
        for item in items {
            if item[rule_key] == rule_value {
                result += 1;
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
    fn test_1773() {
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec!["phone".to_string(), "gold".to_string(), "iphone".to_string()]
                ],
                "color".to_string(),
                "silver".to_string()
            ),
            1
        );
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "phone".to_string()
                    ],
                    vec!["phone".to_string(), "gold".to_string(), "iphone".to_string()]
                ],
                "type".to_string(),
                "phone".to_string()
            ),
            2
        );
    }
}
