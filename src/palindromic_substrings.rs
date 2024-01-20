// https://leetcode.com/problems/palindromic-substrings

pub fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    // p[i][j] is true if s[i..=j] is palindrome
    let mut p = vec![vec![false; n]; n];
    // All single-character strings are palindrome
    for i in 0..n {
        p[i][i] = true;
    }
    for i in 0..n {
        for j in 0..i {
            // Need j == i - 1 to handle string of length 2 where i-1 < j+1
            // and thus represents an invalid slice (e.g: s[2..=1])
            p[i][j] = s[i] == s[j] && (j == i - 1 || p[i - 1][j + 1]);
        }
    }
    let mut count = 0;
    for i in 0..n {
        for j in 0..=i {
            if p[i][j] {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_substrings("abc".to_string()), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(count_substrings("aaa".to_string()), 6);
    }
}
