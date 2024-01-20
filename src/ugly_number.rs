// https://leetcode.com/problems/ugly-number

pub fn is_ugly(mut n: i32) -> bool {
    if n == 0 {
        return false;
    }
    for i in [2, 3, 5] {
        while n % i == 0 {
            n /= i;
        }
    }
    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_ugly(6), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_ugly(1), true);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_ugly(14), false);
    }
}
