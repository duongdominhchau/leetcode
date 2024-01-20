// https://leetcode.com/problems/best-time-to-buy-and-sell-stock

pub fn max_profit_dp(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    // min_price[i] = lowest stock price before day i
    let mut min_price = vec![0; n];
    min_price[0] = prices[0];
    for i in 1..n {
        min_price[i] = min_price[i - 1].min(prices[i]);
    }

    let mut best_profit = 0;
    for i in 0..n {
        best_profit = best_profit.max(prices[i] - min_price[i]);
    }
    best_profit
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut best_profit = 0;
    for i in 1..prices.len() {
        best_profit = best_profit.max(prices[i] - min_price);
        min_price = min_price.min(prices[i]);
    }
    best_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
