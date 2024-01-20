// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

fn helper(digits: &str, index: usize, current: &mut String, result: &mut Vec<String>) {
    if index == digits.len() {
        result.push(current.clone());
        return;
    }
    let digit = digits.as_bytes()[index] as char;
    let chars = match digit {
        '2' => "abc",
        '3' => "def",
        '4' => "ghi",
        '5' => "jkl",
        '6' => "mno",
        '7' => "pqrs",
        '8' => "tuv",
        '9' => "wxyz",
        _ => unreachable!(),
    };
    for ch in chars.chars() {
        current.push(ch);
        helper(digits, index + 1, current, result);
        current.pop();
    }
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::new();
    helper(&digits, 0, &mut String::new(), &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(digits: &str, expected: &[&str]) {
        let expected: Vec<String> = expected.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(letter_combinations(digits.to_string()), expected);
    }

    #[test]
    fn test_1() {
        check(
            "23",
            &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
        );
    }
    #[test]
    fn test_2() {
        check("", &[]);
    }
    #[test]
    fn test_3() {
        check("2", &["a", "b", "c"]);
    }
}
