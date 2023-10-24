struct SparseVector {
    nums: Vec<i32>,
}

impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    // Return the dotProduct of two sparse vectors
    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut result = 0;
        for (i, &num) in self.nums.iter().enumerate() {
            result += num * vec.nums[i];
        }
        result
    }
}
