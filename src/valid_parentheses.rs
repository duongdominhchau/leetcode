pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            close => match stack.pop() {
                Some(c) if c != close => {
                    return false;
                }
                None => return false,
                _ => continue,
            },
        }
    }
    stack.is_empty()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(is_valid(")(".to_string()), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(is_valid("(".to_string()), false);
    }
}
