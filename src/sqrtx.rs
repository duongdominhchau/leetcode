// https://leetcode.com/problems/sqrtx/

pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let x = x as i64;
    // Use binary search to find the value
    let mut left = 1;
    let mut right = x;
    while left <= right {
        let i = (left + right) / 2;
        if i * i > x {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }
    if left * left > x {
        left -= 1;
    }
    left as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(my_sqrt(4), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(my_sqrt(8), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(my_sqrt(1), 1);
    }
    #[test]
    fn test_4() {
        assert_eq!(my_sqrt(2), 1);
    }
    #[test]
    fn test_5() {
        assert_eq!(my_sqrt(3), 1);
    }
    #[test]
    fn test_6() {
        assert_eq!(my_sqrt(2147483647), 46340);
    }
    #[test]
    fn test_7() {
        assert_eq!(my_sqrt(0), 0);
    }
}
