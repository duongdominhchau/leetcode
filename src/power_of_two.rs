// https://leetcode.com/problems/power-of-two

pub fn is_power_of_two(n: i32) -> bool {
    // Power of 2 will have all zeroes after the first 1, so subtract 1 will
    // result in a new shorter bit string with all ones and thus there will be
    // no common 1 bit => n & n-1 == 0
    // For other numbers, we have 2 or more 1 bits (e.g: 101), subtracting 1
    // only affect the right-most 1 bit, the left 1 bits are still preserved after
    // the calculation so the & yields non-zero number
    n > 0 && (n & (n - 1)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_power_of_two(1), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_power_of_two(16), true);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_power_of_two(3), false);
    }
    #[test]
    fn test_4() {
        assert_eq!(is_power_of_two(0), false);
    }
}
