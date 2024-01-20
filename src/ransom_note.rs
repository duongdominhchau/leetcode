// https://leetcode.com/problems/ransom-note/

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;

    let mut letters = HashMap::new();
    for ch in magazine.chars() {
        let entry = letters.entry(ch).or_insert(0);
        *entry += 1;
    }

    for ch in ransom_note.chars() {
        let entry = letters.entry(ch).or_insert(0);
        if *entry == 0 {
            return false;
        }
        *entry -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: bool) {
        assert_eq!(can_construct(a.to_string(), b.to_string()), expected);
    }

    #[test]
    fn test_1() {
        check("a", "b", false);
    }
    #[test]
    fn test_2() {
        check("aa", "ab", false);
    }
    #[test]
    fn test_3() {
        check("aa", "aab", true);
    }
}
