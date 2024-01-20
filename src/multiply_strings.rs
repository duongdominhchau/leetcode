// https://leetcode.com/problems/multiply-strings/

fn add(a: &str, b: &str) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let (mut a_iter, mut b_iter) = (a.chars().rev(), b.chars().rev());
    loop {
        match (a_iter.next(), b_iter.next()) {
            (None, None) => {
                if carry != 0 {
                    result.push(char::from_digit(carry, 10).unwrap());
                }
                break;
            }
            (None, Some(d)) | (Some(d), None) => {
                carry += d.to_digit(10).unwrap();
                result.push(char::from_digit(carry % 10, 10).unwrap());
                carry /= 10;
            }
            (Some(d1), Some(d2)) => {
                carry += d1.to_digit(10).unwrap() + d2.to_digit(10).unwrap();
                result.push(char::from_digit(carry % 10, 10).unwrap());
                carry /= 10;
            }
        }
    }
    result.chars().rev().collect()
}
fn trim_zeros(s: &str) -> String {
    let s = s.trim_end_matches('0');
    if s.is_empty() { "0" } else { s }.to_string()
}
fn multiply_digit(a: &str, digit: u32) -> String {
    let mut result = String::new();
    let mut carry = 0;
    for d in a.chars().rev() {
        let prod = d.to_digit(10).unwrap() * digit + carry;
        result.push(char::from_digit(prod % 10, 10).unwrap());
        carry = prod / 10;
    }
    if carry != 0 {
        result.push(char::from_digit(carry, 10).unwrap());
    }
    trim_zeros(&result).chars().rev().collect()
}

pub fn multiply(num1: String, num2: String) -> String {
    let mut zeros = 0;
    let mut result = "0".to_string();
    for digit in num2.chars().rev() {
        let mut prod = multiply_digit(&num1, digit.to_digit(10).unwrap());
        if prod != "0" {
            for _ in 0..zeros {
                prod.push('0');
            }
        }
        result = add(&result, &prod);
        zeros += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: &str) {
        assert_eq!(multiply(a.to_string(), b.to_string()), expected.to_string());
    }

    #[test]
    fn test_1() {
        check("2", "3", "6");
    }
    #[test]
    fn test_2() {
        check("123", "456", "56088");
    }
    #[test]
    fn test_3() {
        check("9133", "0", "0");
    }
    #[test]
    fn test_4() {
        check("0", "52", "0");
    }
}
