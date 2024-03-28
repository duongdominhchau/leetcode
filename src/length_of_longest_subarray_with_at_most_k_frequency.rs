// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut left = 0;
    let mut right = 0;
    let mut freq = HashMap::new();
    freq.insert(nums[0], 1);
    let mut max_len = 1;
    while right < nums.len() {
        right += 1;
        if right == nums.len() {
            break;
        }
        let mut count = *freq
            .entry(nums[right])
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if count <= k {
            max_len = max_len.max(right - left + 1);
        } else {
            while count > k {
                freq.entry(nums[left])
                    .and_modify(|count| *count -= 1)
                    .or_insert(0);
                if nums[left] == nums[right] {
                    count -= 1;
                }
                left += 1;
            }
        }
    }
    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2), 6);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4), 4);
    }
}
