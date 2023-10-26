struct Solution;

impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        for s in strings {
            let key = Self::get_key(&s);
            map.entry(key).or_insert(vec![]).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }

    pub fn get_key(s: &str) -> String {
        let mut key = String::new();
        let mut chars: Vec<char> = s.chars().collect();
        let first = chars[0];
        for i in 0..chars.len() {
            let mut diff = chars[i] as i32 - first as i32;
            if diff < 0 {
                diff += 26;
            }
            key.push((diff + 'a' as i32) as u8 as char);
        }
        key
    }
}
