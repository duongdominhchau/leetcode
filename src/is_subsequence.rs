// https://leetcode.com/problems/is-subsequence/

pub fn is_subsequence(s: String, t: String) -> bool {
    let (s, t) = (s.as_bytes(), t.as_bytes());
    let mut s_i = 0;
    let mut t_i = 0;
    while s_i < s.len() && t_i < t.len() {
        if s[s_i] == t[t_i] {
            s_i += 1;
        }
        t_i += 1;
    }
    s_i == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: bool) {
        assert_eq!(is_subsequence(a.to_string(), b.to_string()), expected);
    }
    #[test]
    fn test_1() {
        check("abc", "ahbgdc", true);
    }
    #[test]
    fn test_2() {
        check("axc", "ahbgdc", false);
    }
}
