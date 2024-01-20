// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends

pub fn minimum_length(s: String) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut left = 0;
    let mut right = n - 1;
    while left < right && s[left] == s[right] {
        let ch = s[left];
        while left <= right && s[left] == ch {
            left += 1;
        }
        while left <= right && s[right] == ch {
            right -= 1;
        }
    }
    if left > right {
        0
    } else {
        (1 + right - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, expected: i32) {
        assert_eq!(minimum_length(a.to_string()), expected);
    }

    #[test]
    fn test_1() {
        check("ca", 2);
    }
    #[test]
    fn test_2() {
        check("cabaabac", 0);
    }
    #[test]
    fn test_3() {
        check("aabccabba", 3);
    }
    #[test]
    fn test_4() {
        check(
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbabbbbbbbbbbbbbbbccbcbcbccbbabbb",
            1,
        );
    }
}
