// https://leetcode.com/problems/happy-number/

pub fn is_happy(mut n: i32) -> bool {
    use std::collections::HashSet;

    let mut prev_sums = HashSet::new();
    while n != 1 {
        let mut sum = 0;
        let mut t = n;
        while t != 0 {
            let digit = t % 10;
            t /= 10;
            sum += digit * digit;
        }
        if !prev_sums.insert(sum) {
            return false;
        }
        n = sum;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_happy(19), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_happy(2), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_happy(7), true);
    }
    #[test]
    fn test_4() {
        assert_eq!(is_happy(4), false);
    }
}
