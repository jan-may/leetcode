/**
 * [38] Count and Say
 *
 * The count-and-say sequence is a sequence of digit strings defined by the recursive formula:
 *
 * 	countAndSay(1) = "1"
 * 	countAndSay(n) is the way you would "say" the digit string from countAndSay(n-1), which is then converted into a different digit string.
 *
 * To determine how you "say" a digit string, split it into the minimal number of substrings such that each substring contains exactly one unique digit. Then for each substring, say the number of digits, then say the digit. Finally, concatenate every said digit.
 * For example, the saying and conversion for digit string "3322251":
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/23/countandsay.jpg" style="width: 581px; height: 172px;" />
 * Given a positive integer n, return the n^th term of the count-and-say sequence.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 1
 * Output: "1"
 * Explanation: This is the base case.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 4
 * Output: "1211"
 * Explanation:
 * countAndSay(1) = "1"
 * countAndSay(2) = say "1" = one 1 = "11"
 * countAndSay(3) = say "11" = two 1's = "21"
 * countAndSay(4) = say "21" = one 2 + one 1 = "12" + "11" = "1211"
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-and-say/
// discuss: https://leetcode.com/problems/count-and-say/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut input = "1".to_string();
        if n == 1 {
            return input;
        }
        let mut input = input;
        for _ in 1..n {
            let mut output = String::new();
            let mut count = 1;
            let mut last = input.remove(0);
            for c in input.chars() {
                match c == last {
                    true => count += 1,
                    false => {
                        output.push_str(&format!("{}{}", count, last));
                        count = 1;
                        last = c;
                    }
                }
            }
            output.push_str(&format!("{}{}", count, last));
            input = output.clone();
        }
        input
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
        assert_eq!(Solution::count_and_say(2), "11".to_string());
        assert_eq!(Solution::count_and_say(3), "21".to_string());
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
        assert_eq!(Solution::count_and_say(5), "111221".to_string());
    }
}
