// https://leetcode.com/problems/perfect-squares/

pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let perfect_squares: Vec<usize> = (1..=n).take_while(|&i| i * i <= n).map(|i| i * i).collect();
    if perfect_squares.binary_search(&n).is_ok() {
        return 1;
    }

    let mut best = vec![i32::MAX; n + 1];
    best[0] = 0;
    for i in 1..=n {
        let prev_best = perfect_squares
            .iter()
            .take_while(|&square| *square <= i)
            .map(|&square| best[i - square])
            .min()
            .unwrap();
        best[i] = best[i].min(prev_best + 1);
    }
    best[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(num_squares(12), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(num_squares(13), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(num_squares(1), 1);
    }
}
