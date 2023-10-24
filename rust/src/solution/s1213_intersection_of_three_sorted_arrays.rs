struct Solution;

impl Solution {
    pub fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
        let set1 = arr1.into_iter().collect::<std::collections::HashSet<i32>>();
        let set2 = arr2.into_iter().collect::<std::collections::HashSet<i32>>();
        let set3 = arr3.into_iter().collect::<std::collections::HashSet<i32>>();
        let mut result = Vec::new();
        for num in set1.intersection(&set2) {
            if set3.contains(num) {
                result.push(*num);
            }
        }
        result.sort();
        result
    }
}
