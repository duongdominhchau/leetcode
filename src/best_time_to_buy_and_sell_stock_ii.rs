// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii

pub fn max_profit(mut prices: Vec<i32>) -> i32 {
    let n = prices.len();

    // Calculate the diff (the profit)
    for i in (1..n).rev() {
        prices[i] -= prices[i - 1];
    }
    // Then try to take as many positive diff as possible.
    //
    // If we have a profit followed by another profit, we can merge both into one
    // for even greater profit. Example: prices = [1, 5, 7], we can buy at 1 and
    // sell at 7. If we sell the stock earlier, we need to buy it with a greater price
    // and thus reduce our profit. We can see here it is optimal to buy as early as possible
    // and sell it when the price reaches its local peak.
    //
    // Now consider a price drop then rise (e.g: [3, 1, 4]). Because the price drops,
    // buying early will cost us more. The best time to buy is after the price drop,
    // and once again the best time to sell is when it reaches a local peak.
    //
    // Because of these 2 points, the optimal solution will be the sum of all the positive
    // diff, as they are the date range when we buy at the local lowest price and sell at the
    // local highest price.

    prices[1..].iter().filter(|&&x| x > 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
    #[test]
    fn test_3() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
