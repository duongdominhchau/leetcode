// https://leetcode.com/problems/count-elements-with-maximum-frequency/`

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    for num in nums {
        freq.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut freq_freq = HashMap::new();
    for (num, count) in freq.into_iter() {
        freq_freq.entry(count).and_modify(|x| *x += 1).or_insert(1);
    }

    freq_freq
        .into_iter()
        .max_by_key(|(count, count_count)| *count)
        .map(|(count, count_count)| count * count_count)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
    #[test]
    fn test_3() {
        assert_eq!(max_frequency_elements(vec![1]), 1);
    }
}
