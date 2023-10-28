struct Solution;

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut nums = vec![];
        let mut num = num;
        while num > 0 {
            nums.push(num % 10);
            num /= 10;
        }
        nums.sort();
        let num1 = nums[0] * 10 + nums[2];
        let num2 = nums[1] * 10 + nums[3];
        num1 + num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::minimum_sum(2932), 52);
        assert_eq!(Solution::minimum_sum(4009), 13);
    }
}
