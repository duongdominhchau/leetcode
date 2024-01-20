// https://leetcode.com/problems/first-unique-character-in-a-string

pub fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for (index, ch) in s.chars().enumerate() {
        let entry = freq.entry(ch).or_insert((0, index));
        entry.0 += 1;
    }
    freq.values()
        .filter(|(count, _index)| *count == 1)
        .map(|(_count, index)| *index as i32)
        .into_iter()
        .min()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first_uniq_char("leetcode".to_string()), 0);
    }
    #[test]
    fn test_2() {
        assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(first_uniq_char("aabb".to_string()), -1);
    }
}
