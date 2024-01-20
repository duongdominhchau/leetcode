// https://leetcode.com/problems/maximum-odd-binary-number/

pub fn maximum_odd_binary_number(s: String) -> String {
    let count_one = s.bytes().filter(|&ch| ch == '1' as u8).count();
    let mut result = vec!['0' as u8; s.len()];
    result[s.len() - 1] = '1' as u8;
    for i in 1..count_one {
        result[i - 1] = '1' as u8;
    }
    unsafe { String::from_utf8_unchecked(result) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: &str) {
        assert_eq!(
            maximum_odd_binary_number(input.to_string()),
            expected.to_string()
        )
    }

    #[test]
    fn test_1() {
        check("010", "001");
    }
    #[test]
    fn test_2() {
        check("0101", "1001");
    }
}
