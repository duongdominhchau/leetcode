// https://leetcode.com/problems/coin-change

pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
    coins.sort_unstable_by_key(|x| -x);
    let coins: Vec<usize> = coins
        .into_iter()
        .skip_while(|&coin| coin > amount)
        .map(|coin| coin as usize)
        .collect();
    let amount = amount as usize;

    // best[i] is the smallest number of coins needed to reach the amount i
    // -1 means the amount is impossible to create from the coin values provided
    //
    // best[0] = 0
    // best[i] = min(best[i-k] + 1) for k in coins
    let mut best = vec![-1; amount + 1];
    best[0] = 0;
    for &i in coins.iter() {
        best[i] = 1;
    }

    for i in 1..=amount {
        let count = coins
            .iter()
            .filter(|&coin| i >= *coin)
            .map(|&coin| best[i - coin])
            .filter(|&count| count != -1)
            .min()
            .unwrap_or(-1);
        if count != -1 {
            best[i] = count + 1;
        }
    }
    best[amount]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }
    #[test]
    fn test_3() {
        assert_eq!(coin_change(vec![1], 0), 0);
    }
    #[test]
    fn test_4() {
        assert_eq!(coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
    #[test]
    fn test_5() {
        assert_eq!(coin_change(vec![1, 2147483647], 2), 2);
    }
}
