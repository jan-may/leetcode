use std::collections::{HashMap, HashSet};

struct ValidWordAbbr {
    map: HashMap<String, HashSet<String>>,
}

impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut map = HashMap::new();
        for word in dictionary {
            let abbr = Self::get_abbr(&word);
            map.entry(abbr).or_insert(HashSet::new()).insert(word);
        }
        Self { map }
    }

    fn is_unique(&self, word: String) -> bool {
        let abbr = Self::get_abbr(&word);
        match self.map.get(&abbr) {
            Some(set) => {
                if set.len() == 1 && set.contains(&word) {
                    true
                } else {
                    false
                }
            }
            None => true,
        }
    }

    fn get_abbr(word: &str) -> String {
        let mut abbr = String::new();
        let chars: Vec<char> = word.chars().collect();
        let len = chars.len();
        if len <= 2 {
            return word.to_string();
        }
        abbr.push(chars[0]);
        abbr.push_str(&len.to_string());
        abbr.push(chars[len - 1]);
        abbr
    }
}
