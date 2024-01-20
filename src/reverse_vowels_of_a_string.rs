// https://leetcode.com/problems/reverse-vowels-of-a-string

fn is_vowel(ch: u8) -> bool {
    "aeiouAEIOU".contains(ch as char)
}
pub fn reverse_vowels(s: String) -> String {
    let mut s = s.as_bytes().to_vec();

    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        if !is_vowel(s[left]) {
            left += 1;
            continue;
        }
        if !is_vowel(s[right]) {
            right -= 1;
            continue;
        }
        s.swap(left, right);
        left += 1;
        right -= 1;
    }

    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: &str) {
        assert_eq!(reverse_vowels(input.to_string()), expected.to_string());
    }

    #[test]
    fn test_1() {
        check("hello", "holle");
    }
    #[test]
    fn test_2() {
        check("leetcode", "leotcede");
    }
    #[test]
    fn test_3() {
        check("aA", "Aa");
    }
}
