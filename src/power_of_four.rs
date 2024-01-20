// https://leetcode.com/problems/power-of-four

pub fn is_power_of_four_naive(mut n: i32) -> bool {
    while n % 4 == 0 {
        n /= 4;
        if n == 0 {
            return false;
        }
    }
    n == 1
}

pub fn is_power_of_four(n: i32) -> bool {
    match n {
        0 => false,
        1 => true,
        _ => {
            // The number 4 in binary is 100, multiplying by 4 means shifting left twice. Therefore, any
            // power of four will have exactly 1 bit set followed by an even number of zeroes
            if n & (n - 1) != 0 {
                return false;
            }
            n.trailing_zeros() % 2 == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_power_of_four(16), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_power_of_four(5), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_power_of_four(1), true);
    }
    #[test]
    fn test_4() {
        assert_eq!(is_power_of_four(0), false);
    }
}
