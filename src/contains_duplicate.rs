// https://leetcode.com/problems/contains-duplicate

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut s = HashSet::new();
    s.reserve(nums.len());

    for x in nums {
        if !s.insert(x) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
