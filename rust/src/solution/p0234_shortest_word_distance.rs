struct Solution;

impl Solution {
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let mut result = i32::MAX;
        let mut i1 = -1;
        let mut i2 = -1;
        for i in 0..words_dict.len() {
            if words_dict[i] == word1 {
                i1 = i as i32;
            } else if words_dict[i] == word2 {
                i2 = i as i32;
            }
            if i1 != -1 && i2 != -1 {
                result = result.min((i1 - i2).abs());
            }
        }
        result
    }
}
