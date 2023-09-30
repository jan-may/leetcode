
pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut i = nums.len() as i32 - 1;
        let mut set = std::collections::HashSet::new();
        let mut target = (1..=k).collect::<std::collections::HashSet<_>>(); // Create a set of elements 1 to k.

        while !set.is_superset(&target) {
            count += 1;
            if !set.contains(&nums[i as usize]) {
                set.insert(nums[i as usize]);
            }
            i -= 1;
        }

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c114() {
        assert_eq!(Solution::min_operations(vec![3,1,5,4,2], 2), 4);
        assert_eq!(Solution::min_operations(vec![3,1,5,4,2], 5), 5);
    }
}
