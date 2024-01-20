pub fn length_of_longest_substring(s: String) -> i32 {
    let mut best_len = 0;
    let mut last_index = 0;
    for i in 1..s.len() {
        let a = &s[last_index..i];
        let ch = s.as_bytes()[i] as char;
        if let Some(offset) = a.find(|c| c == ch) {
            best_len = best_len.max(a.len());
            // Must skip the old duplicate to have a valid string
            last_index += offset + 1;
        }
    }
    best_len = best_len.max(s.len() - last_index);
    best_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(length_of_longest_substring("aa".to_string()), 1);
    }
    #[test]
    fn test_2() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }
    #[test]
    fn test_3() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }
    #[test]
    fn test_4() {
        assert_eq!(length_of_longest_substring(" ".to_string()), 1);
    }
    #[test]
    fn test_5() {
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
    }
    #[test]
    fn test_6() {
        assert_eq!(length_of_longest_substring("aab".to_string()), 2);
    }
}
