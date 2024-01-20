// https://leetcode.com/problems/valid-palindrome/

pub fn is_palindrome(s: String) -> bool {
    // Idea for a better implementation, I won't do that as it's too trivial to write:
    // Use two pointers, one from the beginning and the other from the end, skip the invalid
    // characters from both side, then compare the character pairs until they intersect.
    let s: String = s
        .chars()
        .filter(|&ch| ch.is_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();
    let n = s.len();
    let s = s.as_bytes();
    for i in 0..n / 2 {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_palindrome(" ".to_string()), true);
    }
}
