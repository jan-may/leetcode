struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones_split(nums: Vec<i32>) -> i32 {
        // split integer array for every 0
        let mut nums = nums;
        let mut nums: Vec<_> = nums.split(|&x| x == 0).collect();
        let mut result = 0;
        if nums.len() == 1 {
            return nums[0].len() as i32;
        }
        for i in 0..nums.len() - 1 {
            result = result.max(nums[i].len() + nums[i + 1].len() + 1);
        }
        result as i32
    }

    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        // two pointers
        let mut result = 0;
        let mut prev = 0;
        let mut curr = 0;
        for num in nums {
            match num {
                0 => {
                    prev = curr + 1;
                    curr = 0;
                }
                _ => curr += 1,
            }
            result = result.max(prev + curr);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0487_example_1() {
        let nums = vec![1, 0, 1, 1, 0];
        let result = 4;

        assert_eq!(Solution::find_max_consecutive_ones(nums), result);
    }

    #[test]
    fn test_0487_example_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let result = 4;

        assert_eq!(Solution::find_max_consecutive_ones(nums), result);
    }
}
