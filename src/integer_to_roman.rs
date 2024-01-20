// https://leetcode.com/problems/integer-to-roman

pub fn int_to_roman(mut num: i32) -> String {
    let digits = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];
    let mut result = String::new();
    for (roman, arabic) in digits {
        while num >= arabic {
            result.push_str(roman);
            num -= arabic;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(int_to_roman(3), "III".to_string());
    }
    #[test]
    fn test_2() {
        assert_eq!(int_to_roman(58), "LVIII".to_string());
    }
    #[test]
    fn test_3() {
        assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
    }
}
