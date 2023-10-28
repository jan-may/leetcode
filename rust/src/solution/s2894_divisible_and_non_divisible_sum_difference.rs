struct Solution;

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let mut num1 = 0;
        let mut num2 = 0;
        for i in 1..=n {
            if i % m != 0 {
                num1 += i;
            }
        }
        for j in 1..=n {
            if j % m == 0 {
                num2 += j;
            }
        }
        num1 - num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::difference_of_sums(10, 3), 19);
        assert_eq!(Solution::difference_of_sums(5, 6), 15);
    }
}
