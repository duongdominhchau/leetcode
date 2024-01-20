// https://leetcode.com/problems/set-mismatch

use std::collections::HashSet;

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut s = HashSet::new();
    let mut duplicated_number = 0;
    for x in nums.iter() {
        if s.contains(x) {
            duplicated_number = *x;
            break;
        }
        s.insert(x);
    }
    // sum_correct - miss + dup = sum_error
    let sum_error: i32 = nums.iter().sum();
    let sum_correct = (1..=nums.len()).sum::<usize>() as i32;
    let missing_number = sum_correct - sum_error + duplicated_number;
    vec![duplicated_number, missing_number]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }
}
