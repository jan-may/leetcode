pub struct Solution {}

impl Solution {
    pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
        //...
        let mut stack = vec![];
        let mut res = vec![];
        for word in words {
            if word == "prev" {
                if let Some(num) = stack.pop() {
                    res.push(num);
                } else {
                    res.push(-1);
                }
            } else {
                let num = word.parse::<i32>().unwrap();
                stack.push(num);
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c115() {
        assert_eq!(
            Solution::last_visited_integers(vec![
                "1".to_string(),
                "2".to_string(),
                "prev".to_string(),
                "prev".to_string(),
                "prev".to_string()
            ]),
            vec![2, 1, -1]
        );
        assert_eq!(
            Solution::last_visited_integers(vec![
                "1".to_string(),
                "prev".to_string(),
                "2".to_string(),
                "prev".to_string(),
                "prev".to_string(),
            ]),
            vec![1, 2, 1]
        );
    }
}
