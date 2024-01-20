// https://leetcode.com/problems/climbing-stairs

pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return n;
    }
    // This is literally Fibonacci number with different start.
    //
    // Let h[i] be the number of ways we can reach height i. We can reach height i from height i-2
    // or i-1, so h[i] = h[i-2] + h[i-1]. The only difference is we start with 1 and 2
    let mut f1 = 1;
    let mut f2 = 2;
    let mut f = 0;
    for _ in 3..=n {
        f = f1 + f2;
        f1 = f2;
        f2 = f;
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(climb_stairs(2), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(climb_stairs(3), 3);
    }
}
