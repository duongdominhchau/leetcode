// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
use std::collections::HashMap;

fn char_freq(s: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for ch in s.chars() {
        let count = freq.entry(ch).or_insert(0);
        *count += 1;
    }
    freq
}
pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let chars_freq = char_freq(&chars);
    let mut count = 0;
    'outer: for word in words {
        let freq = char_freq(&word);
        for (ch, ch_freq) in freq {
            if ch_freq > *chars_freq.get(&ch).unwrap_or(&0) {
                continue 'outer;
            }
        }
        count += word.len();
    }
    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(words: Vec<&str>, chars: &str, expected: i32) {
        assert_eq!(
            count_characters(
                words.into_iter().map(|s| s.to_string()).collect(),
                chars.to_string()
            ),
            expected
        );
    }

    #[test]
    fn test_1() {
        check(vec!["cat", "bt", "hat", "tree"], "atach", 6);
    }
    #[test]
    fn test_2() {
        check(vec!["hello", "world", "leetcode"], "welldonehoneyr", 10);
    }
}
