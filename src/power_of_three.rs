// https://leetcode.com/problems/power-of-three/

pub fn is_power_of_three(mut n: i32) -> bool {
    while n % 3 == 0 {
        n /= 3;
        if n == 0 {
            return false;
        }
    }
    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_power_of_three(27), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_power_of_three(0), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_power_of_three(-1), false);
    }
    #[test]
    fn test_4() {
        assert_eq!(is_power_of_three(9), true);
    }
}
