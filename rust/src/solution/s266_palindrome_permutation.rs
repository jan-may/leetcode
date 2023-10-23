struct Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut odd_count = 0;
        for (_, v) in map {
            if v % 2 == 1 {
                odd_count += 1;
            }
        }
        odd_count <= 1
    }
}
