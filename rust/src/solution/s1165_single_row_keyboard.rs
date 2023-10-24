struct Solution;

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut result = 0;
        let mut prev = 0;
        let mut map = std::collections::HashMap::new();
        for (i, c) in keyboard.chars().enumerate() {
            map.insert(c, i);
        }
        for c in word.chars() {
            let curr = map.get(&c).unwrap();
            let diff = (curr - prev) as i32;
            result += diff.abs();
            prev = *curr;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1165_solution() {
        assert_eq!(
            Solution::calculate_time("abcdefghijklmnopqrstuvwxyz".to_string(), "cba".to_string()),
            4
        );
        assert_eq!(
            Solution::calculate_time(
                "pqrstuvwxyzabcdefghijklmno".to_string(),
                "leetcode".to_string()
            ),
            73
        );
    }
}
