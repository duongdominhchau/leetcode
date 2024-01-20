// https://leetcode.com/problems/contiguous-array

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut sum = 0;
    // first[s] = the index where `sum` value is `s` for the first time
    let mut first = HashMap::new();
    // We can have a sum of 0 even before reading any element
    first.insert(0, -1);
    let mut max_len = 0;
    for i in 0..nums.len() {
        sum += if nums[i] == 0 { -1 } else { 1 };
        // If we encounter the same sum twice, the subarray between them
        // must have a sum of 0
        first
            .entry(sum)
            .and_modify(|index| max_len = max_len.max(i as i32 - *index))
            .or_insert(i as i32);
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_max_length(vec![0, 1]), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(find_max_length(vec![0, 1, 0]), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(find_max_length(vec![1]), 0);
    }
    #[test]
    fn test_4() {
        assert_eq!(find_max_length(vec![0, 1, 0, 0, 1]), 4);
    }
    #[test]
    fn test_5() {
        assert_eq!(find_max_length(vec![0, 1, 0, 1, 1]), 4);
    }
}
