// https://leetcode.com/problems/a-number-after-a-double-reversal

pub fn is_same_after_reversals(num: i32) -> bool {
    // We only lose digits if trailing zeroes become leading zeroes
    num == 0 || num % 10 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_same_after_reversals(526), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_same_after_reversals(1800), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_same_after_reversals(0), true);
    }
}
