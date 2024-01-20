// https://leetcode.com/problems/custom-sort-string/

pub fn custom_sort_string(order: String, s: String) -> String {
    use std::collections::HashMap;

    let mut chars = HashMap::new();
    for ch in s.chars() {
        chars.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut result = String::new();
    for ch in order.chars() {
        let count = *chars.get(&ch).unwrap_or(&0);
        for _ in 0..count {
            result.push(ch);
        }
        chars.remove(&ch);
    }
    for (ch, count) in chars {
        for _ in 0..count {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(order: &str, s: &str, expected: &str) {
        assert_eq!(
            custom_sort_string(order.to_string(), s.to_string()),
            expected.to_string()
        )
    }

    #[test]
    fn test_1() {
        check("cba", "abcd", "cbad");
    }
    #[test]
    fn test_2() {
        check("bcafg", "abcd", "bcad");
    }
}
