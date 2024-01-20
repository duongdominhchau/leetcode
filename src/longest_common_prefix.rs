// https://leetcode.com/problems/longest-common-prefix/

use std::str::Chars;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    let mut iters: Vec<Chars<'_>> = strs.iter().map(|s| s.chars()).collect();
    loop {
        let chars: Vec<Option<char>> = iters.iter_mut().map(|i| i.next()).collect();
        let first_char = chars[0];
        if first_char.is_some() && chars.iter().all(|&x| x == first_char) {
            prefix.push(first_char.unwrap());
        } else {
            return prefix;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            longest_common_prefix(
                ["flower", "flow", "flight"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            "fl".to_string()
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            longest_common_prefix(["flower", "x"].iter().map(|s| s.to_string()).collect()),
            "".to_string()
        );
    }
}
