//

pub fn find_the_difference(s: String, t: String) -> char {
    use std::collections::HashMap;

    let mut chars = HashMap::new();
    for ch in s.chars() {
        let count = chars.entry(ch).or_insert(0);
        *count += 1;
    }

    for ch in t.chars() {
        let count = chars.entry(ch).or_insert(0);
        if *count == 0 {
            return ch;
        }
        *count -= 1;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str, t: &str, expected: char) {
        assert_eq!(find_the_difference(s.to_string(), t.to_string()), expected);
    }

    #[test]
    fn test_1() {
        check("abcd", "abcde", 'e');
    }
    #[test]
    fn test_2() {
        check("", "y", 'y');
    }
}
