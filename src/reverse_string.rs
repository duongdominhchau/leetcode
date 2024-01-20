// https://leetcode.com/problems/reverse-string

pub fn reverse_string(s: &mut Vec<char>) {
    let n = s.len();
    for i in 0..n / 2 {
        s.swap(i, n - 1 - i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(mut s: Vec<char>, expected: Vec<char>) {
        reverse_string(&mut s);
        assert_eq!(s, expected);
    }

    #[test]
    fn test_1() {
        check(vec!['h', 'e', 'l', 'l', 'o'], vec!['o', 'l', 'l', 'e', 'h']);
    }
    #[test]
    fn test_2() {
        check(
            vec!['H', 'a', 'n', 'n', 'a', 'h'],
            vec!['h', 'a', 'n', 'n', 'a', 'H'],
        );
    }
}
