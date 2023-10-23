use std::collections::HashMap;

struct TwoSum {
    vec: Vec<i32>,
}

impl TwoSum {
    fn new() -> Self {
        TwoSum { vec: vec![] }
    }

    fn add(&mut self, number: i32) {
        self.vec.push(number);
    }

    fn find(&self, value: i32) -> bool {
        let mut seen = HashMap::new();
        for i in 0..self.vec.len() {
            let complement = value - self.vec[i];
            if seen.contains_key(&complement) {
                return true;
            }
            seen.insert(self.vec[i], i);
        }
        false
    }
}
