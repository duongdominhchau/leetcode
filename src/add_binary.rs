// https://leetcode.com/problems/add-binary

fn digit(val: u8) -> u8 {
    val + b'0'
}
fn digit_at(s: &[u8], index: i32) -> u8 {
    if 0 <= index && index < s.len() as i32 {
        s[index as usize] - b'0'
    } else {
        0
    }
}

pub fn add_binary(a: String, b: String) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let len_a = a.len();
    let len_b = b.len();
    let max_len = a.len().max(b.len());

    let mut result = Vec::new();
    result.reserve_exact(max_len + 1);
    let mut carry = 0;
    for i in 0..max_len {
        let i = i as i32;
        let bit_a = digit_at(a, len_a as i32 - 1 - i);
        let bit_b = digit_at(b, len_b as i32 - 1 - i);
        let sum = carry + bit_a + bit_b;
        carry = sum / 2;
        result.push(digit(sum % 2));
    }
    if carry > 0 {
        result.push(digit(carry));
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: &str) {
        assert_eq!(
            add_binary(a.to_string(), b.to_string()),
            expected.to_string()
        );
    }

    #[test]
    fn test_1() {
        check("11", "1", "100");
    }
    #[test]
    fn test_2() {
        check("1010", "1011", "10101");
    }
}
