fn sum_from_1(n: i32) -> i32 {
    n * (n + 1) / 2
}
pub fn pivot_integer(n: i32) -> i32 {
    for i in 1..=n {
        let left_sum = sum_from_1(i);
        let right_sum = sum_from_1(n) - left_sum + i;
        if left_sum == right_sum {
            return i;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(pivot_integer(8), 6);
    }
    #[test]
    fn test_2() {
        assert_eq!(pivot_integer(1), 1);
    }
    #[test]
    fn test_3() {
        assert_eq!(pivot_integer(4), -1);
    }
}
