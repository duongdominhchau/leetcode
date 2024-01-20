pub fn reverse(x: i32) -> i32 {
    let mut reversed: i32 = 0;
    let mut x = x;
    while x != 0 {
        reversed = match reversed.checked_mul(10) {
            Some(val) => val,
            None => return 0,
        };
        // x % 10 is negative when x is negative already, so we don't need to handle
        // that explicitly
        reversed = match reversed.checked_add(x % 10) {
            Some(val) => {
                println!("{val:?}");
                val
            }
            None => return 0,
        };
        x /= 10;
    }
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse(123), 321);
    }
    #[test]
    fn test_2() {
        assert_eq!(reverse(-123), -321);
    }
    #[test]
    fn test_3() {
        assert_eq!(reverse(120), 21);
    }
    #[test]
    fn test_4() {
        assert_eq!(reverse(987654321_i32), 123456789);
    }
    #[test]
    fn test_5() {
        assert_eq!(reverse(123456789), 987654321);
    }
    #[test]
    fn test_6() {
        assert_eq!(reverse(1234567899_i32), 0);
    }
}
