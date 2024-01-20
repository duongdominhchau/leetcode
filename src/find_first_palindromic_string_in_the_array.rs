// https://leetcode.com/problems/find-first-palindromic-string-in-the-array

fn is_palindrome(s: &str) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    for i in 0..=n / 2 {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }
    true
}
pub fn first_palindrome(words: Vec<String>) -> String {
    for word in words {
        if is_palindrome(&word) {
            return word;
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: Vec<&str>, expected: &str) {
        let input = input.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(first_palindrome(input), expected.to_string());
    }

    #[test]
    fn test_1() {
        check(vec!["abc", "car", "ada", "racecar", "cool"], "ada");
    }
    #[test]
    fn test_2() {
        check(vec!["notapalindrome", "racecar"], "racecar");
    }
    #[test]
    fn test_3() {
        check(vec!["def", "ghi"], "");
    }
}
