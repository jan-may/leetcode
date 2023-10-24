struct Solution;

impl Solution {
    pub fn min_product_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort_unstable();
        nums2.sort_unstable_by(|a, b| b.cmp(a));
        let mut result = 0;
        for i in 0..nums1.len() {
            result += nums1[i] * nums2[i];
        }
        result
    }
}
