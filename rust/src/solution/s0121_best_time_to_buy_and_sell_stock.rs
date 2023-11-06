/**
 * [121] Best Time to Buy and Sell Stock
 *
 * You are given an array prices where prices[i] is the price of a given stock on the i^th day.
 * You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
 * Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
 *  
 * <strong class="example">Example 1:
 *
 * Input: prices = [7,1,5,3,6,4]
 * Output: 5
 * Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
 * Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
 *
 * <strong class="example">Example 2:
 *
 * Input: prices = [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transactions are done and the max profit = 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= prices.length <= 10^5
 * 	0 <= prices[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left_ptr = 0;
        let mut right_ptr = prices.len() - 1;
        let mut min_price = std::i32::MAX;
        let mut max = 0;
        for _ in 0..prices.len() {
            if prices[left_ptr] < min_price {
                min_price = prices[left_ptr];
            }
            if prices[left_ptr] - min_price > max {
                max = prices[left_ptr] - min_price;
            }
            left_ptr += 1;
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
