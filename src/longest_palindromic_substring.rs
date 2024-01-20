fn is_palindrome(s: &str) -> bool {
    let s = s.as_bytes();
    for i in 0..=(s.len() / 2) {
        if s[i] != s[s.len() - 1 - i] {
            return false;
        }
    }
    true
}
pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    let mut best = String::new();
    for i in 0..n {
        // Stop early when there is no string longer than current best
        if s[i..n].len() <= best.len() {
            break;
        }

        for j in (i..n).rev() {
            let substr = &s[i..=j];
            // Same here, stop early when string is getting shorter than current best
            if substr.len() <= best.len() {
                break;
            }
            if is_palindrome(substr) {
                // If it is not longer than current best, it won't reach here
                best = substr.to_string();
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(longest_palindrome("babad".to_string()), "bab");
    }
    #[test]
    fn test_2() {
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
    }
    #[test]
    fn test_3() {
        assert_eq!(longest_palindrome("xabbac".to_string()), "abba");
    }
    #[test]
    fn test_4() {
        assert_eq!(longest_palindrome("xabac".to_string()), "aba");
    }
    #[test]
    fn test_5() {
        assert_eq!(longest_palindrome("".to_string()), "");
    }
    #[test]
    fn test_6() {
        assert_eq!(longest_palindrome("a".to_string()), "a");
    }
}
