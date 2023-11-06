/**
 * [22] Generate Parentheses
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *  
 * <strong class="example">Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 * <strong class="example">Example 2:
 * Input: n = 1
 * Output: ["()"]
 *  
 * Constraints:
 *
 * 	1 <= n <= 8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/generate-parentheses/
// discuss: https://leetcode.com/problems/generate-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Solution::backtracking(n, n, String::new(), &mut result);
        result
    }

    fn backtracking(left: i32, right: i32, mut path: String, result: &mut Vec<String>) {
        if left == 0 && right == 0 {
            result.push(path);
            return;
        }
        if left > 0 {
            Solution::backtracking(left - 1, right, path.clone() + "(", result);
        }
        if right > left {
            Solution::backtracking(left, right - 1, path + ")", result);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}
