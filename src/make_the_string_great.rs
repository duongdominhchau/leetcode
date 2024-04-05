// https://leetcode.com/problems/make-the-string-great/

pub fn make_good(s: String) -> String {
    const CASE_DISTANCE: u8 = 'a' as u8 - 'A' as u8;

    let mut stack: Vec<u8> = Vec::new();
    for ch in s.chars() {
        if stack.is_empty() {
            stack.push(ch as u8);
            continue;
        }
        let last = stack.last().unwrap();
        if ch.is_alphabetic() && last.abs_diff(ch as u8) == CASE_DISTANCE {
            stack.pop();
        } else {
            stack.push(ch as u8);
        }
    }
    stack.into_iter().map(|c| c as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(make_good("leEeetcode".to_string()), "leetcode".to_string());
    }
}
