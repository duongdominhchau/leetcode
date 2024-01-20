pub fn my_atoi(s: String) -> i32 {
    let mut chars = s.chars().skip_while(|c| *c == ' ');

    let mut value_str = String::new();
    if let Some(&c) = chars.clone().peekable().peek() {
        match c {
            '-' => {
                value_str.push(c);
                chars.next();
            }
            '+' => {
                chars.next();
            }
            _ => {}
        }
    }

    value_str.push_str(&chars.take_while(|c| c.is_ascii_digit()).collect::<String>());
    match value_str.parse::<i32>() {
        Ok(value) => value,
        Err(e) => match e.kind() {
            std::num::IntErrorKind::PosOverflow => i32::MAX,
            std::num::IntErrorKind::NegOverflow => i32::MIN,
            _ => 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }
    #[test]
    fn test_2() {
        assert_eq!(my_atoi("    -42".to_string()), -42);
    }
    #[test]
    fn test_3() {
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    }
    #[test]
    fn test_4() {
        assert_eq!(my_atoi("words and 987".to_string()), 0);
    }
    #[test]
    fn test_5() {
        assert_eq!(my_atoi("20000000000000000000".to_string()), 2147483647);
    }
    #[test]
    fn test_6() {
        assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
    }
}
