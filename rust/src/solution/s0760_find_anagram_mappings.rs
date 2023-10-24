use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hash_map: std::collections::HashMap<i32, Vec<usize>> = HashMap::new();
        let mut result = Vec::new();
        for (i, num) in nums2.iter().enumerate() {
            let mut vec = hash_map.get(num).unwrap_or(&vec![]).clone();
            vec.push(i);
            hash_map.insert(*num, vec);
        }
        for num in nums1 {
            let vec = hash_map.get(&num).unwrap();
            result.push(vec[vec.len() - 1] as i32);
        }
        result
    }
}
