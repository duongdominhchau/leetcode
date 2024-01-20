use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut counts = HashMap::<char, usize>::new();
    s.chars().for_each(|c| match counts.get_mut(&c) {
        Some(count) => {
            *count += 1;
        }
        None => {
            counts.insert(c, 1);
        }
    });
    let mut max_len = counts
        .iter()
        .map(|(_, count)| count - count % 2)
        .fold(0, |acc, count| acc + count);
    if max_len < s.len() {
        max_len += 1;
    }
    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
    }
    #[test]
    fn test_2() {
        assert_eq!(longest_palindrome("a".to_string()), 1);
    }
    #[test]
    fn test_3() {
        assert_eq!(longest_palindrome("aa".to_string()), 2);
    }
}
