// https://leetcode.com/problems/word-pattern/

use std::collections::HashMap;
use std::hash::Hash;

fn str_pattern<T>(iter: impl Iterator<Item = T>) -> Vec<usize>
where
    T: Sized + Eq + PartialEq + Hash,
{
    let mut mapping: HashMap<T, usize> = HashMap::new();
    let mut result = Vec::new();
    for element in iter {
        let next = mapping.len();
        let entry = mapping.entry(element).or_insert(next);
        result.push(*entry);
    }
    result
}
pub fn word_pattern(pattern: String, s: String) -> bool {
    str_pattern(pattern.chars()) == str_pattern(s.split_ascii_whitespace().map(|s| s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: bool) {
        assert_eq!(word_pattern(a.to_string(), b.to_string()), expected);
    }

    #[test]
    fn test_1() {
        check("abba", "dog cat cat dog", true);
    }
    #[test]
    fn test_2() {
        check("abba", "dog cat cat fish", false);
    }
    #[test]
    fn test_3() {
        check("aaaa", "dog cat cat dog", false);
    }
}
