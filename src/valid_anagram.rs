// https://leetcode.com/problems/valid-anagram/

pub fn is_anagram(s: String, t: String) -> bool {
    use std::collections::HashMap;
    let mut frequency = HashMap::new();
    for ch in s.chars() {
        let count = frequency.entry(ch).or_insert(0);
        *count += 1;
    }
    for ch in t.chars() {
        let count = frequency.entry(ch).or_insert(0);
        *count -= 1;
    }
    frequency.into_values().all(|val| val == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: bool) {
        assert_eq!(is_anagram(a.to_string(), b.to_string()), expected);
    }

    #[test]
    fn test_1() {
        check("anagram", "nagaram", true);
    }
    #[test]
    fn test_2() {
        check("rat", "car", false);
    }
    #[test]
    fn test_3() {
        check("cát", "tác", true);
    }
}
