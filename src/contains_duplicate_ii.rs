// https://leetcode.com/problems/contains-duplicate-ii

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    let k = k as usize;
    let mut s = HashMap::new();
    s.reserve(nums.len());

    for i in 0..nums.len() {
        let prev_index = s.entry(nums[i]).or_insert(i);
        if *prev_index != i && i - *prev_index <= k {
            return true;
        }
        *prev_index = i;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    }
    #[test]
    fn test_3() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}
