// https://leetcode.com/problems/isomorphic-strings

fn build_pattern(s: &str) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mapping = HashMap::new();
    let mut result = Vec::new();
    let mut next = 0;
    for ch in s.chars() {
        let mapped_num = mapping.entry(ch).or_insert_with(|| {
            let t = next;
            next += 1;
            t
        });
        result.push(*mapped_num);
    }
    result
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    build_pattern(&s) == build_pattern(&t)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: bool) {
        assert_eq!(is_isomorphic(a.to_string(), b.to_string()), expected);
    }

    #[test]
    fn test_1() {
        check("egg", "add", true);
    }
    #[test]
    fn test_2() {
        check("foo", "bar", false);
    }
    #[test]
    fn test_3() {
        check("paper", "title", true);
    }
}
