/**
 * [187] Repeated DNA Sequences
 *
 * The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
 *
 * 	For example, "ACGAATTCCG" is a DNA sequence.
 *
 * When studying DNA, it is useful to identify repeated sequences within the DNA.
 * Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
 * Output: ["AAAAACCCCC","CCCCCAAAAA"]
 * <strong class="example">Example 2:
 * Input: s = "AAAAAAAAAAAAA"
 * Output: ["AAAAAAAAAA"]
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either 'A', 'C', 'G', or 'T'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/repeated-dna-sequences/
// discuss: https://leetcode.com/problems/repeated-dna-sequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut map = HashMap::new();
        let mut i = 0;
        while i + 10 <= s.len() {
            let key = &s[i..i + 10];
            let count = map.entry(key).or_insert(0);
            *count += 1;
            if *count == 2 {
                result.push(key.to_string());
            }
            i += 1;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_187() {
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()),
            vec_string!["AAAAACCCCC", "CCCCCAAAAA"]
        );
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()),
            vec_string!["AAAAAAAAAA"]
        );
    }
}
