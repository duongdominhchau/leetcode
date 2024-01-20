// https://leetcode.com/problems/roman-to-integer

pub fn roman_to_int(s: String) -> i32 {
    let units = [
        // Note: Order is important
        ("IV", 4),
        ("IX", 9),
        ("XL", 40),
        ("XC", 90),
        ("CD", 400),
        ("CM", 900),
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000),
    ];
    let mut value = 0;
    let mut s = &s[..];
    while !s.is_empty() {
        for (roman, arabic) in units {
            if s.starts_with(roman) {
                value += arabic;
                s = &s[roman.len()..];
                break;
            }
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }
    #[test]
    fn test_3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
