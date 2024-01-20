// https://leetcode.com/problems/intersection-of-two-arrays-ii

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut nums = HashMap::new();
    for n in nums2 {
        let entry = nums.entry(n).or_insert(0);
        *entry += 1;
    }

    nums1
        .into_iter()
        .filter(|n| match nums.get_mut(n) {
            Some(count) => {
                if *count == 0 {
                    false
                } else {
                    *count -= 1;
                    true
                }
            }
            None => false,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    }
    #[test]
    fn test_2() {
        assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
    }
}
