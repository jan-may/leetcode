/**
 * [506] Relative Ranks
 *
 * You are given an integer array score of size n, where score[i] is the score of the i^th athlete in a competition. All the scores are guaranteed to be unique.
 * The athletes are placed based on their scores, where the 1^st place athlete has the highest score, the 2^nd place athlete has the 2^nd highest score, and so on. The placement of each athlete determines their rank:
 * 
 * 	The 1^st place athlete's rank is "Gold Medal".
 * 	The 2^nd place athlete's rank is "Silver Medal".
 * 	The 3^rd place athlete's rank is "Bronze Medal".
 * 	For the 4^th place to the n^th place athlete, their rank is their placement number (i.e., the x^th place athlete's rank is "x").
 * 
 * Return an array answer of size n where answer[i] is the rank of the i^th athlete.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: score = [5,4,3,2,1]
 * Output: ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
 * Explanation: The placements are [1^st, 2^nd, 3^rd, 4^th, 5^th].
 * <strong class="example">Example 2:
 * 
 * Input: score = [10,3,8,9,4]
 * Output: ["Gold Medal","5","Bronze Medal","Silver Medal","4"]
 * Explanation: The placements are [1^st, 5^th, 3^rd, 2^nd, 4^th].
 * 
 *  
 * Constraints:
 * 
 * 	n == score.length
 * 	1 <= n <= 10^4
 * 	0 <= score[i] <= 10^6
 * 	All the values in score are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/relative-ranks/
// discuss: https://leetcode.com/problems/relative-ranks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted_score = score.clone();
        sorted_score.sort_unstable_by(|a, b| b.cmp(a));
        let mut map = std::collections::HashMap::new();
        for (i, &s) in sorted_score.iter().enumerate() {
            map.insert(s, i);
        }
        let mut result = Vec::new();
        for &s in score.iter() {
            let rank = map.get(&s).unwrap() + 1;
            result.push(match rank {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                _ => rank.to_string(),
            });
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_506() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec![
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ]
        );
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec![
                "Gold Medal".to_string(),
                "5".to_string(),
                "Bronze Medal".to_string(),
                "Silver Medal".to_string(),
                "4".to_string()
            ]
        );
    }
}
