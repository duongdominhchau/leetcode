// https://leetcode.com/problems/sort-characters-by-frequency/

pub fn frequency_sort(s: String) -> String {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    s.chars().for_each(|ch| {
        let count = freq.entry(ch).or_insert(0);
        *count += 1;
    });

    let mut result = String::new();
    result.reserve_exact(s.len());
    let mut vec: Vec<_> = freq.into_iter().collect();
    vec.sort_unstable_by(|a, b| a.1.cmp(&b.1).reverse().then_with(|| a.0.cmp(&b.0)));
    for (ch, count) in vec.into_iter() {
        for _ in 0..count {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(frequency_sort("tree".to_string()), "eert".to_string());
    }
    #[test]
    fn test_2() {
        assert_eq!(frequency_sort("cccaaa".to_string()), "aaaccc".to_string());
    }
    #[test]
    fn test_3() {
        assert_eq!(frequency_sort("Aabb".to_string()), "bbAa".to_string());
    }
}
