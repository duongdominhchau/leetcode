// https://leetcode.com/problems/minimum-window-substring
use std::collections::HashMap;

fn include_all(a: &HashMap<u8, i32>, b: &HashMap<u8, i32>) -> bool {
    for (k, v) in b.iter() {
        match a.get(k) {
            Some(count) => {
                if count < v {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}
pub fn min_window(s: String, t: String) -> String {
    if s.len() < t.len() {
        return String::new();
    }
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut t_chars = HashMap::new();
    t.iter().for_each(|b| {
        let count = t_chars.entry(*b).or_insert(0);
        *count += 1;
    });

    let mut s_chars = HashMap::new();
    let mut left = 0;
    let mut right = 0;
    let mut best_left = None;
    let mut best_right = None;
    while left <= right && right < s.len() {
        // Find a window containing string t
        while right < s.len() && !include_all(&s_chars, &t_chars) {
            let count = s_chars.entry(s[right]).or_insert(0);
            *count += 1;
            right += 1;
        }
        if right == s.len() && !include_all(&s_chars, &t_chars) {
            break;
        }
        // Shorten it as much as possible
        while left < right
            && s_chars.get(&s[left]).unwrap_or(&0) > t_chars.get(&s[left]).unwrap_or(&0)
        {
            let count = s_chars.get_mut(&s[left]).unwrap();
            *count -= 1;
            left += 1;
        }
        // Record new best
        if best_left.is_none() || right - left < best_right.unwrap() - best_left.unwrap() {
            best_left = Some(left);
            best_right = Some(right);
        }
        // Remove left-most character from the sliding window
        if let Some(count) = s_chars.get_mut(&s[left]) {
            *count -= 1;
        }
        left += 1;
    }
    match best_left {
        Some(_) => String::from_utf8(s[best_left.unwrap()..best_right.unwrap()].to_vec()).unwrap(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
    }
    #[test]
    fn test_4() {
        assert_eq!(
            min_window("ab".to_string(), "a".to_string()),
            "a".to_string()
        );
    }
    #[test]
    fn test_5() {
        assert_eq!(min_window("a".to_string(), "b".to_string()), "".to_string());
    }
}
